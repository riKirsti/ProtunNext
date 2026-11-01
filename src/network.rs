//! Reactor-facing transports. A single bounded packet worker serves UDP/TCP/TLS.
use crate::{
    Error, Result,
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
    #[cfg(windows)]
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.register(&mut self.0, t, i)
    }
    #[cfg(windows)]
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        r.reregister(&mut self.0, t, i)
    }

    #[cfg(unix)]
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
}
impl ConnectingTcp {
    pub(crate) fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            connected: false,
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
        self.socket.read(bytes)
    }
}
impl Write for ConnectingTcp {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.ready()?;
        self.socket.write(bytes)
    }
    fn write_vectored(&mut self, bytes: &[IoSlice<'_>]) -> io::Result<usize> {
        self.ready()?;
        self.socket.write_vectored(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.ready()?;
        self.socket.flush()
    }
}
impl WireSocket for ConnectingTcp {
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
    fn register(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::register(self.socket_mut(), r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: Token, i: Interest) -> io::Result<()> {
        WireSocket::reregister(self.socket_mut(), r, t, i)
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
