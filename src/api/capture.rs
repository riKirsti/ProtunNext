//! Capture control runs outside packet I/O; an enabled sink is explicitly passed
//! to the packet worker. No file is created merely by constructing a connection.
use super::types::{CaptureStopReason, Event, FileWriteMode, PcapFile, PcapFileInfo};
use crate::{Error, Result, capture::Capture};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    time::Duration,
};
pub struct CaptureController {
    current: Option<(PcapFileInfo, Capture<File>)>,
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
        let mut events = Vec::new();
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
            PcapFile::Fd(fd) => {
                #[cfg(unix)]
                {
                    use std::os::fd::FromRawFd;
                    // SAFETY: duplicates the raw input fd; only the fresh fd is owned here.
                    let copy = unsafe { libc::fcntl(*fd, libc::F_DUPFD_CLOEXEC, 3) };
                    if copy < 0 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                    // SAFETY: successful fcntl returned a new owned descriptor.
                    let file = unsafe { File::from_raw_fd(copy) };
                    // Legacy Fd transfers ownership; close the caller-supplied fd
                    // only after duplication succeeds. Callers relinquish it.
                    // SAFETY: scalar descriptor, close never accesses Rust memory.
                    unsafe {
                        libc::close(*fd);
                    }
                    (file, true)
                }
                #[cfg(not(unix))]
                {
                    let _ = fd;
                    return Err(Error::Unsupported("capture fd on this platform"));
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
            Capture::resume(file, size, limit)?
        } else {
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;
            Capture::new(file, limit)?
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
        capture.finish()?.sync_all()?;
        Ok(Some(Event::PacketCaptureStopped {
            reason: CaptureStopReason::MaxSizeReached { file },
        }))
    }
    pub fn stop(&mut self, disconnected: bool) -> Result<Event> {
        let reason = match self.current.take() {
            Some((file, capture)) => {
                capture.finish()?.sync_all()?;
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
