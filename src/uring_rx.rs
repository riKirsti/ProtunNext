//! Optional Linux multishot receive. Sends remain sendmmsg-based; Mio observes
//! CQ notifications through eventfd alongside TUN, control and socket readiness.
//! No SQPOLL, busy polling, zero-copy-send loans or raw packet bypass.
use crate::{
    Error, Result,
    packet::{Packet, PacketPool},
    performance::IoCounters,
    stream::Receive,
};
use io_uring::{IoUring, cqueue, opcode, types};
use std::{
    fs::File,
    io,
    net::UdpSocket,
    os::fd::{AsRawFd, FromRawFd},
    ptr::NonNull,
    sync::atomic::{AtomicU16, Ordering},
};
const GROUP: u16 = 7;
struct RingMemory {
    pointer: NonNull<types::BufRingEntry>,
    bytes: usize,
}
impl RingMemory {
    fn new(entries: usize) -> io::Result<Self> {
        // SAFETY: sysconf returns a scalar page size and does not access memory.
        let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        if page <= 0 {
            return Err(io::Error::last_os_error());
        }
        let bytes =
            (entries * size_of::<types::BufRingEntry>()).div_ceil(page as usize) * page as usize;
        // SAFETY: a new private zero-filled, page-aligned mapping; no fixed address.
        let pointer = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                bytes,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        if pointer == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            pointer: NonNull::new(pointer.cast()).expect("mmap address"),
            bytes,
        })
    }
}
impl Drop for RingMemory {
    fn drop(&mut self) {
        // SAFETY: exclusively owned mapping, released only after kernel buffer access stops.
        unsafe {
            libc::munmap(self.pointer.as_ptr().cast(), self.bytes);
        }
    }
}
pub(crate) struct Receiver {
    ring: Option<IoUring>,
    memory: Option<RingMemory>,
    buffers: Vec<Packet>,
    event: File,
    tail: u16,
    active: bool,
    pub(crate) disabled: bool,
    pub(crate) submissions: u64,
}
impl Receiver {
    pub(crate) fn new(socket: &UdpSocket, count: u16) -> io::Result<Self> {
        let ring = IoUring::builder().setup_cqsize(128).build(32)?;
        // Require safe bounded shutdown support *before* submitting buffer loans.
        match ring.submitter().register_sync_cancel(
            Some(types::Timespec::new().sec(1)),
            types::CancelBuilder::any(),
        ) {
            Ok(()) => {}
            Err(e) if e.raw_os_error() == Some(libc::ENOENT) => {}
            Err(e) => return Err(e),
        }
        let mut probe = io_uring::Probe::new();
        ring.submitter().register_probe(&mut probe)?;
        if !probe.is_supported(opcode::RecvMulti::CODE) {
            return Err(io::ErrorKind::Unsupported.into());
        }
        let count = count.max(8).next_power_of_two() as usize;
        let memory = RingMemory::new(count)?;
        // SAFETY: allocates a new nonblocking notification descriptor, no borrowed memory.
        let event = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        if event < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: successful eventfd returned a new uniquely owned descriptor.
        let event = unsafe { File::from_raw_fd(event) };
        let mut pool = PacketPool::new(count, 65535).map_err(io::Error::other)?;
        let buffers = (0..count)
            .map(|_| pool.take().expect("bounded count"))
            .collect();
        // SAFETY: page-aligned mapping and all buffers are owned until synchronous
        // cancellation/unregistration in Receiver::drop; ring length is a power of two.
        unsafe {
            ring.submitter().register_buf_ring_with_flags(
                memory.pointer.as_ptr() as u64,
                count as u16,
                GROUP,
                0,
            )?;
        }
        // From here every fallible setup step drops Receiver, which unregisters
        // the buffer group before allowing memory and payload buffers to drop.
        let mut receiver = Self {
            ring: Some(ring),
            memory: Some(memory),
            buffers,
            event,
            tail: 0,
            active: false,
            disabled: false,
            submissions: 0,
        };
        receiver
            .ring
            .as_ref()
            .expect("owned ring")
            .submitter()
            .register_eventfd(receiver.event.as_raw_fd())?;
        for id in 0..count {
            receiver.provide(id);
        }
        receiver.arm(socket)?;
        Ok(receiver)
    }
    pub(crate) fn event_fd(&self) -> i32 {
        self.event.as_raw_fd()
    }
    fn provide(&mut self, id: usize) {
        let memory = self.memory.as_ref().expect("registered mapping");
        let index = self.tail as usize & (self.buffers.len() - 1);
        let bytes = self.buffers[id].writable();
        // SAFETY: only the entry returned by the kernel is rewritten; the ring's
        // published tail is advanced with release ordering *after* these fields.
        unsafe {
            let entry = &mut *memory.pointer.as_ptr().add(index);
            entry.set_addr(bytes.as_mut_ptr() as u64);
            entry.set_len(bytes.len() as u32);
            entry.set_bid(id as u16);
        }
        self.tail = self.tail.wrapping_add(1);
        // SAFETY: tail is the ABI's aligned u16 shared producer field. This sole
        // worker publishes it atomically, after initializing the offered entry.
        unsafe {
            let tail = types::BufRingEntry::tail(memory.pointer.as_ptr()).cast::<AtomicU16>();
            (&*tail).store(self.tail, Ordering::Release);
        }
    }
    fn arm(&mut self, socket: &UdpSocket) -> io::Result<()> {
        if self.active || self.disabled {
            return Ok(());
        }
        let entry = opcode::RecvMulti::new(types::Fd(socket.as_raw_fd()), GROUP)
            .flags(libc::MSG_TRUNC)
            .build()
            .user_data(1);
        let ring = self.ring.as_mut().expect("owned ring");
        // SAFETY: socket stays alive in Datagram, and all published buffers are
        // retained until final completion or synchronous cancellation in Drop.
        unsafe {
            ring.submission()
                .push(&entry)
                .map_err(|_| io::Error::other("receive SQ full"))?;
        }
        self.active = true;
        self.submissions += 1;
        ring.submit()?;
        Ok(())
    }
    fn drain_event(&self) {
        let mut value = 0u64;
        // SAFETY: nonblocking eventfd writes exactly one u64 into this live scalar.
        unsafe {
            libc::read(self.event.as_raw_fd(), (&mut value as *mut u64).cast(), 8);
        }
    }
    pub(crate) fn receive(
        &mut self,
        socket: &UdpSocket,
        output: &mut Packet,
        counters: &mut IoCounters,
    ) -> Result<Receive> {
        if output.writable().len() != 65535 {
            return Err(Error::Invalid("io_uring requires full receive slots"));
        }
        self.arm(socket)?;
        let mut completion = self.ring.as_mut().expect("owned ring").completion().next();
        if completion.is_none() {
            self.drain_event();
            completion = self.ring.as_mut().expect("owned ring").completion().next();
        }
        let Some(completion) = completion else {
            return Ok(Receive::WouldBlock);
        };
        self.active = cqueue::more(completion.flags());
        counters.io_uring_completions += 1;
        let result = completion.result();
        let id = cqueue::buffer_select(completion.flags()).map(usize::from);
        if let Some(id) = id {
            if id >= self.buffers.len() {
                return Err(Error::Invalid("io_uring buffer id"));
            }
            if result >= 0 && result as usize <= 65535 {
                self.buffers[id].set_len(result as usize)?;
                std::mem::swap(output, &mut self.buffers[id]);
                self.buffers[id].set_len(0)?;
                self.provide(id);
                counters.received_datagrams += 1;
                counters.max_receive_batch = counters.max_receive_batch.max(1);
                return Ok(Receive::Packet);
            }
            self.buffers[id].set_len(0)?;
            self.provide(id);
        }
        if result >= 0 {
            counters.truncated_datagrams += 1;
            return Ok(Receive::Yield);
        }
        match -result {
            libc::ENOBUFS => {
                counters.enobufs += 1;
                Ok(Receive::Yield)
            }
            libc::EINTR | libc::EAGAIN => Ok(Receive::Yield),
            libc::EINVAL | libc::EOPNOTSUPP | libc::ENOSYS => {
                self.disabled = true;
                counters.kernel_fallbacks += 1;
                Err(Error::Unsupported("kernel multishot receive"))
            }
            code => Err(io::Error::from_raw_os_error(code).into()),
        }
    }
}
impl Drop for Receiver {
    fn drop(&mut self) {
        if let Some(ring) = self.ring.as_ref() {
            let stopped = match ring.submitter().register_sync_cancel(
                Some(types::Timespec::new().sec(1)),
                types::CancelBuilder::any(),
            ) {
                Ok(()) => true,
                Err(e) if e.raw_os_error() == Some(libc::ENOENT) => true,
                Err(_) => false,
            };
            if !stopped {
                // A kernel refusing cancellation must never be allowed to access freed
                // userspace buffers. Quarantine these bounded allocations on this rare
                // shutdown failure; closing the ring releases its kernel resources.
                std::mem::forget(std::mem::take(&mut self.buffers));
                if let Some(memory) = self.memory.take() {
                    std::mem::forget(memory);
                }
            } else {
                let _ = ring.submitter().unregister_buf_ring(GROUP);
            }
        }
        // Destroy the ring before the registered mapping and buffers are released.
        self.ring.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_multishot_receive_recycles_buffers_and_cancels_an_idle_request() {
        let a = UdpSocket::bind("127.0.0.1:0").unwrap();
        let b = UdpSocket::bind("127.0.0.1:0").unwrap();
        a.connect(b.local_addr().unwrap()).unwrap();
        b.connect(a.local_addr().unwrap()).unwrap();
        b.set_nonblocking(true).unwrap();
        let mut receiver = match Receiver::new(&b, 8) {
            Ok(receiver) => receiver,
            Err(e) => {
                assert_ne!(
                    std::env::var("PROTUN_REQUIRE_URING").as_deref(),
                    Ok("1"),
                    "required io_uring unavailable: {e}"
                );
                eprintln!("io_uring unavailable; portable fallback is required: {e}");
                return;
            }
        };
        let mut pool = PacketPool::new(2, 65535).unwrap();
        let mut output = pool.take().unwrap();
        let mut counters = IoCounters::default();
        for turn in 0..16 {
            // More than the eight registered buffers, checking wrap/reuse.
            for n in 0..4 {
                a.send(&[turn, n, 9, 7]).unwrap();
            }
            for n in 0..4 {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
                loop {
                    assert!(std::time::Instant::now() < deadline, "completion deadline");
                    match receiver.receive(&b, &mut output, &mut counters) {
                        Ok(Receive::Packet) => break,
                        Err(Error::Unsupported(_)) => {
                            assert_ne!(std::env::var("PROTUN_REQUIRE_URING").as_deref(), Ok("1"));
                            return;
                        }
                        Ok(_) => std::thread::sleep(std::time::Duration::from_millis(1)),
                        Err(e) => panic!("receive: {e}"),
                    }
                }
                assert_eq!(output.data(), [turn, n, 9, 7]);
            }
        }
        assert_eq!(counters.received_datagrams, 64);
        assert!(
            receiver.submissions < 64,
            "must use genuine multishot requests"
        );
        eprintln!(
            "actual multishot: {} datagrams, {} submissions",
            counters.received_datagrams, receiver.submissions
        );
        let start = std::time::Instant::now();
        drop(receiver);
        assert!(
            start.elapsed() < std::time::Duration::from_secs(2),
            "idle cancellation must be bounded"
        );
    }
}
