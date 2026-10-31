use crate::{Error, Result};
use std::collections::VecDeque;
use zeroize::Zeroize;

pub const HEADROOM: usize = 64;
pub const MAX_IP_PACKET: usize = 65_535;

/// Allocation stays attached to the packet as ownership moves between stages.
/// A packet is recycled explicitly after the final consumer acknowledges it.
pub struct Packet {
    bytes: Box<[u8]>,
    start: usize,
    end: usize,
}
impl Packet {
    fn allocate(capacity: usize) -> Self {
        Self {
            bytes: vec![0; capacity + HEADROOM].into_boxed_slice(),
            start: HEADROOM,
            end: HEADROOM,
        }
    }
    pub fn data(&self) -> &[u8] {
        &self.bytes[self.start..self.end]
    }
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.bytes[self.start..self.end]
    }
    pub fn writable(&mut self) -> &mut [u8] {
        &mut self.bytes[HEADROOM..]
    }
    pub fn set_len(&mut self, len: usize) -> Result<()> {
        if len > self.bytes.len() - HEADROOM {
            return Err(Error::Invalid("packet length"));
        }
        self.start = HEADROOM;
        self.end = HEADROOM + len;
        Ok(())
    }
    pub fn prepend(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > self.start {
            return Err(Error::Full);
        }
        self.start -= bytes.len();
        self.bytes[self.start..self.start + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }
    pub fn consume(&mut self, count: usize) -> Result<()> {
        if count > self.end - self.start {
            return Err(Error::Invalid("packet consumption"));
        }
        self.start += count;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
impl Drop for Packet {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

pub struct PacketPool {
    free: Vec<Packet>,
    capacity: usize,
    limit: usize,
}
impl PacketPool {
    pub fn new(count: usize, capacity: usize) -> Result<Self> {
        if count == 0 || count > 4096 || capacity == 0 || capacity > MAX_IP_PACKET + 32 {
            return Err(Error::Invalid("pool bounds"));
        }
        Ok(Self {
            free: (0..count).map(|_| Packet::allocate(capacity)).collect(),
            capacity,
            limit: count,
        })
    }
    pub fn take(&mut self) -> Option<Packet> {
        self.free.pop()
    }
    pub fn recycle(&mut self, mut packet: Packet) -> Result<()> {
        if packet.bytes.len() != self.capacity + HEADROOM || self.free.len() == self.limit {
            return Err(Error::Invalid("foreign or excess packet"));
        }
        // Clear live data before making a plaintext packet reusable.
        packet.bytes[..packet.end].zeroize();
        packet.start = HEADROOM;
        packet.end = HEADROOM;
        self.free.push(packet);
        Ok(())
    }
    pub fn available(&self) -> usize {
        self.free.len()
    }
}

/// Rejects excess ownership rather than silently losing a packet or growing.
pub struct PacketQueue {
    packets: VecDeque<Packet>,
    limit: usize,
}
impl PacketQueue {
    pub fn new(limit: usize) -> Result<Self> {
        if limit == 0 || limit > 4096 {
            return Err(Error::Invalid("queue limit"));
        }
        Ok(Self {
            packets: VecDeque::with_capacity(limit),
            limit,
        })
    }
    pub fn push(&mut self, packet: Packet) -> std::result::Result<(), Packet> {
        if self.packets.len() == self.limit {
            Err(packet)
        } else {
            self.packets.push_back(packet);
            Ok(())
        }
    }
    pub fn front(&self) -> Option<&Packet> {
        self.packets.front()
    }
    pub fn front_mut(&mut self) -> Option<&mut Packet> {
        self.packets.front_mut()
    }
    pub fn pop(&mut self) -> Option<Packet> {
        self.packets.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }
    pub fn len(&self) -> usize {
        self.packets.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &Packet> {
        self.packets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moving_and_partial_writes_preserve_allocation() {
        let mut pool = PacketPool::new(2, 1500).unwrap();
        let mut p = pool.take().unwrap();
        p.writable()[..3].copy_from_slice(b"abc");
        p.set_len(3).unwrap();
        let address = p.data().as_ptr();
        let mut q = PacketQueue::new(1).unwrap();
        assert!(q.push(p).is_ok());
        assert_eq!(q.front().unwrap().data().as_ptr(), address);
        q.front_mut().unwrap().consume(1).unwrap();
        assert_eq!(q.front().unwrap().data(), b"bc");
        pool.recycle(q.pop().unwrap()).unwrap();
        assert_eq!(pool.available(), 2);
    }
    #[test]
    fn bounded_queue_returns_rejected_ownership() {
        let mut pool = PacketPool::new(2, 100).unwrap();
        let mut q = PacketQueue::new(1).unwrap();
        assert!(q.push(pool.take().unwrap()).is_ok());
        let rejected = q.push(pool.take().unwrap()).err().unwrap();
        pool.recycle(rejected).unwrap();
        pool.recycle(q.pop().unwrap()).unwrap();
        assert_eq!(pool.available(), 2);
    }
}
