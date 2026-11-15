//! Framed TCP/TLS delivery; TLS policy and record buffering stay in their owners.
use super::{Network, socket::WireSocket};
use crate::{
    Result,
    packet::{Packet, PacketPool, PacketQueue},
    stream::{Flush, FramedIo, Receive},
    tls::TlsIo,
};
use mio::{Interest, Token};
use std::io;

impl<S: WireSocket> WireSocket for TlsIo<S> {
    fn diagnostics(&self) -> crate::performance::IoCounters {
        self.socket().diagnostics()
    }
    fn receive_frame(
        stream: &mut FramedIo<Self>,
        packet: &mut Packet,
        pool: &mut PacketPool,
    ) -> Result<Receive> {
        #[cfg(any(windows, target_vendor = "apple"))]
        {
            let _ = pool;
            stream.receive(packet)
        }
        #[cfg(not(any(windows, target_vendor = "apple")))]
        {
            stream.receive_borrowed(packet, pool, |io, consume| io.read_plaintext(consume))
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
    fn receive(&mut self, packet: &mut Packet, pool: &mut PacketPool) -> Result<Receive> {
        S::receive_frame(&mut self.0, packet, pool)
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
