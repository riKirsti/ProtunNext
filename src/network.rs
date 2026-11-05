//! Reactor-facing transports. A single bounded packet worker serves UDP/TCP/TLS.
#[cfg(not(any(target_os = "linux", target_os = "android")))]
use crate::Error;
use crate::{
    Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::{Flush, FramedIo, Receive},
    tls::TlsIo,
};
#[cfg(unix)]
use mio::unix::SourceFd;
use mio::{Interest, Token};
use std::io::{self, IoSlice, Read, Write};

#[cfg(windows)]
use mio::net::{TcpStream, UdpSocket};
#[cfg(unix)]
use std::{
    net::{TcpStream, UdpSocket},
    os::fd::AsRawFd,
};

pub(crate) trait Network {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        Default::default()
    }
    fn retry_delay(&self) -> Option<std::time::Duration> {
        None
    }
    fn max_ip_packet(&self) -> usize {
        65_503 // 65,535-byte WG datagram minus data header and AEAD tag.
    }
    #[cfg(unix)]
    fn fd(&self) -> i32;
    #[cfg(unix)]
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()> {
        registry.register(&mut SourceFd(&self.fd()), token, interest)
    }
    #[cfg(unix)]
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()> {
        registry.reregister(&mut SourceFd(&self.fd()), token, interest)
    }
    #[cfg(windows)]
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()>;
    #[cfg(windows)]
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: Token,
        interest: Interest,
    ) -> io::Result<()>;
    fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive>;
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
pub(crate) struct Datagram {
    socket: UdpSocket,
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    counters: crate::performance::IoCounters,
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    retry_at: Option<std::time::Instant>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    batch: crate::linux_batch::Batch,
}
impl Datagram {
    pub(crate) fn new(socket: UdpSocket) -> Self {
        Self {
            #[cfg(any(target_os = "linux", target_os = "android"))]
            batch: crate::linux_batch::Batch::new(&socket, true),
            socket,
            #[cfg(not(any(target_os = "linux", target_os = "android")))]
            counters: Default::default(),
            #[cfg(not(any(target_os = "linux", target_os = "android")))]
            retry_at: None,
        }
    }
}
impl Network for Datagram {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.counters
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            self.counters
        }
    }
    fn retry_delay(&self) -> Option<std::time::Duration> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.retry_delay()
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            self.retry_at
                .map(|when| when.saturating_duration_since(std::time::Instant::now()))
                .filter(|delay| !delay.is_zero())
        }
    }
    #[cfg(windows)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut self.socket, t, i)
    }
    #[cfg(unix)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut SourceFd(&self.fd()), t, i)?;
        Ok(())
    }
    #[cfg(windows)]
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.reregister(&mut self.socket, t, i)
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket.as_raw_fd()
    }
    fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.receive(&self.socket, packet, pool)
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            let _ = pool;
            self.counters.receive_calls += 1;
            let result = self.socket.recv(packet.writable());
            if let Err(error) = &result {
                self.counters.socket_error(error, false);
            }
            match result {
                Ok(n) => {
                    packet.set_len(n)?;
                    self.counters.received_datagrams += 1;
                    self.counters.received_bytes += n as u64;
                    self.counters.max_receive_batch = self.counters.max_receive_batch.max(1);
                    Ok(Receive::Packet)
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(Receive::WouldBlock),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => Ok(Receive::Yield),
                Err(e) => Err(e.into()),
            }
        }
    }
    fn flush(&mut self, queue: &mut PacketQueue, pool: &mut PacketPool) -> Result<bool> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            self.batch.flush(&self.socket, queue, pool)
        }
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            if self.retry_delay().is_some() {
                return Ok(!queue.is_empty());
            }
            self.retry_at = None;
            for _ in 0..64 {
                let Some(packet) = queue.front() else {
                    return Ok(false);
                };
                self.counters.send_calls += 1;
                let result = self.socket.send(packet.data());
                if let Err(error) = &result {
                    self.counters.socket_error(error, true);
                }
                match result {
                    Ok(n) if n == packet.data().len() => {
                        pool.recycle(queue.pop().expect("front exists"))?;
                        self.counters.sent_datagrams += 1;
                        self.counters.sent_bytes += n as u64;
                        self.counters.max_send_batch = 1;
                    }
                    Ok(_) => return Err(Error::Invalid("partial UDP send")),
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(true),
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) if crate::platform::send_pressure(&e) => {
                        self.counters.enobufs += 1;
                        self.retry_at =
                            Some(std::time::Instant::now() + std::time::Duration::from_millis(2));
                        return Ok(true);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Ok(!queue.is_empty())
        }
    }
}

