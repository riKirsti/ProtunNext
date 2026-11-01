//! Route later IPv4 fragments using the first fragment's ownership decision.
//! The pinned upstream uses a ten-second first-fragment verdict cache, not
//! reassembly in its router. Reassembly remains in the destination IP stack.
use super::internal::PacketDisposition;
use std::{collections::HashMap, time::Duration};

const TIMEOUT: Duration = Duration::from_secs(10);
const CAPACITY: usize = 4096;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Key {
    source: [u8; 4],
    destination: [u8; 4],
    protocol: u8,
    id: u16,
}
struct Fragment {
    key: Key,
    first: bool,
    more: bool,
}
fn parse(packet: &[u8]) -> Option<Fragment> {
    if packet.len() < 20 || packet[0] >> 4 != 4 {
        return None;
    }
    let header = usize::from(packet[0] & 15) * 4;
    let length = usize::from(u16::from_be_bytes([packet[2], packet[3]]));
    if header < 20 || length < header || length > packet.len() {
        return None;
    }
    let flags = u16::from_be_bytes([packet[6], packet[7]]);
    if flags & 0x3fff == 0 {
        return None;
    }
    Some(Fragment {
        key: Key {
            source: packet[12..16].try_into().ok()?,
            destination: packet[16..20].try_into().ok()?,
            protocol: packet[9],
            id: u16::from_be_bytes([packet[4], packet[5]]),
        },
        first: flags & 0x1fff == 0,
        more: flags & 0x2000 != 0,
    })
}

#[derive(Default)]
pub(super) struct FragmentRoutes {
    entries: HashMap<Key, (Duration, PacketDisposition)>,
}
impl FragmentRoutes {
    pub(super) fn later(packet: &[u8]) -> bool {
        parse(packet).is_some_and(|f| !f.first)
    }
    pub(super) fn lookup(&mut self, packet: &[u8], now: Duration) -> Option<PacketDisposition> {
        let fragment = parse(packet)?;
        if fragment.first {
            return None;
        }
        let entry = if fragment.more {
            self.entries.get(&fragment.key).copied()
        } else {
            self.entries.remove(&fragment.key)
        }?;
        (now.saturating_sub(entry.0) < TIMEOUT).then_some(entry.1)
    }
    pub(super) fn remember(&mut self, packet: &[u8], now: Duration, owner: PacketDisposition) {
        let Some(fragment) = parse(packet).filter(|f| f.first && f.more) else {
            return;
        };
        self.entries
            .retain(|_, (at, _)| now.saturating_sub(*at) < TIMEOUT);
        if self.entries.len() < CAPACITY || self.entries.contains_key(&fragment.key) {
            self.entries.insert(fragment.key, (now, owner));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn packet(id: u16, protocol: u8, flags: u16) -> [u8; 28] {
        let mut p = [0; 28];
        p[0] = 0x45;
        p[2..4].copy_from_slice(&28u16.to_be_bytes());
        p[4..6].copy_from_slice(&id.to_be_bytes());
        p[6..8].copy_from_slice(&flags.to_be_bytes());
        p[9] = protocol;
        p
    }
    #[test]
    fn expiry_protocol_isolation_and_terminal_fragment_release() {
        let mut routes = FragmentRoutes::default();
        routes.remember(
            &packet(1, 17, 0x2000),
            Duration::ZERO,
            PacketDisposition::Private,
        );
        assert_eq!(routes.lookup(&packet(1, 6, 1), Duration::ZERO), None);
        assert_eq!(
            routes.lookup(&packet(1, 17, 0x2001), Duration::from_secs(9)),
            Some(PacketDisposition::Private)
        );
        assert_eq!(
            routes.lookup(&packet(1, 17, 1), Duration::from_secs(10)),
            None
        );
        routes.remember(
            &packet(2, 17, 0x2000),
            Duration::ZERO,
            PacketDisposition::Customer,
        );
        assert_eq!(
            routes.lookup(&packet(2, 17, 1), Duration::ZERO),
            Some(PacketDisposition::Customer)
        );
        assert_eq!(routes.lookup(&packet(2, 17, 1), Duration::ZERO), None);
        for id in 0..=CAPACITY as u16 {
            routes.remember(
                &packet(id, 17, 0x2000),
                Duration::ZERO,
                PacketDisposition::Private,
            );
        }
        assert_eq!(routes.entries.len(), CAPACITY);
        routes.remember(
            &packet(9000, 17, 0x2000),
            TIMEOUT,
            PacketDisposition::Private,
        );
        assert_eq!(routes.entries.len(), 1);
    }
}
