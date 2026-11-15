//! Reactor contract shared by the platform transports.
//! Socket setup, datagrams and framed streams have separate private owners.
mod datagram;
mod socket;
mod stream;
#[cfg(unix)]
pub(crate) mod unix;

pub(crate) use datagram::Datagram;
pub(crate) use socket::ConnectingTcp;
pub(crate) use stream::Stream;

use crate::{
    Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::Receive,
};
#[cfg(unix)]
use mio::unix::SourceFd;
use mio::{Interest, Token};
use std::io;

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
