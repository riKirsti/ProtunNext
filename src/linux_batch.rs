//! Connected UDP batching. Linux sendmmsg/recvmmsg contracts are documented in
//! docs/performance.md. All payload buffers stay owned until the syscall returns.
use crate::{
    Error, Result,
    packet::{Packet, PacketPool, PacketQueue},
    performance::IoCounters,
    stream::Receive,
};
use std::{collections::VecDeque, io, net::UdpSocket, os::fd::AsRawFd};

pub(crate) const MAX_BATCH: usize = 32;
const BATCH_SIZE: usize = 16;
#[repr(C, align(8))]
struct Control([u8; 64]);
struct Received {
    packet: Packet,
    segment: usize,
    offset: usize,
}
pub(crate) struct Batch {
    ready: VecDeque<Received>,
    slots: Vec<Packet>,
    pub(crate) counters: IoCounters,
    retry_at: Option<std::time::Instant>,
    gso: bool,
}
impl Batch {
    pub(crate) fn new(socket: &UdpSocket, offload: bool) -> Self {
        let mut counters = IoCounters::default();
        if offload {
            let enabled: libc::c_int = 1;
            // SAFETY: a live UDP socket and a correctly sized read-only scalar.
            if unsafe {
                libc::setsockopt(
                    socket.as_raw_fd(),
                    libc::SOL_UDP,
                    libc::UDP_GRO,
                    (&enabled as *const libc::c_int).cast(),
                    size_of::<libc::c_int>() as _,
                )
            } < 0
            {
                counters.kernel_fallbacks += 1;
            }
        }
        Self {
            ready: VecDeque::with_capacity(MAX_BATCH),
            slots: Vec::with_capacity(MAX_BATCH),
            counters,
            retry_at: None,
            gso: offload,
        }
    }
    fn pop_ready(&mut self, output: &mut Packet, pool: &mut PacketPool) -> Result<bool> {
        let Some(mut received) = self.ready.pop_front() else {
            return Ok(false);
        };
        if received.segment == 0 {
            std::mem::swap(output, &mut received.packet);
            pool.recycle(received.packet)?;
        } else {
            let end = (received.offset + received.segment).min(received.packet.data().len());
            let n = end - received.offset;
            if n > output.writable().len() {
                return Err(Error::Full);
            }
            output.writable()[..n].copy_from_slice(&received.packet.data()[received.offset..end]);
            output.set_len(n)?;
            received.offset = end;
            if end == received.packet.data().len() {
                pool.recycle(received.packet)?;
            } else {
                self.ready.push_front(received);
            }
        }
        self.counters.received_datagrams += 1;
        Ok(true)
    }
    pub(crate) fn receive(
        &mut self,
        socket: &UdpSocket,
        output: &mut Packet,
        pool: &mut PacketPool,
    ) -> Result<Receive> {
        if self.pop_ready(output, pool)? {
            return Ok(Receive::Packet);
        }
        let count = BATCH_SIZE
            .min(pool.available().saturating_sub(1).max(1))
            .min(pool.available());
        if count == 0 {
            return Ok(Receive::Yield);
        }
        for _ in 0..count {
            self.slots.push(pool.take_sized(65535).ok_or(Error::Full)?);
        }
        // SAFETY: all-zero iovec/mmsghdr values have null pointers and zero sizes;
        // every active entry is initialized below before passing it to the kernel.
        let mut vectors: [libc::iovec; MAX_BATCH] = unsafe { std::mem::zeroed() };
        // SAFETY: same initialization argument as above, including all pointer fields.
        let mut messages: [libc::mmsghdr; MAX_BATCH] = unsafe { std::mem::zeroed() };
        let mut controls: [Control; MAX_BATCH] = std::array::from_fn(|_| Control([0; 64]));
        for (i, packet) in self.slots.iter_mut().enumerate() {
            let bytes = packet.writable();
            vectors[i].iov_base = bytes.as_mut_ptr().cast();
            vectors[i].iov_len = bytes.len();
            messages[i].msg_hdr.msg_iov = &mut vectors[i];
            messages[i].msg_hdr.msg_iovlen = 1;
            messages[i].msg_hdr.msg_control = controls[i].0.as_mut_ptr().cast();
            messages[i].msg_hdr.msg_controllen = 64;
        }
        self.counters.receive_calls += 1;
        // SAFETY: connected live socket, bounded initialized descriptor array, and
        // disjoint owned output allocations remain valid throughout this sync call.
        let received = unsafe {
            libc::recvmmsg(
                socket.as_raw_fd(),
                messages.as_mut_ptr(),
                count as u32,
                libc::MSG_DONTWAIT as _,
                std::ptr::null_mut(),
            )
        };
        let error = (received < 0).then(io::Error::last_os_error);
        if let Some(error) = &error {
            self.counters.socket_error(error, false);
        }
        let received = received.max(0) as usize;
        self.counters.max_receive_batch = self.counters.max_receive_batch.max(received as u64);
        for (i, mut packet) in self.slots.drain(..).enumerate() {
            if i < received {
                let n = messages[i].msg_len as usize;
                self.counters.received_bytes += n.min(65535) as u64;
                if messages[i].msg_hdr.msg_flags & (libc::MSG_TRUNC | libc::MSG_CTRUNC) != 0 {
                    packet.set_len(n.min(65535))?;
                    self.counters.truncated_datagrams += 1;
                    pool.recycle(packet)?;
                } else {
                    packet.set_len(n)?;
                    let segment = gro_size(&messages[i].msg_hdr)?;
                    if segment > n || (segment > 0 && n == 0) {
                        self.counters.truncated_datagrams += 1;
                        pool.recycle(packet)?;
                        continue;
                    }
                    if segment > 0 {
                        self.counters.gro_datagrams += n.div_ceil(segment) as u64;
                    }
                    self.ready.push_back(Received {
                        packet,
                        segment,
                        offset: 0,
                    });
                }
            } else {
                pool.recycle(packet)?;
            }
        }
        if let Some(error) = error {
            return match error.raw_os_error() {
                _ if error.kind() == io::ErrorKind::WouldBlock => Ok(Receive::WouldBlock),
                _ if error.kind() == io::ErrorKind::Interrupted => Ok(Receive::Yield),
                _ => Err(error.into()),
            };
        }
        if self.pop_ready(output, pool)? {
            Ok(Receive::Packet)
        } else {
            Ok(Receive::Yield)
        }
    }
    pub(crate) fn retry_delay(&self) -> Option<std::time::Duration> {
        self.retry_at
            .map(|when| when.saturating_duration_since(std::time::Instant::now()))
            .filter(|delay| !delay.is_zero())
    }
    pub(crate) fn flush(
        &mut self,
        socket: &UdpSocket,
        queue: &mut PacketQueue,
        pool: &mut PacketPool,
    ) -> Result<bool> {
        if self.retry_delay().is_some() {
            return Ok(!queue.is_empty());
        }
        self.retry_at = None;
        for _ in 0..64 {
            if queue.is_empty() {
                return Ok(false);
            }
            if self.gso
                && queue.len() > 1
                && let Some(pending) = self.flush_gso(socket, queue, pool)?
            {
                if pending {
                    return Ok(true);
                }
                continue;
            }
            // SAFETY: zeroed syscall descriptors are populated with live borrowed
            // packet slices below; sendmmsg only reads those packet allocations.
            let mut vectors: [libc::iovec; MAX_BATCH] = unsafe { std::mem::zeroed() };
            // SAFETY: zero is a valid empty mmsghdr, including null pointer fields.
            let mut messages: [libc::mmsghdr; MAX_BATCH] = unsafe { std::mem::zeroed() };
            let mut count = queue.len().min(BATCH_SIZE);
            if self.gso {
                // Do not let an isolated control/empty packet make sendmmsg
                // swallow a following GSO-eligible group. Preserve wire order.
                let mut lengths = queue
                    .iter()
                    .take(count)
                    .map(|p| p.data().len())
                    .enumerate()
                    .peekable();
                while let Some((index, size)) = lengths.next() {
                    if index > 0
                        && size > 0
                        && let Some((_, next)) = lengths.peek()
                        && *next > 0
                        && *next <= size
                        && size + next <= 65_487
                    {
                        count = index;
                        break;
                    }
                }
            }
            for (i, packet) in queue.iter().take(count).enumerate() {
                vectors[i].iov_base = packet.data().as_ptr().cast_mut().cast();
                vectors[i].iov_len = packet.data().len();
                messages[i].msg_hdr.msg_iov = &mut vectors[i];
                messages[i].msg_hdr.msg_iovlen = 1;
            }
            self.counters.send_calls += 1;
            // SAFETY: initialized read-only descriptors and all packet buffers
            // remain alive until this synchronous call returns.
            let sent = unsafe {
                libc::sendmmsg(
                    socket.as_raw_fd(),
                    messages.as_mut_ptr(),
                    count as u32,
                    (libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL) as _,
                )
            };
            if sent < 0 {
                let error = io::Error::last_os_error();
                self.counters.socket_error(&error, true);
                match error.raw_os_error() {
                    Some(libc::ENOBUFS) => {
                        self.counters.enobufs += 1;
                        self.retry_at =
                            Some(std::time::Instant::now() + std::time::Duration::from_millis(2));
                        return Ok(true);
                    }
                    _ if error.kind() == io::ErrorKind::WouldBlock => return Ok(true),
                    _ if error.kind() == io::ErrorKind::Interrupted => continue,
                    _ => return Err(error.into()),
                }
            }
            let sent = sent as usize;
            self.counters.max_send_batch = self.counters.max_send_batch.max(sent as u64);
            self.acknowledge(queue, pool, &messages[..sent])?;
            if sent == 0 {
                break;
            }
        }
        Ok(!queue.is_empty())
    }
    fn acknowledge(
        &mut self,
        queue: &mut PacketQueue,
        pool: &mut PacketPool,
        messages: &[libc::mmsghdr],
    ) -> Result<()> {
        for message in messages {
            let packet = queue.pop().ok_or(Error::Invalid("batch send count"))?;
            let valid = message.msg_len as usize == packet.data().len();
            pool.recycle(packet)?;
            if !valid {
                return Err(Error::Invalid("partial UDP datagram"));
            }
            self.counters.sent_datagrams += 1;
            self.counters.sent_bytes += message.msg_len as u64;
        }
        Ok(())
    }
    fn flush_gso(
        &mut self,
        socket: &UdpSocket,
        queue: &mut PacketQueue,
        pool: &mut PacketPool,
    ) -> Result<Option<bool>> {
        let segment = queue.front().expect("nonempty").data().len();
        if segment == 0 || segment > u16::MAX as usize {
            return Ok(None);
        }
        let mut total = 0;
        let mut count = 0;
        // SAFETY: descriptors start empty and are populated with read-only packet slices.
        let mut vectors: [libc::iovec; MAX_BATCH] = unsafe { std::mem::zeroed() };
        for packet in queue.iter().take(BATCH_SIZE) {
            let n = packet.data().len();
            if n == 0 || n > segment || total + n > 65_487 {
                break;
            }
            vectors[count].iov_base = packet.data().as_ptr().cast_mut().cast();
            vectors[count].iov_len = n;
            total += n;
            count += 1;
            if n < segment {
                break;
            } // Only the final segment can be short.
        }
        if count < 2 {
            return Ok(None);
        }
        let mut control = Control([0; 64]);
        // SAFETY: zeroed header is valid; all active pointer fields are set below.
        let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
        msg.msg_iov = vectors.as_mut_ptr();
        msg.msg_iovlen = count as _;
        msg.msg_control = control.0.as_mut_ptr().cast();
        // SAFETY: CMSG helpers compute fixed ABI sizes for a two-byte scalar.
        msg.msg_controllen = unsafe { libc::CMSG_SPACE(2) } as _;
        // SAFETY: aligned storage holds a complete cmsghdr plus two data bytes.
        unsafe {
            let header = libc::CMSG_FIRSTHDR(&msg);
            (*header).cmsg_level = libc::SOL_UDP;
            (*header).cmsg_type = libc::UDP_SEGMENT;
            (*header).cmsg_len = libc::CMSG_LEN(2) as _;
            std::ptr::copy_nonoverlapping(
                (segment as u16).to_ne_bytes().as_ptr(),
                libc::CMSG_DATA(header),
                2,
            );
        }
        self.counters.send_calls += 1;
        // SAFETY: connected socket, bounded initialized vectors/control, all payloads
        // remain owned by queue; this copying send has no asynchronous buffer loans.
        let n = unsafe {
            libc::sendmsg(
                socket.as_raw_fd(),
                &msg,
                libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL,
            )
        };
        if n < 0 {
            let error = io::Error::last_os_error();
            self.counters.socket_error(&error, true);
            match error.raw_os_error() {
                Some(
                    libc::EINVAL
                    | libc::EIO
                    | libc::EMSGSIZE
                    | libc::ENOPROTOOPT
                    | libc::EOPNOTSUPP,
                ) => {
                    self.gso = false;
                    self.counters.kernel_fallbacks += 1;
                    return Ok(None);
                }
                Some(libc::ENOBUFS) => {
                    self.counters.enobufs += 1;
                    self.retry_at =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(2));
                    return Ok(Some(true));
                }
                _ if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                ) =>
                {
                    return Ok(Some(true));
                }
                _ => return Err(error.into()),
            }
        }
        if n as usize != total {
            return Err(Error::Invalid("partial GSO send"));
        }
        for _ in 0..count {
            pool.recycle(queue.pop().expect("validated group"))?;
        }
        self.counters.sent_datagrams += count as u64;
        self.counters.sent_bytes += total as u64;
        self.counters.gso_datagrams += count as u64;
        self.counters.max_send_batch = self.counters.max_send_batch.max(count as u64);
        Ok(Some(false))
    }
}

