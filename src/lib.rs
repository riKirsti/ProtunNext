//! Independent packet engine. Control operations do not traverse the packet path.
//! See docs/parity.json for verified support, rather than inferring it from types.
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "local-agent")]
pub mod agent;
pub mod api;
pub mod capture;
pub mod config;
#[cfg(unix)]
pub mod connection;
pub mod control;
pub mod framing;
#[cfg(unix)]
pub mod linux;
#[cfg(unix)]
mod linux_network;
#[cfg(unix)]
pub use linux as unix;
pub mod natpmp;
pub mod packet;
pub mod routing;
pub mod session;
pub mod stealth;
pub mod stream;
pub mod tls;
#[cfg(unix)]
mod tun_device;
pub mod virtual_policy;

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