pub(crate) trait WireSocket: Read + Write {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        Default::default()
    }
    fn receive_frame(stream: &mut FramedIo<Self>, packet: &mut Packet) -> Result<Receive>
    where
        Self: Sized,
    {
        stream.receive(packet)
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()>;
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()>;

    #[cfg(unix)]
    fn fd(&self) -> i32;
    fn wants_write(&self) -> bool {
        false
    }
    fn handshaking(&self) -> bool {
        false
    }
}
impl WireSocket for TcpStream {
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        #[cfg(unix)]
        {
            r.register(&mut SourceFd(&self.as_raw_fd()), t, i)
        }
        #[cfg(windows)]
        {
            r.register(self, t, i)
        }
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        #[cfg(unix)]
        {
            r.reregister(&mut SourceFd(&self.as_raw_fd()), t, i)
        }
        #[cfg(windows)]
        {
            r.reregister(self, t, i)
        }
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.as_raw_fd()
    }
}
/// A nonblocking SYN is completed by the packet reactor rather than a setup
/// wait loop. The TUN and local service continue making progress meanwhile.
pub(crate) struct ConnectingTcp {
    socket: TcpStream,
    connected: bool,
    counters: crate::performance::IoCounters,
}
impl ConnectingTcp {
    pub(crate) fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            connected: false,
            counters: Default::default(),
        }
    }
    fn ready(&mut self) -> io::Result<()> {
        if self.connected {
            return Ok(());
        }
        if let Some(error) = self.socket.take_error()? {
            return Err(error);
        }
        match self.socket.peer_addr() {
            Ok(_) => {
                self.connected = true;
                Ok(())
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotConnected | io::ErrorKind::WouldBlock
                ) =>
            {
                Err(io::ErrorKind::WouldBlock.into())
            }
            Err(error) => Err(error),
        }
    }
}
impl Read for ConnectingTcp {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.ready()?;
        self.counters.receive_calls += 1;
        let result = self.socket.read(bytes);
        if let Err(error) = &result {
            self.counters.socket_error(error, false);
        }
        if let Ok(n) = result {
            self.counters.received_bytes += n as u64;
        }
        result
    }
}
impl Write for ConnectingTcp {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.ready()?;
        self.counters.send_calls += 1;
        let result = self.socket.write(bytes);
        if let Err(error) = &result {
            self.counters.socket_error(error, true);
        }
        if let Ok(n) = result {
            self.counters.sent_bytes += n as u64;
        }
        result
    }
    fn write_vectored(&mut self, bytes: &[IoSlice<'_>]) -> io::Result<usize> {
        self.ready()?;
        self.counters.send_calls += 1;
        let result = self.socket.write_vectored(bytes);
        if let Err(error) = &result {
            self.counters.socket_error(error, true);
        }
        if let Ok(n) = result {
            self.counters.sent_bytes += n as u64;
        }
        result
    }
    fn flush(&mut self) -> io::Result<()> {
        self.ready()?;
        self.socket.flush()
    }
}
impl WireSocket for ConnectingTcp {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        self.counters
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(&mut self.socket, r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(&mut self.socket, r, t, i)
    }
    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket.as_raw_fd()
    }
    fn wants_write(&self) -> bool {
        !self.connected
    }
    fn handshaking(&self) -> bool {
        !self.connected
    }
}
impl<S: WireSocket> WireSocket for TlsIo<S> {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        self.socket().diagnostics()
    }
    fn receive_frame(stream: &mut FramedIo<Self>, packet: &mut Packet) -> Result<Receive> {
        #[cfg(any(windows, target_vendor = "apple"))]
        {
            stream.receive(packet)
        }
        #[cfg(not(any(windows, target_vendor = "apple")))]
        {
            stream.receive_using(packet, |io, decoder, _| {
                Ok(io.read_plaintext(|data| {
                    decoder.feed(data).map_err(io::Error::other)?;
                    Ok(data.len())
                })?)
            })
        }
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        self.socket_mut().register(r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        self.socket_mut().reregister(r, t, i)
    }
    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.socket().fd()
    }
    fn wants_write(&self) -> bool {
        self.wants_write() || self.socket().wants_write()
    }
    fn handshaking(&self) -> bool {
        self.is_handshaking() || self.socket().handshaking()
    }
}
pub(crate) struct Stream<S>(pub FramedIo<S>);
impl<S: WireSocket> Network for Stream<S> {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        self.0.io().diagnostics()
    }
    fn max_ip_packet(&self) -> usize {
        0x3fff - 32 // Must also fit a normal, uncompressed TunSafe frame.
    }
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(self.0.io_mut(), r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(self.0.io_mut(), r, t, i)
    }

    #[cfg(unix)]
    fn fd(&self) -> i32 {
        self.0.io().fd()
    }
    fn receive(&mut self, packet: &mut Packet, _: &mut PacketPool) -> Result<Receive> {
        S::receive_frame(&mut self.0, packet)
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
