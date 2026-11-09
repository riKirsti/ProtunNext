//! TunSafe wire format, independently implemented from its published specification.
//! https://tunsafe.com/user-guide/tcp — TT=00 normal, TT=10 predicted data.
use crate::{Error, Result, packet::Packet};
const MAX_BODY: usize = 0x3fff;

fn next_header(mut h: [u8; 16]) -> [u8; 16] {
    let n = u64::from_le_bytes(h[8..].try_into().expect("fixed header"));
    h[8..].copy_from_slice(&n.wrapping_add(1).to_le_bytes());
    h
}
fn data_header(p: &[u8]) -> Option<[u8; 16]> {
    if p.len() >= 32 && p[..4] == [4, 0, 0, 0] {
        Some(p[..16].try_into().expect("checked header"))
    } else {
        None
    }
}
#[derive(Default)]
pub struct Encoder {
    previous: Option<[u8; 16]>,
}
impl Encoder {
    pub fn reset(&mut self) {
        self.previous = None;
    }
    pub fn frame(&mut self, p: &mut Packet) -> Result<()> {
        if p.data().len() < 16 {
            return Err(Error::Invalid("short WireGuard datagram"));
        }
        let header = data_header(p.data());
        let compressed = header
            .zip(self.previous)
            .is_some_and(|(h, prev)| h == next_header(prev));
        let size = p.data().len() - if compressed { 16 } else { 0 };
        if size > MAX_BODY {
            return Err(Error::Invalid("TCP frame too large"));
        }
        if compressed {
            p.consume(16)?;
        }
        p.prepend(&((size as u16) | if compressed { 0x8000 } else { 0 }).to_be_bytes())?;
        if let Some(h) = header {
            self.previous = Some(h);
        }
        Ok(())
    }
}

