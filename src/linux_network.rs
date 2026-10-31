//! Reactor-facing transports. A single bounded packet worker serves UDP/TCP/TLS.
use crate::{
    Error, Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::{Flush, FramedIo, Receive},
    tls::TlsIo,
};
use mio::{Events, Interest, Poll, Token, unix::SourceFd};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream, UdpSocket},
    os::fd::AsRawFd,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

pub(crate) trait Network {
    fn fd(&self) -> i32;
    fn receive(&mut self, packet: &mut Packet) -> Result<Receive>;
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool>;
    fn pending(&self) -> usize {
        0
    }
    fn wants_write(&self) -> bool {
        false
    }
    fn handshaking(&self) -> bool {
        false
    }
}
pub(crate) struct Datagram(pub UdpSocket);
impl Network for Datagram {
    fn fd(&self) -> i32 {
        self.0.as_raw_fd()
    }
    fn receive(&mut self, packet: &mut Packet) -> Result<Receive> {
        match self.0.recv(packet.writable()) {
            Ok(n) => {
                packet.set_len(n)?;
                Ok(Receive::Packet)
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(Receive::WouldBlock),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => Ok(Receive::Yield),
            Err(e) => Err(e.into()),
        }
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        for _ in 0..64 {
            let Some(packet) = queue.front() else {
                return Ok(false);
            };
            match self.0.send(packet.data()) {
                Ok(n) if n == packet.data().len() => {
                    pool.recycle(queue.pop().expect("front exists"))?
                }
                Ok(_) => return Err(Error::Invalid("partial UDP send")),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(true),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(!queue.is_empty())
    }
}
pub(crate) trait WireSocket: Read + Write {
    fn fd(&self) -> i32;
    fn wants_write(&self) -> bool {
        false
    }
    fn handshaking(&self) -> bool {
        false
    }
}
impl WireSocket for TcpStream {
    fn fd(&self) -> i32 {
        self.as_raw_fd()
    }
}
impl WireSocket for TlsIo<TcpStream> {
    fn fd(&self) -> i32 {
        self.socket().as_raw_fd()
    }
    fn wants_write(&self) -> bool {
        self.wants_write()
    }
    fn handshaking(&self) -> bool {
        self.is_handshaking()
    }
}
pub(crate) struct Stream<S>(pub FramedIo<S>);
impl<S: WireSocket> Network for Stream<S> {
    fn fd(&self) -> i32 {
        self.0.io().fd()
    }
    fn receive(&mut self, packet: &mut Packet) -> Result<Receive> {
        self.0.receive(packet)
    }
    fn pending(&self) -> usize {
        self.0.pending()
    }
    fn wants_write(&self) -> bool {
        self.0.io().wants_write()
    }
    fn handshaking(&self) -> bool {
        self.0.io().handshaking()
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        while self.0.pending() < 64 {
            let Some(packet) = queue.pop() else {
                break;
            };
            if let Err(rejected) = self.0.queue(packet) {
                pool.recycle(rejected.packet)?;
                return Err(rejected.error);
            }
        }
        for _ in 0..64 {
            match self.0.flush(pool)? {
                Flush::Empty | Flush::WouldBlock => break,
                Flush::Progress => {}
            }
        }
        // TLS ciphertext can remain buffered after the final plaintext packet
        // has left FramedIo. Always pump it and retain writable interest.
        match self.0.io_mut().flush() {
            Ok(()) => {}
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
        Ok(self.0.pending() > 0 || self.wants_write())
    }
}

pub(crate) fn bind_device(fd: i32, device: &str, ipv6: bool) -> Result<()> {
    if device.is_empty() || device.len() >= libc::IFNAMSIZ || device.as_bytes().contains(&0) {
        return Err(Error::Invalid("uplink device"));
    }
    let device = std::ffi::CString::new(device).map_err(|_| Error::Invalid("uplink device"))?;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let _ = ipv6;
        // SAFETY: live fd and bounded, NUL-terminated interface name.
        if unsafe {
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_BINDTODEVICE,
                device.as_ptr().cast(),
                device.as_bytes_with_nul().len() as libc::socklen_t,
            )
        } < 0
        {
            return Err(io::Error::last_os_error().into());
        }
    }
    #[cfg(target_vendor = "apple")]
    {
        // SAFETY: bounded, NUL-terminated interface name remains valid.
        let index = unsafe { libc::if_nametoindex(device.as_ptr()) };
        if index == 0 {
            return Err(io::Error::last_os_error().into());
        }
        let (level, option) = if ipv6 {
            (libc::IPPROTO_IPV6, libc::IPV6_BOUND_IF)
        } else {
            (libc::IPPROTO_IP, libc::IP_BOUND_IF)
        };
        // SAFETY: live socket, correctly sized read-only interface-index scalar.
        if unsafe {
            libc::setsockopt(
                fd,
                level,
                option,
                (&index as *const u32).cast(),
                size_of::<u32>() as libc::socklen_t,
            )
        } < 0
        {
            return Err(io::Error::last_os_error().into());
        }
    }
    Ok(())
}

/// Protect against recursive routing before connect sends the first SYN.
pub(crate) fn connect_tcp(
    endpoint: SocketAddr,
    device: &str,
    stop: &AtomicBool,
    protector: Option<&dyn crate::api::types::SocketProtector>,
) -> Result<TcpStream> {
    let socket = socket2::Socket::new(
        socket2::Domain::for_address(endpoint),
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;
    socket.set_nonblocking(true)?;
    let fd = socket.as_raw_fd();
    if !device.is_empty() {
        bind_device(fd, device, endpoint.is_ipv6())?;
    }
    if let Some(protector) = protector
        && !protector.protect_socket(fd)
    {
        return Err(io::Error::from(io::ErrorKind::PermissionDenied).into());
    }
    let connecting = match socket.connect(&endpoint.into()) {
        Ok(()) => false,
        Err(error) if error.raw_os_error() == Some(libc::EINPROGRESS) => true,
        Err(error) => return Err(error.into()),
    };
    let socket: TcpStream = socket.into();
    if connecting {
        let mut poll = Poll::new()?;
        poll.registry()
            .register(&mut SourceFd(&fd), Token(0), Interest::WRITABLE)?;
        let mut events = Events::with_capacity(4);
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if stop.load(Ordering::Relaxed) {
                return Err(io::Error::from(io::ErrorKind::Interrupted).into());
            }
            if Instant::now() >= deadline {
                return Err(io::Error::from(io::ErrorKind::TimedOut).into());
            }
            match poll.poll(&mut events, Some(Duration::from_millis(100))) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                result => result?,
            }
            if events.is_empty() {
                continue;
            }
            if let Some(error) = socket.take_error()? {
                return Err(error.into());
            }
            match socket.peer_addr() {
                Ok(_) => break,
                Err(e)
                    if e.kind() == io::ErrorKind::NotConnected
                        || e.raw_os_error() == Some(libc::EINPROGRESS) =>
                {
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
    socket.set_nodelay(true)?;
    Ok(socket)
}
