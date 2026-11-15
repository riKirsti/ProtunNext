//! Candidate ordering and retry policy; no worker, socket or callback ownership.
use crate::api::types::*;
use std::time::Instant;

#[derive(Clone)]
pub(super) struct Candidate {
    pub peer: PeerInfo,
    pub protocol: Protocol,
    pub port: u16,
}

impl Candidate {
    pub fn info(&self) -> PeerConnectionInfo {
        PeerConnectionInfo {
            peer_id: self.peer.peer_id.clone(),
            entry_ip: self.peer.server_ip,
            protocol: self.protocol,
            port: self.port,
        }
    }
    fn same_tunnel(&self, other: &Self) -> bool {
        self.peer.server_ip == other.peer.server_ip
            && self.peer.server_public_key == other.peer.server_public_key
            && self.protocol == other.protocol
            && self.port == other.port
    }
}

pub(super) struct PeerPlan {
    candidates: Vec<Candidate>,
    next: usize,
    retry_at: Instant,
    attempts: u32,
}
impl PeerPlan {
    pub fn new(now: Instant) -> Self {
        Self {
            candidates: Vec::new(),
            next: 0,
            retry_at: now,
            attempts: 0,
        }
    }
    pub fn rebuild(&mut self, peers: &mut Vec<PeerInfo>) {
        // Lower priority wins; stable ties and IP deduplication keep the first.
        peers.sort_by_key(|peer| peer.priority);
        let mut seen = std::collections::HashSet::new();
        peers.retain(|peer| seen.insert(peer.server_ip));
        for peer in peers.iter_mut() {
            for ports in [
                &mut peer.udp_ports,
                &mut peer.tcp_ports,
                &mut peer.tls_ports,
            ] {
                let mut seen = std::collections::HashSet::new();
                ports.retain(|port| *port != 0 && seen.insert(*port));
            }
        }
        self.candidates.clear();
        self.next = 0;
        for peer in peers {
            for (protocol, ports) in [
                (Protocol::WireguardUdp, &peer.udp_ports),
                (Protocol::WireguardTcp, &peer.tcp_ports),
                (Protocol::Stealth, &peer.tls_ports),
            ] {
                self.candidates.extend(ports.iter().map(|&port| Candidate {
                    peer: peer.clone(),
                    protocol,
                    port,
                }));
            }
        }
    }
    pub fn ready(&self, network_available: bool, now: Instant) -> bool {
        network_available && !self.candidates.is_empty() && now >= self.retry_at
    }
    pub fn next(&mut self) -> Option<Candidate> {
        if self.candidates.is_empty() {
            return None;
        }
        let candidate = self.candidates[self.next % self.candidates.len()].clone();
        self.next = self.next.wrapping_add(1);
        Some(candidate)
    }
    pub fn retained(&self, active: &Candidate) -> Option<Candidate> {
        self.candidates
            .iter()
            .find(|new| new.same_tunnel(active))
            .cloned()
    }
    pub fn retry(&mut self, now: Instant) {
        self.retry_at = now + crate::control::Candidates::retry_delay(self.attempts);
        self.attempts = self.attempts.saturating_add(1);
    }
    pub fn retry_now(&mut self, now: Instant) {
        self.retry_at = now;
    }
    pub fn restart(&mut self, now: Instant) {
        self.retry_now(now);
        self.next = 0;
    }
    pub fn connected(&mut self) {
        self.attempts = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn peer(id: &str, address: &str, priority: i32) -> PeerInfo {
        PeerInfo {
            peer_id: id.into(),
            server_ip: IpAddress(address.parse().unwrap()),
            server_public_key: WgPeerPublicKey([9; 32]),
            priority,
            udp_ports: vec![0, 7, 7],
            tcp_ports: vec![443],
            tls_ports: vec![8443],
            exit_label: None,
            server_name: None,
        }
    }
    #[test]
    fn stable_priority_deduplication_and_protocol_order_survive_rebuild() {
        let mut peers = vec![
            peer("first", "192.0.2.1", 1),
            peer("duplicate", "192.0.2.1", 1),
            peer("preferred", "192.0.2.2", -1),
            peer("tie", "192.0.2.3", 1),
        ];
        let mut plan = PeerPlan::new(Instant::now());
        plan.rebuild(&mut peers);
        assert_eq!(
            peers.iter().map(|p| p.peer_id.as_str()).collect::<Vec<_>>(),
            ["preferred", "first", "tie"]
        );
        for id in ["preferred", "first", "tie"] {
            for (protocol, port) in [
                (Protocol::WireguardUdp, 7),
                (Protocol::WireguardTcp, 443),
                (Protocol::Stealth, 8443),
            ] {
                let candidate = plan.next().unwrap();
                assert_eq!(candidate.peer.peer_id, id);
                assert_eq!((candidate.protocol, candidate.port), (protocol, port));
                assert_eq!(candidate.peer.udp_ports, [7]);
            }
        }
        assert_eq!(plan.next().unwrap().peer.peer_id, "preferred");
    }
    #[test]
    fn retry_backoff_caps_and_connectivity_restart_does_not_reset_failures() {
        let now = Instant::now();
        let mut plan = PeerPlan::new(now);
        assert!(!plan.ready(true, now));
        plan.rebuild(&mut vec![peer("fixture", "192.0.2.1", 0)]);
        assert!(!plan.ready(false, now));
        for ms in [250, 500, 1000, 2000, 4000, 8000, 10000, 10000] {
            plan.retry(now);
            assert!(!plan.ready(true, now + Duration::from_millis(ms - 1)));
            assert!(plan.ready(true, now + Duration::from_millis(ms)));
        }
        plan.next();
        plan.restart(now);
        assert!(plan.ready(true, now));
        assert_eq!(plan.next().unwrap().protocol, Protocol::WireguardUdp);
        plan.retry(now);
        assert!(!plan.ready(true, now + Duration::from_millis(9999)));
        plan.connected();
        plan.retry(now);
        assert!(plan.ready(true, now + Duration::from_millis(250)));
    }
    #[test]
    fn metadata_updates_retain_tunnel_but_changed_keys_and_ports_do_not() {
        let mut peers = vec![peer("original", "192.0.2.1", 0)];
        let mut plan = PeerPlan::new(Instant::now());
        plan.rebuild(&mut peers);
        let active = plan.next().unwrap();
        peers[0].peer_id = "updated".into();
        peers[0].exit_label = Some("fixture".into());
        peers[0].server_name = Some("example.com".into());
        plan.rebuild(&mut peers);
        assert_eq!(plan.retained(&active).unwrap().peer.peer_id, "updated");
        peers[0].server_public_key = WgPeerPublicKey([8; 32]);
        plan.rebuild(&mut peers);
        assert!(plan.retained(&active).is_none());
        peers[0].server_public_key = active.peer.server_public_key.clone();
        peers[0].udp_ports.clear();
        plan.rebuild(&mut peers);
        assert!(plan.retained(&active).is_none());
    }
}
