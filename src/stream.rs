//! Reusable framed TCP/TLS transport with bounded queues and vectored writes.
//! Connection ownership is independent of the crypto engine and account runtime.
use crate::{
    Error, Result,
    framing::{Decoder, Encoder},
    packet::{Packet, PacketPool, PacketQueue},
};
use std::io::{self, IoSlice, Read, Write};
pub struct Rejected {
    pub error: Error,
    pub packet: Packet,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Flush {
    Empty,
    Progress,
    WouldBlock,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Receive {
    Packet,
    WouldBlock,
    Yield,
    Eof,
}
pub struct FramedIo<S> {
    io: S,
    encoder: Encoder,
    decoder: Decoder,
    pending: PacketQueue,
    limit: usize,
    scratch: Box<[u8]>,
}
impl<S: Read + Write> FramedIo<S> {
    pub fn new(io: S, queue_limit: usize) -> Result<Self> {
        Ok(Self {
            io,
            encoder: Encoder::default(),
            decoder: Decoder::new(65536)?,
            pending: PacketQueue::new(queue_limit)?,
            limit: queue_limit,
            scratch: vec![0; 16384].into_boxed_slice(),
        })
    }
    pub fn queue(&mut self, mut packet: Packet) -> std::result::Result<(), Rejected> {
        if self.pending.len() == self.limit {
            return Err(Rejected {
                error: Error::Full,
                packet,
            });
        }
        if let Err(error) = self.encoder.frame(&mut packet) {
            return Err(Rejected { error, packet });
        }
        // Capacity was checked before advancing compression state.
        self.pending.push(packet).map_err(|packet| Rejected {
            error: Error::Full,
            packet,
        })
    }
    pub fn flush(&mut self, pool: &mut PacketPool) -> Result<Flush> {
        if self.pending.is_empty() {
            return Ok(Flush::Empty);
        }
        let mut views: [IoSlice<'_>; 64] = std::array::from_fn(|_| IoSlice::new(&[]));
        let mut count = 0;
        let mut total = 0;
        for (n, p) in self.pending.iter().take(64).enumerate() {
            views[n] = IoSlice::new(p.data());
            count = n + 1;
            total += p.data().len();
        }
        let written = match self.io.write_vectored(&views[..count]) {
            Ok(0) => {
                return Err(
                    io::Error::new(io::ErrorKind::WriteZero, "stream accepted no bytes").into(),
                );
            }
            Ok(n) if n <= total => n,
            Ok(_) => return Err(Error::Invalid("stream write count")),
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::Interrupted =>
            {
                return Ok(Flush::WouldBlock);
            }
            Err(e) => return Err(e.into()),
        };
        let mut remaining = written;
        while remaining > 0 {
            let p = self
                .pending
                .front_mut()
                .ok_or(Error::Invalid("stream queue accounting"))?;
            let used = remaining.min(p.data().len());
            p.consume(used)?;
            remaining -= used;
            if p.is_empty() {
                pool.recycle(self.pending.pop().expect("front exists"))?;
            }
        }
        Ok(if self.pending.is_empty() {
            Flush::Empty
        } else {
            Flush::Progress
        })
    }
    pub fn receive(&mut self, output: &mut Packet) -> Result<Receive> {
        for _ in 0..64 {
            if self.decoder.decode_into(output)? {
                return Ok(Receive::Packet);
            }
            let capacity = self.scratch.len().min(self.decoder.remaining());
            if capacity == 0 {
                return Err(Error::Full);
            }
            match self.io.read(&mut self.scratch[..capacity]) {
                Ok(0) if self.decoder.buffered() == 0 => return Ok(Receive::Eof),
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "partial WireGuard TCP frame",
                    )
                    .into());
                }
                Ok(n) => self.decoder.feed(&self.scratch[..n])?,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(Receive::WouldBlock),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        // Caller must reschedule without relying on another edge-triggered event.
        Ok(Receive::Yield)
    }
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
    pub fn close(mut self, pool: &mut PacketPool) -> Result<S> {
        while let Some(p) = self.pending.pop() {
            pool.recycle(p)?;
        }
        Ok(self.io)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Short {
        bytes: Vec<u8>,
        max: usize,
        block: bool,
    }
    impl Read for Short {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::WouldBlock.into())
        }
    }
    impl Write for Short {
        fn write(&mut self, p: &[u8]) -> io::Result<usize> {
            if self.block {
                return Err(io::ErrorKind::WouldBlock.into());
            }
            let n = p.len().min(self.max);
            self.bytes.extend_from_slice(&p[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn short_writes_and_backpressure_preserve_exact_frame() {
        let mut pool = PacketPool::new(2, 100).unwrap();
        let mut packet = pool.take().unwrap();
        packet.writable()[..32].fill(0);
        packet.writable()[0] = 4;
        packet.set_len(32).unwrap();
        let mut expected = vec![0, 32];
        expected.extend_from_slice(packet.data());
        let mut stream = FramedIo::new(
            Short {
                bytes: Vec::new(),
                max: 3,
                block: true,
            },
            1,
        )
        .unwrap();
        assert!(stream.queue(packet).is_ok());
        assert_eq!(stream.flush(&mut pool).unwrap(), Flush::WouldBlock);
        stream.io.block = false;
        while stream.pending() > 0 {
            stream.flush(&mut pool).unwrap();
        }
        assert_eq!(stream.close(&mut pool).unwrap().bytes, expected);
        assert_eq!(pool.available(), 2);
    }
}
