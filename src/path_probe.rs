//! Allocation-free data-path verification. Timings, the 500 unanswered-packet
//! trigger and wire marker come from published pvpnclient 4.4.1 udp.rs/main.rs.
use std::time::Duration;
pub(crate) const PAYLOAD: &[u8] = b"libpvpnclient data probe";
pub(crate) const SIZE: usize = 28 + PAYLOAD.len();
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Action {
    None,
    Send,
    Failed,
}
pub(crate) struct PathProbe {
    addresses: crate::addressing::Addresses,
    active: bool,
    verified: bool,
    next: Duration,
    attempt: usize,
    unanswered: u32,
    requested: bool,
    identifier: u16,
}
impl PathProbe {
    #[cfg(test)]
    pub(crate) fn new(now: Duration) -> crate::Result<Self> {
        Self::with_addresses(now, Default::default())
    }
    pub(crate) fn with_addresses(
        now: Duration,
        addresses: crate::addressing::Addresses,
    ) -> crate::Result<Self> {
        let mut id = [0; 2];
        getrandom::fill(&mut id).map_err(|e| crate::Error::Protocol(e.to_string()))?;
        Ok(Self {
            addresses,
            active: true,
            verified: false,
            next: now,
            attempt: 0,
            unanswered: 0,
            requested: false,
            identifier: u16::from_ne_bytes(id),
        })
    }
    pub(crate) fn sent(&mut self) {
        self.unanswered = self.unanswered.saturating_add(1);
        if self.unanswered >= 500 {
            self.requested = true;
            self.unanswered = 0;
        }
    }
    pub(crate) fn received(&mut self) -> bool {
        let first = !self.verified;
        self.verified = true;
        self.active = false;
        self.requested = false;
        self.unanswered = 0;
        first
    }
    pub(crate) fn retry_handshake(&self) -> bool {
        self.attempt > 1
    }
    pub(crate) fn action(&mut self, now: Duration) -> Action {
        if !self.active && self.requested {
            self.active = true;
            self.attempt = 0;
            self.next = now;
            self.requested = false;
        }
        if !self.active || now < self.next {
            return Action::None;
        }
        let delays = [500, 1000, 2000, 4000];
        let Some(delay) = delays.get(self.attempt) else {
            return Action::Failed;
        };
        self.attempt += 1;
        self.next = now + Duration::from_millis(*delay);
        Action::Send
    }
    pub(crate) fn delay(&self, now: Duration) -> Option<Duration> {
        if self.active {
            Some(self.next.saturating_sub(now))
        } else if self.requested {
            Some(Duration::ZERO)
        } else {
            None
        }
    }
    pub(crate) fn packet(&self) -> [u8; SIZE] {
        let mut p = [0; SIZE];
        p[0] = 0x45;
        p[2..4].copy_from_slice(&(SIZE as u16).to_be_bytes());
        p[8] = 64;
        p[9] = 1;
        p[12..16].copy_from_slice(&self.addresses.v4(2).octets());
        p[16..20].copy_from_slice(&self.addresses.v4(1).octets());
        p[20] = 8;
        p[24..26].copy_from_slice(&self.identifier.to_be_bytes());
        p[28..].copy_from_slice(PAYLOAD);
        let ip = checksum(&p[..20]);
        p[10..12].copy_from_slice(&ip.to_be_bytes());
        let icmp = checksum(&p[20..]);
        p[22..24].copy_from_slice(&icmp.to_be_bytes());
        p
    }
    pub(crate) fn is_reply(packet: &[u8]) -> bool {
        if packet.len() < 28 || packet[0] >> 4 != 4 || packet[9] != 1 {
            return false;
        }
        let offset = (packet[0] & 15) as usize * 4;
        offset >= 20 && packet.get(offset) == Some(&0) && packet.get(offset + 8..) == Some(PAYLOAD)
    }
}
fn checksum(bytes: &[u8]) -> u16 {
    let (pairs, tail) = bytes.as_chunks::<2>();
    let mut sum: u32 = pairs.iter().map(|p| u16::from_be_bytes(*p) as u32).sum();
    if let Some(byte) = tail.first() {
        sum += (*byte as u32) << 8;
    }
    while sum > 65535 {
        sum = (sum & 65535) + (sum >> 16);
    }
    !(sum as u16)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retry_budget_is_7500ms_and_only_decrypted_data_resets_it() {
        let mut probe = PathProbe::new(Duration::ZERO).unwrap();
        for time in [0, 500, 1500, 3500] {
            assert_eq!(probe.action(Duration::from_millis(time)), Action::Send);
        }
        assert_eq!(probe.action(Duration::from_millis(7499)), Action::None);
        assert_eq!(probe.action(Duration::from_millis(7500)), Action::Failed);
        probe.received();
        assert_eq!(probe.action(Duration::from_secs(100)), Action::None);
        for _ in 0..499 {
            probe.sent();
        }
        assert_eq!(probe.action(Duration::from_secs(101)), Action::None);
        probe.sent();
        assert_eq!(probe.action(Duration::from_secs(101)), Action::Send);
        probe.received();
        assert_eq!(probe.delay(Duration::from_secs(102)), None);
    }
    #[test]
    fn probe_is_valid_ip_and_icmp_and_replies_are_recognized() {
        let probe = PathProbe::new(Duration::ZERO).unwrap();
        let mut p = probe.packet();
        assert_eq!(checksum(&p[..20]), 0);
        assert_eq!(checksum(&p[20..]), 0);
        assert!(!PathProbe::is_reply(&p));
        p[20] = 0;
        assert!(PathProbe::is_reply(&p));
        p[28] ^= 1;
        assert!(!PathProbe::is_reply(&p));
    }
    #[test]
    fn alternate_subnet_probe_has_selected_addresses_and_valid_checksums() {
        let addresses = crate::addressing::Addresses::new(37).unwrap();
        let p = PathProbe::with_addresses(Duration::ZERO, addresses)
            .unwrap()
            .packet();
        assert_eq!(&p[12..16], &[10, 37, 0, 2]);
        assert_eq!(&p[16..20], &[10, 37, 0, 1]);
        assert_eq!(checksum(&p[..20]), 0);
        assert_eq!(checksum(&p[20..]), 0);
    }
}
