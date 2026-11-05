//! Fixed-loopback worker screen only. Blocking setup and incomplete control-write
//! recovery/record-limit policy exclude this adapter from production use.
//! The unchanged caller supplies both its configured TLS connection and socket.
use crate::{
    Result,
    network::{ConnectingTcp, WireSocket},
    packet::Packet,
    stream::{FramedIo, Receive},
};
use std::{
    io::{self, IoSlice, Read, Write},
    net::TcpStream,
    os::fd::FromRawFd,
    time::Duration,
};
mod core;
pub(crate) struct Io {
    socket: ConnectingTcp,
    tls: core::Ktls,
}
pub(crate) fn framed(
    mut socket: ConnectingTcp,
    connection: crate::tls::ClientConnection,
) -> Result<FramedIo<Io>> {
    // Duplicate the owned socket, preserving uplink binding/protection and the
    // existing trust policy/SNI in `connection`. No new network connection.
    let fd = unsafe { libc::fcntl(socket.fd(), libc::F_DUPFD_CLOEXEC, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let tcp = unsafe { TcpStream::from_raw_fd(fd) };
    let mut pollfd = libc::pollfd {
        fd,
        events: libc::POLLOUT,
        revents: 0,
    };
    let n = unsafe { libc::poll(&mut pollfd, 1, 5000) };
    if n <= 0 {
        return Err(if n == 0 {
            io::ErrorKind::TimedOut.into()
        } else {
            io::Error::last_os_error()
        }
        .into());
    }
    if let Some(e) = tcp.take_error()? {
        return Err(e.into());
    }
    tcp.set_nonblocking(false)?;
    tcp.set_read_timeout(Some(Duration::from_secs(5)))?;
    tcp.set_write_timeout(Some(Duration::from_secs(5)))?;
    let tls = core::Ktls::from_connection(tcp, connection)?;
    println!("KTLS_WORKER_READY {}", tls.suite());
    // File status is shared by dup; restore nonblocking I/O for the real reactor.
    // The socket remains owned entirely by this isolated benchmark connection.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    socket.flush()?; // commits ConnectingTcp's completed SYN state
    FramedIo::borrowing(Io { socket, tls }, 64)
}
impl Read for Io {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        self.tls.read(b)
    }
}
impl Write for Io {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.tls.write(b)
    }
    fn write_vectored(&mut self, b: &[IoSlice<'_>]) -> io::Result<usize> {
        self.tls.write_vectored(b)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.tls.flush()
    }
}
impl WireSocket for Io {
    fn fd(&self) -> i32 {
        self.socket.fd()
    }
    fn register(&mut self, r: &mio::Registry, t: mio::Token, i: mio::Interest) -> io::Result<()> {
        self.socket.register(r, t, i)
    }
    fn reregister(&mut self, r: &mio::Registry, t: mio::Token, i: mio::Interest) -> io::Result<()> {
        self.socket.reregister(r, t, i)
    }
    fn receive_frame(stream: &mut FramedIo<Self>, packet: &mut Packet) -> Result<Receive> {
        stream.receive_using(packet, |io, decoder, _| {
            Ok(io.tls.read_plaintext(|data| {
                decoder.feed(data).map_err(io::Error::other)?;
                Ok(data.len())
            })?)
        })
    }
}
