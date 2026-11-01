//! Explicit, per-connection experimental paths. Defaults preserve the established
//! runtime. Unsupported kernel optimizations fall back before consuming packets.
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum UdpBackend {
    #[default]
    Portable,
    Batch,
    IoUring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Options {
    pub udp_backend: UdpBackend,
    /// Maximum available work per syscall; never wait to fill a batch.
    pub udp_batch_size: u16,
    pub udp_offload: bool,
    pub unbuffered_tls: bool,
    pub compact_pool: bool,
    pub in_place_crypto: bool,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            udp_backend: UdpBackend::Portable,
            udp_batch_size: 16,
            udp_offload: false,
            unbuffered_tls: false,
            compact_pool: false,
            in_place_crypto: false,
        }
    }
}
impl Options {
    pub fn validate(&self) -> Result<()> {
        if self.in_place_crypto && !cfg!(feature = "in-place-crypto") {
            return Err(Error::Unsupported("build requires in-place-crypto feature"));
        }
        if !(1..=32).contains(&self.udp_batch_size) {
            return Err(Error::Invalid("UDP batch size must be 1..=32"));
        }
        if self.udp_offload && self.udp_backend == UdpBackend::Portable {
            return Err(Error::Invalid("UDP offload requires a batched backend"));
        }
        Ok(())
    }
}

/// Packet-worker-local counters: no atomics, callbacks, packet contents or timing
/// on each packet. Published with the existing coalesced statistics snapshot.
#[derive(Debug, Default, Clone, Copy, serde::Serialize)]
pub struct IoCounters {
    pub receive_calls: u64,
    pub send_calls: u64,
    pub received_datagrams: u64,
    pub sent_datagrams: u64,
    pub max_receive_batch: u64,
    pub max_send_batch: u64,
    pub truncated_datagrams: u64,
    pub gso_datagrams: u64,
    pub gro_datagrams: u64,
    pub kernel_fallbacks: u64,
    pub enobufs: u64,
    pub io_uring_completions: u64,
    pub io_uring_submissions: u64,
}
