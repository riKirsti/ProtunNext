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
    high_water: usize,
}
impl Packet {
    fn allocate(capacity: usize) -> Self {
        Self {
            bytes: vec![0; capacity + HEADROOM].into_boxed_slice(),
            start: HEADROOM,
            end: HEADROOM,
            high_water: HEADROOM,
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
        self.high_water = self.high_water.max(self.end);
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
    pub(crate) fn set_data_len(&mut self, length: usize) -> Result<()> {
        if length > self.bytes.len() - self.start {
            return Err(Error::Full);
        }
        self.end = self.start + length;
        self.high_water = self.high_water.max(self.end);
        Ok(())
    }
    #[cfg(feature = "in-place-crypto")]
    pub(crate) fn wireguard_buffer(&mut self) -> Result<&mut [u8]> {
        if self.start < 16 || self.bytes.len() - self.end < 16 {
            return Err(Error::Full);
        }
        self.start -= 16;
        self.end += 16;
        self.high_water = self.high_water.max(self.end);
        Ok(&mut self.bytes[self.start..])
    }
}
impl Drop for Packet {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

pub struct PacketPool {
    free: Vec<Packet>,
    large: Vec<Packet>,
    capacity: usize,
    maximum: usize,
    limit: usize,
    #[cfg(feature = "debug-counters")]
    recycled_wipe_bytes: u64,
}
impl PacketPool {
    pub fn new(count: usize, capacity: usize) -> Result<Self> {
        if count == 0 || count > 4096 || capacity == 0 || capacity > MAX_IP_PACKET + 32 {
            return Err(Error::Invalid("pool bounds"));
        }
        Ok(Self {
            free: (0..count).map(|_| Packet::allocate(capacity)).collect(),
            large: Vec::new(),
            capacity,
            maximum: capacity,
            limit: count,
            #[cfg(feature = "debug-counters")]
            recycled_wipe_bytes: 0,
        })
    }
    pub fn take(&mut self) -> Option<Packet> {
        self.free.pop().or_else(|| self.large.pop())
    }
    /// Keep ordinary packets small while reserving complete-read scratch space.
    /// The total number of owned buffers remains bounded by `count`.
    pub fn compact(count: usize, capacity: usize, maximum: usize) -> Result<Self> {
        if capacity > maximum || count < 4 {
            return Err(Error::Invalid("compact pool bounds"));
        }
        let mut pool = Self::new(count, capacity)?;
        if maximum > MAX_IP_PACKET + 32 {
            return Err(Error::Invalid("compact packet capacity"));
        }
        pool.maximum = maximum;
        for _ in 0..2 {
            pool.free.pop();
            pool.large.push(Packet::allocate(maximum));
        }
        Ok(pool)
    }
    pub fn take_sized(&mut self, size: usize) -> Option<Packet> {
        if size <= self.capacity {
            return self.take();
        }
        if size > self.maximum {
            return None;
        }
        self.large.pop().or_else(|| {
            // Promotion is only needed when unusually large packets are queued.
            self.free.pop().map(|_| Packet::allocate(self.maximum))
        })
    }
    pub fn recycle(&mut self, mut packet: Packet) -> Result<()> {
        let size = packet.bytes.len() - HEADROOM;
        if (size != self.capacity && size != self.maximum) || self.available() == self.limit {
            return Err(Error::Invalid("foreign or excess packet"));
        }
        // Clear live data before making a plaintext packet reusable.
        packet.bytes[..packet.high_water].zeroize();
        #[cfg(feature = "debug-counters")]
        {
            self.recycled_wipe_bytes = self
                .recycled_wipe_bytes
                .saturating_add(packet.high_water as u64);
        }
        packet.start = HEADROOM;
        packet.end = HEADROOM;
        packet.high_water = HEADROOM;
        if size == self.capacity {
            self.free.push(packet);
        } else {
            self.large.push(packet);
        }
        Ok(())
    }
    pub fn available(&self) -> usize {
        self.free.len() + self.large.len()
    }
    pub fn reserved_bytes(&self) -> usize {
        self.free
            .iter()
            .chain(&self.large)
            .map(|p| p.bytes.len())
            .sum()
    }
    #[cfg(feature = "debug-counters")]
    pub(crate) fn recycled_wipe_bytes(&self) -> u64 {
        self.recycled_wipe_bytes
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
    #[cfg(feature = "debug-counters")]
    pub(crate) fn queued_bytes(&self) -> u64 {
        self.packets.iter().map(|p| p.data().len() as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortened_packet_erases_the_entire_previous_live_region() {
        let mut pool = PacketPool::new(1, 200).unwrap();
        let mut p = pool.take().unwrap();
        p.writable()[..150].fill(0xaa);
        p.set_len(150).unwrap();
        p.set_len(2).unwrap();
        pool.recycle(p).unwrap();
        #[cfg(feature = "debug-counters")]
        assert_eq!(pool.recycled_wipe_bytes(), (HEADROOM + 150) as u64);
        assert!(
            pool.take().unwrap().writable()[..150]
                .iter()
                .all(|b| *b == 0)
        );
    }
    #[test]
    fn compact_pool_keeps_full_packet_reads_and_bounded_ownership() {
        let mut pool = PacketPool::compact(132, 2048, 65535).unwrap();
        assert!(pool.reserved_bytes() < 500_000);
        let mut jumbo = pool.take_sized(65535).unwrap();
        assert_eq!(jumbo.writable().len(), 65535);
        jumbo.writable()[65534] = 7;
        jumbo.set_len(65535).unwrap();
        pool.recycle(jumbo).unwrap();
        assert_eq!(pool.available(), 132);
        assert_eq!(pool.take().unwrap().writable().len(), 2048);
    }
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
        #[cfg(feature = "debug-counters")]
        assert_eq!(q.queued_bytes(), 2);
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
