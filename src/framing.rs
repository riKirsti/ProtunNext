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
    pub fn decode_into(&mut self, output: &mut Packet) -> Result<bool> {
        if self.failed {
            return Err(Error::Invalid("decoder is poisoned"));
        }
        if self.len < 2 {
            return Ok(false);
        }
        let field = u16::from_be_bytes([self.at(0), self.at(1)]);
        let size = (field & 0x3fff) as usize;
        let kind = field >> 14;
        if kind == 1
            || kind == 3
            || (kind == 2 && self.previous.is_none())
            || (kind == 0 && size < 16)
            || (kind == 2 && size < 16)
        {
            self.failed = true;
            return Err(Error::Invalid("invalid TCP frame"));
        }
        if self.len < size + 2 {
            return Ok(false);
        }
        let prefix = if kind == 2 { 16 } else { 0 };
        if output.writable().len() < size + prefix {
            return Err(Error::Full);
        }
        if kind == 2 {
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
}
