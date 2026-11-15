//! Socket readiness, nonblocking TCP connection state and socket diagnostics.
use crate::{
    Result,
    packet::{Packet, PacketPool},
    stream::{FramedIo, Receive},
};
#[cfg(windows)]
use mio::net::TcpStream;
#[cfg(unix)]
use mio::unix::SourceFd;
use mio::{Interest, Token};
use std::io::{self, IoSlice, Read, Write};
#[cfg(unix)]
use std::{net::TcpStream, os::fd::AsRawFd};

pub(crate) trait WireSocket: Read + Write {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        Default::default()
    }
    fn receive_frame(
        stream: &mut FramedIo<Self>,
        packet: &mut Packet,
        _pool: &mut PacketPool,
    ) -> Result<Receive>
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
