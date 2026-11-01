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
    pub io: crate::performance::IoCounters,
    pub to_network_high_water: u64,
    pub to_tun_high_water: u64,
    pub pool_reserved_bytes: u64,
    pub outer_eof: u64,
    pub outer_io_errors: u64,
    pub in_place_encryptions: u64,
    pub in_place_decryptions: u64,
}
pub enum Transport {
    Udp,
    Tcp,
    Tls(Box<rustls::ClientConnection>),
    UnbufferedTls(Box<rustls::client::UnbufferedClientConnection>),
    Stealth(crate::stealth::SniStrategy),
    /// Optional paths without changing existing constructors or config records.
    Tuned(Box<Transport>, crate::performance::Options),
}
impl Transport {
    pub fn with_performance(self, options: crate::performance::Options) -> Self {
        Self::Tuned(Box::new(self), options)
    }
    pub(crate) fn into_options(self) -> crate::Result<(Self, crate::performance::Options)> {
        match self {
            Self::Tuned(inner, options) => {
                options.validate()?;
                if matches!(*inner, Self::Tuned(..)) {
                    return Err(crate::Error::Invalid("nested transport options"));
                }
                Ok((*inner, options))
            }
            inner => Ok((inner, Default::default())),
        }
    }
}
