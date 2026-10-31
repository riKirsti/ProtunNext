//! Control-plane data contracts are not evidence of a working server integration.
use crate::{
    Error, Result,
    config::{Peer, Protocol},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, net::SocketAddr, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Netshield {
    Off,
    Malware,
    AdsAndMalware,
    AdultAdsAndMalware,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentSettings {
    pub split_tcp: Option<bool>,
    pub netshield: Option<Netshield>,
    pub soft_jail: Option<bool>,
    pub port_forwarding: Option<bool>,
    pub random_nat: Option<bool>,
    pub circumvention_routing: Option<bool>,
}
impl AgentSettings {
    pub fn validate(&self) -> Result<()> {
        if self.port_forwarding == Some(true) && self.random_nat == Some(false) {
            return Err(Error::Invalid(
                "port forwarding conflicts with moderate NAT",
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub peer_id: String,
    pub address: SocketAddr,
    pub protocol: Protocol,
    pub peer_key: [u8; 32],
}
/// Candidate selection only. It never emits Connected or claims agent policy applied.
pub struct Candidates {
    endpoints: Vec<Endpoint>,
    next: usize,
    available: bool,
}
impl Candidates {
    pub fn new(mut peers: Vec<Peer>) -> Result<Self> {
        if peers.is_empty() || peers.len() > 256 {
            return Err(Error::Invalid("candidate peer count"));
        }
        let mut ids = BTreeSet::new();
        for p in &peers {
            p.validate()?;
            if !ids.insert(&p.id) {
                return Err(Error::Invalid("duplicate peer ID"));
            }
        }
        peers.sort_by_key(|p| p.priority);
        let mut endpoints = Vec::new();
        for p in peers {
            for protocol in [
                Protocol::WireguardUdp,
                Protocol::WireguardTcp,
                Protocol::Stealth,
            ] {
                for address in p.endpoints(protocol) {
                    endpoints.push(Endpoint {
                        peer_id: p.id.clone(),
                        address,
                        protocol,
                        peer_key: p.public_key,
                    });
                }
            }
        }
        if endpoints.is_empty() {
            return Err(Error::Invalid("no endpoint"));
        }
        Ok(Self {
            endpoints,
            next: 0,
            available: true,
        })
    }
    pub fn network(&mut self, available: bool) {
        self.available = available;
        self.next = 0;
    }
    pub fn next_endpoint(&mut self) -> Option<&Endpoint> {
        if !self.available {
            return None;
        }
        let n = self.next;
        self.next = (self.next + 1) % self.endpoints.len();
        self.endpoints.get(n)
    }
    pub fn retry_delay(attempt: u32) -> Duration {
        Duration::from_millis(250u64.saturating_mul(1u64 << attempt.min(6)).min(10_000))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Support {
    ImplementedLocal,
    NotImplemented,
    PartiallyImplemented,
    NeedsLiveVerification,
}
#[derive(Debug, Deserialize)]
pub struct ParityItem {
    pub id: String,
    pub requirement: String,
    pub source: String,
    pub status: Support,
    pub evidence: Vec<String>,
}
pub fn parity_manifest() -> Result<Vec<ParityItem>> {
    serde_json::from_str(include_str!("../docs/parity.json"))
        .map_err(|e| Error::Protocol(e.to_string()))
}
/// Release gate fails unless every required feature has verified implementation evidence.
pub fn release_gate() -> Result<()> {
    let items = parity_manifest()?;
    if items.is_empty() {
        return Err(Error::Invalid("empty parity requirements"));
    }
    if items
        .iter()
        .any(|i| i.status != Support::ImplementedLocal || i.evidence.is_empty())
    {
        return Err(Error::Unsupported("v3.0.5 feature parity is incomplete"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cannot_claim_parity_with_missing_server_and_platform_work() {
        assert!(release_gate().is_err());
    }
    #[test]
    fn network_down_prevents_selection_and_backoff_is_bounded() {
        let mut c = Candidates::new(vec![Peer {
            id: "peer".into(),
            address: "127.0.0.1".parse().unwrap(),
            public_key: [1; 32],
            udp_ports: vec![1],
            tcp_ports: vec![2],
            tls_ports: vec![3],
            priority: 0,
            exit_label: None,
            server_name: None,
        }])
        .unwrap();
        assert_eq!(c.next_endpoint().unwrap().protocol, Protocol::WireguardUdp);
        c.network(false);
        assert!(c.next_endpoint().is_none());
        c.network(true);
        assert_eq!(c.next_endpoint().unwrap().protocol, Protocol::WireguardUdp);
        assert_eq!(Candidates::retry_delay(u32::MAX), Duration::from_secs(10));
    }
}