fn gro_size(msg: &libc::msghdr) -> Result<usize> {
    // SAFETY: recvmsg returned msg_controllen into our aligned, live control storage.
    let mut header = unsafe { libc::CMSG_FIRSTHDR(msg) };
    while !header.is_null() {
        // SAFETY: header comes from the kernel-ABI CMSG iterator over bounded storage.
        let value = unsafe { &*header };
        // SAFETY: fixed-size CMSG length computation has no memory inputs.
        let base = unsafe { libc::CMSG_LEN(0) } as usize;
        let offset = header as usize - msg.msg_control as usize;
        let end = offset
            .checked_add(value.cmsg_len as usize)
            .ok_or(Error::Invalid("UDP control overflow"))?;
        // msg_controllen is usize on glibc and socklen_t on musl.
        if (value.cmsg_len as usize) < base || end as u64 > msg.msg_controllen as u64 {
            return Err(Error::Invalid("UDP control length"));
        }
        if value.cmsg_level == libc::SOL_UDP && value.cmsg_type == libc::UDP_GRO {
            if (value.cmsg_len as usize) < base + 2 {
                return Err(Error::Invalid("UDP GRO size"));
            }
            // SAFETY: checked two-byte data size, read unaligned rather than assuming ABI alignment.
            let size = unsafe { std::ptr::read_unaligned(libc::CMSG_DATA(header).cast::<u16>()) };
            if size == 0 {
                return Err(Error::Invalid("zero UDP GRO segment"));
            }
            return Ok(size as usize);
        }
        // SAFETY: previous header is within this same live msghdr's control region.
        header = unsafe { libc::CMSG_NXTHDR(msg, header) };
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sockets(ipv6: bool) -> (UdpSocket, UdpSocket) {
        let address = if ipv6 { "[::1]:0" } else { "127.0.0.1:0" };
        let a = UdpSocket::bind(address).unwrap();
        let b = UdpSocket::bind(address).unwrap();
        a.connect(b.local_addr().unwrap()).unwrap();
        b.connect(a.local_addr().unwrap()).unwrap();
        a.set_nonblocking(true).unwrap();
        b.set_nonblocking(true).unwrap();
        (a, b)
    }
    #[test]
    fn actual_batch_and_gso_gro_preserve_datagram_boundaries_and_order() {
        for ipv6 in [false, true] {
            for offload in [false, true] {
                let (a, b) = sockets(ipv6);
                let mut tx = Batch::new(&a, offload);
                let mut rx = Batch::new(&b, offload);
                let mut pool = PacketPool::new(96, 65535).unwrap();
                let mut queue = PacketQueue::new(96).unwrap();
                let mut sizes = vec![
                    0, 16, 16, 1280, 1280, 1280, 1280, 1280, 1280, 1280, 1280, 100, 5000, 1280,
                ];
                // Cross several full syscall batches and exercise a short final
                // datagram after several GSO groups.
                sizes.extend(std::iter::repeat_n(1280, 2 * MAX_BATCH));
                sizes.push(17);
                let expected: Vec<Vec<u8>> = sizes
                    .iter()
                    .enumerate()
                    .map(|(i, n)| vec![i as u8; *n])
                    .collect();
                for data in &expected {
                    let mut packet = pool.take().unwrap();
                    packet.writable()[..data.len()].copy_from_slice(data);
                    packet.set_len(data.len()).unwrap();
                    assert!(queue.push(packet).is_ok());
                }
                while tx.flush(&a, &mut queue, &mut pool).unwrap() {}
                let mut output = pool.take().unwrap();
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
                for data in expected {
                    loop {
                        assert!(std::time::Instant::now() < deadline, "UDP receive timeout");
                        match rx.receive(&b, &mut output, &mut pool).unwrap() {
                            Receive::Packet => break,
                            _ => std::thread::sleep(std::time::Duration::from_millis(1)),
                        }
                    }
                    assert_eq!(output.data(), data);
                }
                assert_eq!(tx.counters.sent_datagrams, sizes.len() as u64);
                assert_eq!(rx.counters.received_datagrams, sizes.len() as u64);
                if offload {
                    assert!(tx.counters.gso_datagrams > 0 || tx.counters.kernel_fallbacks > 0);
                }
            }
        }
    }
    #[test]
    fn partial_batch_acknowledgement_retains_unsent_packet_ownership() {
        let (socket, _) = sockets(false);
        let mut batch = Batch::new(&socket, false);
        let mut pool = PacketPool::new(4, 65535).unwrap();
        let mut queue = PacketQueue::new(4).unwrap();
        for byte in [1, 2, 3] {
            let mut p = pool.take().unwrap();
            p.writable()[0] = byte;
            p.set_len(1).unwrap();
            assert!(queue.push(p).is_ok());
        }
        // A kernel returning one accepted message must leave the remaining two intact.
        // SAFETY: an empty syscall record is valid; only msg_len is read by acknowledge.
        let mut accepted: libc::mmsghdr = unsafe { std::mem::zeroed() };
        accepted.msg_len = 1;
        batch
            .acknowledge(&mut queue, &mut pool, &[accepted])
            .unwrap();
        assert_eq!(queue.len(), 2);
        assert_eq!(pool.available(), 2);
        assert_eq!(queue.front().unwrap().data(), [2]);
        assert_eq!(queue.pop().unwrap().data(), [2]);
        assert_eq!(queue.pop().unwrap().data(), [3]);
    }
    #[test]
    fn malformed_gro_control_lengths_are_rejected() {
        let mut control = Control([0; 64]);
        // SAFETY: initialized empty descriptor whose control points to live aligned storage.
        let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
        msg.msg_control = control.0.as_mut_ptr().cast();
        msg.msg_controllen = 64;
        // SAFETY: aligned complete cmsghdr storage, constructed as a malformed test fixture.
        unsafe {
            let header = libc::CMSG_FIRSTHDR(&msg);
            (*header).cmsg_level = libc::SOL_UDP;
            (*header).cmsg_type = libc::UDP_GRO;
            (*header).cmsg_len = 4096;
        }
        assert!(gro_size(&msg).is_err());
    }
}
