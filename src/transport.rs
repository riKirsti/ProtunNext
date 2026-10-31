//! Shared transport configuration and packet-worker counters.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Counters {
    pub tun_reads: u64,
    pub network_reads: u64,
    pub decrypted_packets: u64,
    pub invalid_packets: u64,
    pub outbound_filtered: u64,
    pub inbound_filtered: u64,
    pub pool_pressure: u64,
    pub polls: u64,
    pub tun_backpressure: u64,
    pub network_backpressure: u64,
}
pub enum Transport {
    Udp,
    Tcp,
    Tls(Box<rustls::ClientConnection>),
    Stealth(crate::stealth::SniStrategy),
}
