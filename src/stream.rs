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
    first: usize,
    end: usize,
    received: PacketQueue,
    #[cfg(not(any(windows, target_vendor = "apple")))]
    receive_error: Option<Error>,
}
impl<S: Read + Write> FramedIo<S> {
    pub fn new(io: S, queue_limit: usize) -> Result<Self> {
        Self::with_scratch(io, queue_limit, 16384)
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    pub(crate) fn borrowing(io: S, queue_limit: usize) -> Result<Self> {
        Self::with_scratch(io, queue_limit, 0)
    }
    fn with_scratch(io: S, queue_limit: usize, scratch: usize) -> Result<Self> {
        Ok(Self {
            io,
            encoder: Encoder::default(),
            decoder: Decoder::new(65536)?,
            pending: PacketQueue::new(queue_limit)?,
            limit: queue_limit,
            scratch: vec![0; scratch].into_boxed_slice(),
            first: 0,
            end: 0,
            received: PacketQueue::new(queue_limit)?,
            #[cfg(not(any(windows, target_vendor = "apple")))]
            receive_error: None,
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
            let mut input = &self.scratch[self.first..self.end];
            let result = self.decoder.decode_from(&mut input, output);
            self.first = self.end - input.len();
            if result? {
                return Ok(Receive::Packet);
            }
            self.first = 0;
            self.end = 0;
            if self.scratch.is_empty() {
                return Err(Error::Full);
            }
            match self.io.read(&mut self.scratch) {
                Ok(0) if self.decoder.buffered() == 0 => return Ok(Receive::Eof),
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "partial WireGuard TCP frame",
                    )
                    .into());
                }
                Ok(n) => self.end = n,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return Ok(Receive::WouldBlock);
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        // Caller must reschedule without relying on another edge-triggered event.
        Ok(Receive::Yield)
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    pub(crate) fn receive_borrowed(
        &mut self,
        output: &mut Packet,
        pool: &mut PacketPool,
        mut read: impl FnMut(&mut S, &mut dyn FnMut(&[u8]) -> io::Result<usize>) -> io::Result<usize>,
    ) -> Result<Receive> {
        if let Some(mut packet) = self.received.pop() {
            std::mem::swap(output, &mut packet);
            pool.recycle(packet)?;
            return Ok(Receive::Packet);
        }
        if let Some(error) = self.receive_error.take() {
            return Err(error);
        }
        for _ in 0..64 {
            if self.decoder.decode_into(output)? {
                return Ok(Receive::Packet);
            }
            let mut produced = false;
            let result = read(&mut self.io, &mut |data| {
                let mut input = data;
                produced = self
                    .decoder
                    .decode_from(&mut input, output)
                    .map_err(io::Error::other)?;
                // Consume this TLS record while it is borrowed. Prefetch only
                // already available frames, bounded by the queue and pool;
                // reserve two buffers for the worker/control path. Any suffix
                // that cannot be extracted now uses the existing bounded ring.
                while produced
                    && !input.is_empty()
                    && self.received.len() < self.limit
                    && pool.available() > 2
                {
                    let mut packet = pool
                        .take_sized(65535)
                        .ok_or_else(|| io::Error::other(Error::Full))?;
                    match self.decoder.decode_from(&mut input, &mut packet) {
                        Ok(true) => self
                            .received
                            .push(packet)
                            .map_err(|_| io::Error::other(Error::Full))?,
                        Ok(false) => {
                            pool.recycle(packet).map_err(io::Error::other)?;
                            break;
                        }
                        Err(error) => {
                            pool.recycle(packet).map_err(io::Error::other)?;
                            // Deliver the valid prefix before reporting a later
                            // malformed frame, just like incremental decoding.
                            self.receive_error = Some(error);
                            return Ok(data.len());
                        }
                    }
                }
                self.decoder.feed(input).map_err(io::Error::other)?;
                Ok(data.len())
            });
            if produced {
                result?;
                return Ok(Receive::Packet);
            }
            match result {
                Ok(0) if self.decoder.buffered() == 0 => return Ok(Receive::Eof),
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "partial WireGuard TCP frame",
                    )
                    .into());
                }
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(Receive::WouldBlock),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(Receive::Yield)
    }
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
    pub fn io(&self) -> &S {
        &self.io
    }
    pub fn io_mut(&mut self) -> &mut S {
        &mut self.io
    }
    pub fn close(mut self, pool: &mut PacketPool) -> Result<S> {
        while let Some(p) = self.pending.pop() {
            pool.recycle(p)?;
        }
        while let Some(p) = self.received.pop() {
            pool.recycle(p)?;
        }
        Ok(self.io)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn frames(count: usize) -> (Vec<u8>, Vec<Vec<u8>>) {
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut encoder = Encoder::default();
        let mut wire = Vec::new();
        let mut expected = Vec::new();
        for n in 0..count {
            let mut p = pool.take().unwrap();
            p.writable()[..80].fill(n as u8);
            p.writable()[..4].copy_from_slice(&[4, 0, 0, 0]);
            p.writable()[4..8].copy_from_slice(&7u32.to_le_bytes());
            p.writable()[8..16].copy_from_slice(&(n as u64).to_le_bytes());
            p.set_len(80).unwrap();
            expected.push(p.data().to_vec());
            encoder.frame(&mut p).unwrap();
            wire.extend_from_slice(p.data());
            pool.recycle(p).unwrap();
        }
        (wire, expected)
    }
    #[test]
    fn complete_receive_spans_bypass_ring_and_preserve_unconsumed_frames() {
        let (wire, expected) = frames(80);
        let mut io = FramedIo::new(io::Cursor::new(wire), 64).unwrap();
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut output = pool.take().unwrap();
        for packet in expected {
            assert_eq!(io.receive(&mut output).unwrap(), Receive::Packet);
            assert_eq!(output.data(), packet);
            assert_eq!(io.decoder.buffered(), 0);
        }
        assert_eq!(io.receive(&mut output).unwrap(), Receive::Eof);
    }
    #[test]
    fn receive_fragmented_eof_is_not_a_clean_close() {
        for bytes in [vec![0], vec![0, 16, 1, 2, 3]] {
            let mut io = FramedIo::new(io::Cursor::new(bytes), 64).unwrap();
            let mut pool = PacketPool::new(1, 200).unwrap();
            let mut output = pool.take().unwrap();
            assert!(
                matches!(io.receive(&mut output), Err(Error::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof)
            );
        }
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    #[test]
    fn borrowed_records_bound_prefetch_and_fallback_under_pool_pressure() {
        for count in [3, 6, 96] {
            for limit in [1, 4, 64] {
                let (wire, expected) = frames(80);
                let mut io = FramedIo::borrowing(io::Cursor::new(wire), limit).unwrap();
                let mut pool = PacketPool::new(count, 65535).unwrap();
                let mut output = pool.take().unwrap();
                for packet in expected {
                    let result = io
                        .receive_borrowed(&mut output, &mut pool, |io, consume| {
                            let first = io.position() as usize;
                            let n = consume(&io.get_ref()[first..])?;
                            io.set_position((first + n) as u64);
                            Ok(n)
                        })
                        .unwrap();
                    assert_eq!(result, Receive::Packet);
                    assert_eq!(output.data(), packet);
                    assert!(io.received.len() <= limit);
                    assert!(pool.available() >= 2);
                }
                io.close(&mut pool).unwrap();
                assert_eq!(pool.available(), count - 1);
            }
        }
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    #[test]
    fn closing_borrowed_stream_recycles_prefetched_packet_ownership() {
        let (wire, _) = frames(8);
        let mut io = FramedIo::borrowing(io::Cursor::new(wire), 4).unwrap();
        let mut pool = PacketPool::new(10, 65535).unwrap();
        let mut output = pool.take().unwrap();
        assert_eq!(
            io.receive_borrowed(&mut output, &mut pool, |io, consume| consume(io.get_ref()))
                .unwrap(),
            Receive::Packet
        );
        assert_eq!(io.received.len(), 4);
        io.close(&mut pool).unwrap();
        assert_eq!(pool.available(), 9);
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    #[test]
    fn borrowed_prefetch_delivers_valid_prefix_before_malformed_tail() {
        let (mut wire, expected) = frames(2);
        wire.extend_from_slice(&[0x40, 16]);
        let mut io = FramedIo::borrowing(io::Cursor::new(wire), 4).unwrap();
        let mut pool = PacketPool::new(10, 65535).unwrap();
        let mut output = pool.take().unwrap();
        for packet in expected {
            assert_eq!(
                io.receive_borrowed(&mut output, &mut pool, |io, consume| consume(io.get_ref()))
                    .unwrap(),
                Receive::Packet
            );
            assert_eq!(output.data(), packet);
        }
        assert!(
            io.receive_borrowed(&mut output, &mut pool, |_, _| panic!(
                "invalid stream must not be read again"
            ))
            .is_err()
        );
        io.close(&mut pool).unwrap();
        assert_eq!(pool.available(), 9);
    }
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