/// Bounded circular storage; acknowledging a frame advances an index, not memmove.
pub struct Decoder {
    bytes: Box<[u8]>,
    head: usize,
    len: usize,
    previous: Option<[u8; 16]>,
    failed: bool,
}
impl Decoder {
    pub fn new(capacity: usize) -> Result<Self> {
        if !(MAX_BODY + 2..=16 * 1024 * 1024).contains(&capacity) {
            return Err(Error::Invalid("decoder capacity"));
        }
        Ok(Self {
            bytes: vec![0; capacity].into_boxed_slice(),
            head: 0,
            len: 0,
            previous: None,
            failed: false,
        })
    }
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.previous = None;
        self.failed = false;
    }
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.len
    }
    pub fn buffered(&self) -> usize {
        self.len
    }
    pub fn feed(&mut self, input: &[u8]) -> Result<()> {
        if self.failed {
            return Err(Error::Invalid("decoder is poisoned"));
        }
        if input.len() > self.remaining() {
            return Err(Error::Full);
        }
        let tail = (self.head + self.len) % self.bytes.len();
        let first = input.len().min(self.bytes.len() - tail);
        self.bytes[tail..tail + first].copy_from_slice(&input[..first]);
        self.bytes[..input.len() - first].copy_from_slice(&input[first..]);
        self.len += input.len();
        Ok(())
    }
    fn at(&self, n: usize) -> u8 {
        self.bytes[(self.head + n) % self.bytes.len()]
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    pub(crate) fn output_capacity(&self, input: &[u8]) -> usize {
        if self.len != 0 || input.len() < 2 {
            return MAX_BODY + 16;
        }
        let field = u16::from_be_bytes([input[0], input[1]]);
        (field & 0x3fff) as usize + if field >> 14 == 2 { 16 } else { 0 }
    }
    fn frame(&mut self, header: [u8; 2]) -> Result<(usize, usize)> {
        if self.failed {
            return Err(Error::Invalid("decoder is poisoned"));
        }
        let field = u16::from_be_bytes(header);
        let size = (field & 0x3fff) as usize;
        let kind = field >> 14;
        if kind == 1 || kind == 3 || (kind == 2 && self.previous.is_none()) || size < 16 {
            self.failed = true;
            return Err(Error::Invalid("invalid TCP frame"));
        }
        Ok((size, if kind == 2 { 16 } else { 0 }))
    }
    /// Extract one complete frame directly from a borrowed receive span. Only
    /// fragmented frames enter circular storage; the unconsumed suffix stays
    /// with the caller. Prediction context is shared with the buffered path.
    pub fn decode_from(&mut self, input: &mut &[u8], output: &mut Packet) -> Result<bool> {
        if self.failed {
            return Err(Error::Invalid("decoder is poisoned"));
        }
        if self.len != 0 {
            if self.len < 2 {
                let n = (2 - self.len).min(input.len());
                self.feed(&input[..n])?;
                *input = &input[n..];
            }
            if self.len < 2 {
                return Ok(false);
            }
            let (size, _) = self.frame([self.at(0), self.at(1)])?;
            let n = (size + 2).saturating_sub(self.len).min(input.len());
            self.feed(&input[..n])?;
            *input = &input[n..];
            return self.decode_into(output);
        }
        if input.len() < 2 {
            self.feed(input)?;
            *input = &[];
            return Ok(false);
        }
        let (size, prefix) = self.frame([input[0], input[1]])?;
        if input.len() < size + 2 {
            self.feed(input)?;
            *input = &[];
            return Ok(false);
        }
        if output.writable().len() < size + prefix {
            return Err(Error::Full);
        }
        if prefix != 0 {
            output.writable()[..prefix]
                .copy_from_slice(&next_header(self.previous.expect("checked context")));
        }
        output.writable()[prefix..prefix + size].copy_from_slice(&input[2..size + 2]);
        output.set_len(size + prefix)?;
        if let Some(header) = data_header(output.data()) {
            self.previous = Some(header);
        }
        *input = &input[size + 2..];
        Ok(true)
    }
    pub fn decode_into(&mut self, output: &mut Packet) -> Result<bool> {
        if self.failed {
            return Err(Error::Invalid("decoder is poisoned"));
        }
        if self.len < 2 {
            return Ok(false);
        }
        let (size, prefix) = self.frame([self.at(0), self.at(1)])?;
        if self.len < size + 2 {
            return Ok(false);
        }
        if output.writable().len() < size + prefix {
            return Err(Error::Full);
        }
        if prefix != 0 {
            output.writable()[..16]
                .copy_from_slice(&next_header(self.previous.expect("checked context")));
        }
        let start = (self.head + 2) % self.bytes.len();
        let first = size.min(self.bytes.len() - start);
        let destination = &mut output.writable()[prefix..prefix + size];
        destination[..first].copy_from_slice(&self.bytes[start..start + first]);
        destination[first..].copy_from_slice(&self.bytes[..size - first]);
        output.set_len(size + prefix)?;
        if let Some(h) = data_header(output.data()) {
            self.previous = Some(h);
        }
        self.head = (self.head + size + 2) % self.bytes.len();
        self.len -= size + 2;
        Ok(true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketPool;
    fn packet(pool: &mut PacketPool, counter: u64) -> Packet {
        let mut p = pool.take().unwrap();
        p.writable()[..40].fill(0);
        p.writable()[0] = 4;
        p.writable()[4] = 7;
        p.writable()[8..16].copy_from_slice(&counter.to_le_bytes());
        p.set_len(40).unwrap();
        p
    }
    #[test]
    fn published_normal_and_prediction_format_with_fragmented_input() {
        let mut pool = PacketPool::new(3, 200).unwrap();
        let mut tx = Encoder::default();
        let mut rx = Decoder::new(20000).unwrap();
        for n in [0, 1, 5, 6, u64::MAX, 0] {
            let mut p = packet(&mut pool, n);
            let original = p.data().to_vec();
            tx.frame(&mut p).unwrap();
            if n == 1 || n == 6 {
                assert_eq!(&p.data()[..2], &[0x80, 24]);
            }
            for b in p.data() {
                rx.feed(&[*b]).unwrap();
            }
            let mut out = pool.take().unwrap();
            assert!(rx.decode_into(&mut out).unwrap());
            assert_eq!(out.data(), original);
            pool.recycle(out).unwrap();
            pool.recycle(p).unwrap();
        }
    }
    #[test]
    fn rejects_reserved_or_contextless_frames_until_reset() {
        let mut d = Decoder::new(20000).unwrap();
        d.feed(&[0x80, 16]).unwrap();
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut p = pool.take().unwrap();
        assert!(d.decode_into(&mut p).is_err());
        assert!(d.feed(&[0, 0]).is_err());
        d.reset();
        assert!(d.feed(&[0x40, 0]).is_ok());
        assert!(d.decode_into(&mut p).is_err());
    }
    #[test]
    fn reset_discards_prediction_context() {
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut e = Encoder::default();
        let mut p = packet(&mut pool, 8);
        e.frame(&mut p).unwrap();
        pool.recycle(p).unwrap();
        e.reset();
        let mut p = packet(&mut pool, 9);
        e.frame(&mut p).unwrap();
        assert_eq!(&p.data()[..2], &[0, 40]);
    }
    #[test]
    fn direct_and_fragmented_spans_preserve_prediction_and_wraparound() {
        let mut pool = PacketPool::new(2, 200).unwrap();
        let mut encoder = Encoder::default();
        let mut wire = Vec::new();
        let mut expected = Vec::new();
        for counter in [0, 1, 5, 6, u64::MAX, 0] {
            let mut p = packet(&mut pool, counter);
            expected.push(p.data().to_vec());
            encoder.frame(&mut p).unwrap();
            wire.extend_from_slice(p.data());
            pool.recycle(p).unwrap();
        }
        // Every receive-span size exercises a different mixture of complete
        // frames, split headers/bodies and normal/predicted transitions.
        for chunk in 1..=wire.len() {
            let mut decoder = Decoder::new(MAX_BODY + 2).unwrap();
            let mut out = pool.take().unwrap();
            for _ in 0..128 {
                let mut observed = Vec::new();
                for bytes in wire.chunks(chunk) {
                    let mut input = bytes;
                    while !input.is_empty() {
                        if decoder.decode_from(&mut input, &mut out).unwrap() {
                            observed.push(out.data().to_vec());
                        }
                    }
                }
                assert_eq!(observed, expected);
                assert_eq!(decoder.buffered(), 0);
            }
            pool.recycle(out).unwrap();
        }
    }
    #[test]
    fn direct_output_pressure_keeps_input_and_prediction_retryable() {
        let mut pool = PacketPool::new(2, 200).unwrap();
        let mut small = PacketPool::new(1, 16).unwrap();
        let mut encoder = Encoder::default();
        let mut decoder = Decoder::new(20000).unwrap();
        let mut output = pool.take().unwrap();
        let mut short = small.take().unwrap();
        for counter in [0, 1] {
            let mut p = packet(&mut pool, counter);
            let expected = p.data().to_vec();
            encoder.frame(&mut p).unwrap();
            let mut input = p.data();
            assert!(matches!(
                decoder.decode_from(&mut input, &mut short),
                Err(Error::Full)
            ));
            assert_eq!(input, p.data());
            assert!(decoder.decode_from(&mut input, &mut output).unwrap());
            assert_eq!(output.data(), expected);
            assert!(input.is_empty());
            assert_eq!(decoder.buffered(), 0);
            pool.recycle(p).unwrap();
        }
    }
    #[test]
    fn direct_invalid_frames_poison_until_reset() {
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut output = pool.take().unwrap();
        for header in [[0x40, 16], [0xc0, 16], [0x80, 16], [0, 15]] {
            let mut decoder = Decoder::new(20000).unwrap();
            let mut input = header.as_slice();
            assert!(decoder.decode_from(&mut input, &mut output).is_err());
            assert!(decoder.decode_from(&mut &[][..], &mut output).is_err());
            assert!(decoder.feed(&[]).is_err());
            decoder.reset();
            assert!(!decoder.decode_from(&mut &[][..], &mut output).unwrap());
        }
    }
}
