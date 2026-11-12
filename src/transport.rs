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
    pub tun_write_calls: u64,
    pub tun_written_packets: u64,
    pub tun_coalesced_packets: u64,
    pub tun_max_write_batch: u64,
    pub tun_offload_fallbacks: u64,
    pub network_backpressure: u64,
    pub io: crate::performance::IoCounters,
    /// Queue occupancy in packets, measured before flushing a work batch.
    pub to_network_high_water: u64,
    pub to_tun_high_water: u64,
    pub pool_reserved_bytes: u64,
    pub outer_eof: u64,
    pub outer_io_errors: u64,
    pub in_place_encryptions: u64,
    pub in_place_decryptions: u64,
    /// Byte accounting is populated only by debug-counters builds.
    pub successful_crypto_copy_bytes: u64,
    pub pool_recycle_wipe_bytes: u64,
    pub to_network_bytes_high_water: u64,
    pub to_tun_bytes_high_water: u64,
}
pub enum Transport {
    Udp,
    Tcp,
    Tls(Box<crate::tls::ClientConnection>),
    Stealth(crate::stealth::SniStrategy),
}
