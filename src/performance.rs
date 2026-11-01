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
    /// Strict opt-in CLI flags; no environment/global state changes.
    pub fn parse(flags: &str) -> Result<Self> {
        let mut options = Self::default();
        let mut backend = false;
        for flag in flags.split(',') {
            match flag {
                "portable" | "batch" | "uring" => {
                    if backend {
                        return Err(Error::Invalid("multiple UDP backends"));
                    }
                    backend = true;
                    options.udp_backend = match flag {
                        "batch" => UdpBackend::Batch,
                        "uring" => UdpBackend::IoUring,
                        _ => UdpBackend::Portable,
                    };
                }
                "offload" => options.udp_offload = true,
                "unbuffered" => options.unbuffered_tls = true,
                "compact" => options.compact_pool = true,
                "in-place" => options.in_place_crypto = true,
                value if value.starts_with("batch-size=") => {
                    options.udp_batch_size = value[11..]
                        .parse()
                        .map_err(|_| Error::Invalid("UDP batch size"))?;
                }
                _ => return Err(Error::Invalid("unknown performance option")),
            }
        }
        if options.udp_offload && !backend {
            options.udp_backend = UdpBackend::Batch;
        }
        options.validate()?;
        Ok(options)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opt_in_flags_are_strict_and_defaults_remain_portable() {
        assert_eq!(Options::default().udp_backend, UdpBackend::Portable);
        let options = Options::parse("offload,compact,unbuffered").unwrap();
        assert_eq!(options.udp_backend, UdpBackend::Batch);
        assert!(options.udp_offload && options.compact_pool && options.unbuffered_tls);
        assert!(Options::parse("portable,offload").is_err());
        assert!(Options::parse("batch,uring").is_err());
        assert!(Options::parse("imaginary-acceleration").is_err());
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
    pub received_bytes: u64,
    pub sent_bytes: u64,
}
