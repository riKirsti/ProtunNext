//! Capture control runs outside packet I/O; an enabled sink is explicitly passed
//! to the packet worker. No file is created merely by constructing a connection.
use super::types::{CaptureStopReason, Event, FileWriteMode, PcapFile, PcapFileInfo};
use crate::{Error, Result, capture::Capture};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};

pub(crate) struct CaptureInput {
    info: PcapFileInfo,
    #[cfg(unix)]
    descriptor: Option<File>,
}

impl CaptureInput {
    pub(crate) fn take(info: PcapFileInfo) -> Result<Self> {
        #[cfg(unix)]
        let descriptor = if let PcapFile::Fd(fd) = info.file {
            use std::os::fd::FromRawFd;
            // SAFETY: the caller transfers fd. Duplicate it before closing the original.
            let copy = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
            let error = (copy < 0).then(io::Error::last_os_error);
            // SAFETY: ownership transfers on failure too.
            unsafe { libc::close(fd) };
            if let Some(error) = error {
                return Err(error.into());
            }
            // SAFETY: successful fcntl returned a new owned descriptor.
            Some(unsafe { File::from_raw_fd(copy) })
        } else {
            None
        };
        #[cfg(not(unix))]
        if matches!(info.file, PcapFile::Fd(_)) {
            return Err(Error::Unsupported("capture fd on this platform"));
        }
        Ok(Self {
            info,
            #[cfg(unix)]
            descriptor,
        })
    }
}

// Only the connection-owned writer needs cancellation. Public synchronous
// CaptureController users keep their existing descriptor behavior.
struct Output {
    file: File,
    #[cfg(unix)]
    cancel: Option<Arc<AtomicBool>>,
}

impl Output {
    fn new(file: File, cancel: Option<Arc<AtomicBool>>) -> io::Result<Self> {
        #[cfg(unix)]
        {
            let cancel = if cancel.is_some() && !file.metadata()?.is_file() {
                cancel
            } else {
                None
            };
            Ok(Self { file, cancel })
        }
        #[cfg(not(unix))]
        {
            let _ = cancel;
            Ok(Self { file })
        }
    }

    #[cfg(unix)]
    fn write_stream(file: &mut File, bytes: &[u8], cancel: &AtomicBool) -> io::Result<usize> {
        use std::os::fd::AsRawFd;

        let fd = file.as_raw_fd();
        // SAFETY: this output owns a live file descriptor.
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }
        let restore = flags & libc::O_NONBLOCK == 0;
        // SAFETY: F_SETFL takes scalar descriptor status flags.
        if restore && unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
            return Err(io::Error::last_os_error());
        }
        let result = loop {
            if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                break Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "capture write cancelled",
                ));
            }
            match file.write(bytes) {
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5));
                }
                result => break result,
            }
        };
        if restore {
            // SAFETY: the file is still live. Preserve any other status flags.
            let current = unsafe { libc::fcntl(fd, libc::F_GETFL) };
            // SAFETY: F_SETFL takes scalar descriptor status flags.
            if current < 0
                || unsafe { libc::fcntl(fd, libc::F_SETFL, current & !libc::O_NONBLOCK) } < 0
            {
                log::warn!("could not restore capture descriptor flags");
            }
        }
        result
    }
}

impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        #[cfg(unix)]
        if let Some(cancel) = &self.cancel {
            return Self::write_stream(&mut self.file, bytes, cancel);
        }
        self.file.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

pub struct CaptureController {
    current: Option<(PcapFileInfo, Capture<Output>)>,
}
impl Default for CaptureController {
    fn default() -> Self {
        Self::new()
    }
}
impl CaptureController {
    pub fn new() -> Self {
        Self { current: None }
    }
    pub fn start(&mut self, info: PcapFileInfo) -> Result<Vec<Event>> {
        self.start_input(CaptureInput::take(info)?, None)
    }
    pub(crate) fn start_input(
        &mut self,
        input: CaptureInput,
        cancel: Option<Arc<AtomicBool>>,
    ) -> Result<Vec<Event>> {
        let info = input.info;
        let mut events = Vec::new();
        let descriptor = matches!(info.file, PcapFile::Fd(_));
        // Open/validate the replacement first so an invalid request does not
        // destroy an existing working capture.
        let (mut file, append) = match &info.file {
            PcapFile::Path { path, mode } => {
                let path = std::path::Path::new(path);
                if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                    std::fs::create_dir_all(parent)?;
                }
                let mut options = File::options();
                options.create(true).read(true).write(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options
                        .mode(0o600)
                        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
                }
                let file = options.open(path)?;
                if !file.metadata()?.is_file() {
                    return Err(Error::Invalid("capture requires regular file"));
                }
                (file, *mode == FileWriteMode::Append)
            }
            PcapFile::Fd(_) => {
                #[cfg(unix)]
                {
                    (input.descriptor.expect("adopted capture descriptor"), false)
                }
                #[cfg(not(unix))]
                {
                    unreachable!("unsupported capture descriptor was rejected")
                }
            }
        };
        let size = if append { file.metadata()?.len() } else { 0 };
        let limit = info.max_bytes.unwrap_or(u64::MAX);
        if limit < 24 || size > limit {
            return Err(Error::Invalid("capture size limit"));
        }
        let capture = if size > 0 {
            if size < 24 {
                return Err(Error::Invalid("partial PCAP header"));
            }
            let mut header = [0; 24];
            file.seek(SeekFrom::Start(0))?;
            file.read_exact(&mut header)?;
            if header[..4] != 0xa1b2c3d4u32.to_le_bytes() || header[20..24] != 101u32.to_le_bytes()
            {
                return Err(Error::Invalid("incompatible PCAP append format"));
            }
            file.seek(SeekFrom::End(0))?;
            Capture::resume(Output::new(file, cancel)?, size, limit)?
        } else {
            if !descriptor {
                file.set_len(0)?;
                file.seek(SeekFrom::Start(0))?;
            }
            Capture::new(Output::new(file, cancel)?, limit)?
        };
        if self.current.is_some() {
            events.push(self.stop(false)?);
        }
        self.current = Some((info.clone(), capture));
        events.push(Event::PacketCaptureStarted { info });
        Ok(events)
    }
    pub fn packet(&mut self, wall: Duration, bytes: &[u8]) -> Result<Option<Event>> {
        let Some((_, capture)) = &mut self.current else {
            return Ok(None);
        };
        if capture.packet(wall, bytes)? {
            return Ok(None);
        }
        let (file, capture) = self.current.take().expect("capture exists");
        finish(capture)?;
        Ok(Some(Event::PacketCaptureStopped {
            reason: CaptureStopReason::MaxSizeReached { file },
        }))
    }
    pub fn stop(&mut self, disconnected: bool) -> Result<Event> {
        let reason = match self.current.take() {
            Some((file, capture)) => {
                finish(capture)?;
                if disconnected {
                    CaptureStopReason::Disconnected { file }
                } else {
                    CaptureStopReason::Request { file }
                }
            }
            None => CaptureStopReason::AlreadyStopped,
        };
        Ok(Event::PacketCaptureStopped { reason })
    }
    pub fn active(&self) -> bool {
        self.current.is_some()
    }
}

fn finish(capture: Capture<Output>) -> std::io::Result<()> {
    let output = capture.finish()?;
    // Pipes and sockets support flushing their writer, but cannot be fsynced.
    // Preserve durable completion for regular capture files.
    if output.file.metadata()?.is_file() {
        output.file.sync_all()?;
    }
    Ok(())
}
