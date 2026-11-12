//! Benchmark-only bounded TUN read handoff. Never linked into normal builds.
use std::{
    fs::File,
    io::{self, Read, Write},
    os::fd::{AsRawFd, FromRawFd},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender, TryRecvError},
    },
    thread::{self, JoinHandle},
};

const BUFFERS: usize = 64;
const CAPACITY: usize = 65_000;

pub(crate) struct Frame {
    bytes: Box<[u8]>,
    len: usize,
}
impl Frame {
    pub(crate) fn data(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
    fn clear(&mut self) {
        crate::packet::erase(&mut self.bytes[..self.len]);
        self.len = 0;
    }
}
impl Drop for Frame {
    fn drop(&mut self) {
        crate::packet::erase(&mut self.bytes);
    }
}

pub(crate) struct Reader {
    receive: Option<Receiver<io::Result<Frame>>>,
    free: Option<SyncSender<Frame>>,
    wake: Arc<File>,
    stop: Arc<AtomicBool>,
    count: Arc<AtomicU64>,
    thread: Option<JoinHandle<()>>,
    clock: libc::clockid_t,
    cpu: usize,
}

fn notify(wake: &File) -> io::Result<()> {
    let mut wake = wake;
    match wake.write(&1u64.to_ne_bytes()) {
        Ok(8) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::Interrupted => notify(wake),
        Err(e) => Err(e),
        _ => Err(io::Error::other("short eventfd write")),
    }
}

impl Reader {
    pub(crate) fn new(mut device: File, cpu: usize) -> io::Result<Self> {
        // SAFETY: live descriptor; scalar query does not change its ownership.
        let flags = unsafe { libc::fcntl(device.as_raw_fd(), libc::F_GETFL) };
        if flags < 0 || flags & libc::O_NONBLOCK == 0 {
            return Err(io::Error::other("reader requires nonblocking descriptor"));
        }
        assert!(cpu < libc::CPU_SETSIZE as usize);
        // SAFETY: no aliases; successful eventfd result is transferred to File.
        let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: valid new owned descriptor, wrapped exactly once.
        let wake = Arc::new(unsafe { File::from_raw_fd(fd) });
        let stop = Arc::new(AtomicBool::new(false));
        let count = Arc::new(AtomicU64::new(0));
        let (free_tx, free_rx) = mpsc::sync_channel::<Frame>(BUFFERS);
        let (out_tx, out_rx) = mpsc::sync_channel::<io::Result<Frame>>(BUFFERS);
        let (ready_tx, ready_rx) = mpsc::channel();
        for _ in 0..BUFFERS {
            free_tx
                .send(Frame {
                    bytes: vec![0; CAPACITY].into_boxed_slice(),
                    len: 0,
                })
                .unwrap_or_else(|_| panic!("reader buffer initialization"));
        }
        let thread_wake = wake.clone();
        let thread_stop = stop.clone();
        let thread_count = count.clone();
        let thread = thread::spawn(move || {
            // SAFETY: complete initialized cpu_set_t and checked CPU index.
            let mut set = unsafe { std::mem::zeroed::<libc::cpu_set_t>() };
            // SAFETY: correctly sized CPU mask, applying only to this thread.
            unsafe {
                libc::CPU_ZERO(&mut set);
                libc::CPU_SET(cpu, &mut set);
                assert_eq!(
                    libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
                    0
                );
            }
            let mut clock = 0;
            // SAFETY: live current pthread and complete clock-id output.
            assert_eq!(
                unsafe { libc::pthread_getcpuclockid(libc::pthread_self(), &mut clock) },
                0
            );
            if ready_tx.send(clock).is_err() {
                return;
            }
            let mut run = || -> io::Result<()> {
                let mut pending = 0;
                while !thread_stop.load(Ordering::Acquire) {
                    let mut frame = match free_rx.try_recv() {
                        Ok(frame) => frame,
                        Err(TryRecvError::Disconnected) => break,
                        Err(TryRecvError::Empty) => {
                            if pending > 0 {
                                notify(&thread_wake)?;
                                pending = 0;
                            }
                            match free_rx.recv() {
                                Ok(frame) => frame,
                                Err(_) => break,
                            }
                        }
                    };
                    loop {
                        if thread_stop.load(Ordering::Acquire) {
                            return Ok(());
                        }
                        match device.read(&mut frame.bytes) {
                            Ok(0) => return Err(io::Error::other("unexpected TUN reader EOF")),
                            Ok(n) => {
                                frame.len = n;
                                break;
                            }
                            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                if pending > 0 {
                                    notify(&thread_wake)?;
                                    pending = 0;
                                }
                                let mut poll = libc::pollfd {
                                    fd: device.as_raw_fd(),
                                    events: libc::POLLIN,
                                    revents: 0,
                                };
                                // SAFETY: one initialized pollfd for a live descriptor.
                                let n = unsafe { libc::poll(&mut poll, 1, 10) };
                                if n < 0
                                    && io::Error::last_os_error().kind()
                                        != io::ErrorKind::Interrupted
                                {
                                    return Err(io::Error::last_os_error());
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    thread_count.fetch_add(1, Ordering::Relaxed);
                    if out_tx.send(Ok(frame)).is_err() {
                        break;
                    }
                    pending += 1;
                    if pending == 32 {
                        notify(&thread_wake)?;
                        pending = 0;
                    }
                }
                if pending > 0 {
                    notify(&thread_wake)?;
                }
                Ok(())
            };
            if let Err(error) = run() {
                let _ = out_tx.send(Err(error));
                let _ = notify(&thread_wake);
            }
        });
        let clock = ready_rx.recv().map_err(io::Error::other)?;
        Ok(Self {
            receive: Some(out_rx),
            free: Some(free_tx),
            wake,
            stop,
            count,
            thread: Some(thread),
            clock,
            cpu,
        })
    }
    pub(crate) fn clock(&self) -> libc::clockid_t {
        self.clock
    }
    pub(crate) fn cpu(&self) -> usize {
        self.cpu
    }
    pub(crate) fn fd(&self) -> i32 {
        self.wake.as_raw_fd()
    }
    pub(crate) fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
    pub(crate) fn drain_wake(&self) -> io::Result<()> {
        let mut wake = self.wake.as_ref();
        let mut bytes = [0; 8];
        loop {
            match wake.read(&mut bytes) {
                Ok(8) => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
                _ => return Err(io::Error::other("short eventfd read")),
            }
        }
    }
    pub(crate) fn next(&self) -> io::Result<Option<Frame>> {
        match self.receive.as_ref().unwrap().try_recv() {
            Ok(frame) => frame.map(Some),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(io::Error::other("TUN reader stopped")),
        }
    }
    pub(crate) fn recycle(&self, mut frame: Frame) -> io::Result<()> {
        frame.clear();
        self.free
            .as_ref()
            .unwrap()
            .send(frame)
            .map_err(|_| io::Error::other("TUN reader recycling stopped"))
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        drop(self.receive.take());
        drop(self.free.take());
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::IntoRawFd;
    use std::os::unix::net::UnixDatagram;
    use std::time::{Duration, Instant};
    fn cpu() -> usize {
        // SAFETY: complete initialized output mask for the current thread.
        let mut set = unsafe { std::mem::zeroed::<libc::cpu_set_t>() };
        // SAFETY: correctly sized writable output mask.
        assert_eq!(
            unsafe { libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set) },
            0
        );
        (0..libc::CPU_SETSIZE as usize)
            .find(|&cpu| unsafe { libc::CPU_ISSET(cpu, &set) })
            .unwrap()
    }
    #[test]
    fn ordered_handoff_wakes_after_idle_and_recycles_cleared_storage() {
        let (read, write) = UnixDatagram::pair().unwrap();
        read.set_nonblocking(true).unwrap();
        // SAFETY: transfer the owned socket descriptor into a File exactly once.
        let read = unsafe { File::from_raw_fd(read.into_raw_fd()) };
        let reader = Reader::new(read, cpu()).unwrap();
        for i in 0u8..70 {
            write.send(&[i; 128]).unwrap();
            let until = Instant::now() + Duration::from_secs(2);
            let frame = loop {
                reader.drain_wake().unwrap();
                if let Some(frame) = reader.next().unwrap() {
                    break frame;
                }
                assert!(Instant::now() < until);
                thread::sleep(Duration::from_millis(1));
            };
            assert_eq!(frame.data(), &[i; 128]);
            reader.recycle(frame).unwrap();
        }
        assert_eq!(reader.count(), 70);
        let mut frame = Frame {
            bytes: vec![0x5a; 128].into_boxed_slice(),
            len: 128,
        };
        frame.clear();
        assert!(frame.bytes.iter().all(|b| *b == 0));
    }
    #[test]
    fn stop_unblocks_when_all_buffers_are_queued() {
        let (read, write) = UnixDatagram::pair().unwrap();
        read.set_nonblocking(true).unwrap();
        write
            .set_write_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        // SAFETY: transfer the owned socket descriptor into a File exactly once.
        let read = unsafe { File::from_raw_fd(read.into_raw_fd()) };
        let reader = Reader::new(read, cpu()).unwrap();
        for _ in 0..BUFFERS {
            write.send(&[1; 64]).unwrap();
        }
        let until = Instant::now() + Duration::from_secs(2);
        while reader.count() < BUFFERS as u64 {
            assert!(Instant::now() < until);
            thread::sleep(Duration::from_millis(1));
        }
        drop(reader);
    }
}
