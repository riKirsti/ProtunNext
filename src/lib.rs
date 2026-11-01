//! Independent packet engine. Control operations do not traverse the packet path.
//! See docs/parity.json for verified support, rather than inferring it from types.
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
pub mod addressing;
#[cfg(feature = "local-agent")]
pub mod agent;
pub mod api;
pub mod capture;
pub mod config;
#[cfg(any(unix, windows))]
pub mod connection;
pub mod control;
#[cfg(any(unix, windows))]
mod engine;
pub mod framing;
#[cfg(unix)]
pub mod linux;
#[cfg(target_os = "linux")]
mod linux_batch;
#[cfg(unix)]
mod linux_network;
#[cfg(any(unix, windows))]
mod network;
#[cfg(any(unix, windows))]
mod path_probe;
#[cfg(all(target_os = "linux", feature = "io-uring"))]
mod uring_rx;
#[cfg(unix)]
pub use linux as unix;
pub mod natpmp;
pub mod offload;
pub mod packet;
pub mod performance;
#[cfg(any(unix, windows))]
mod platform;
pub mod routing;
pub mod session;
pub mod stealth;
pub mod stream;
pub mod tls;
pub mod tls_unbuffered;
pub mod transport;
#[cfg(unix)]
mod tun_device;
#[cfg(windows)]
#[path = "windows/tun.rs"]
mod tun_device;
#[cfg(target_os = "linux")]
mod tun_offload;
pub mod virtual_policy;
#[cfg(windows)]
pub mod windows;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    Invalid(&'static str),
    #[error("bounded buffer full")]
    Full,
    #[error("feature not implemented and verified: {0}")]
    Unsupported(&'static str),
    #[error("protocol operation failed: {0}")]
    Protocol(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
