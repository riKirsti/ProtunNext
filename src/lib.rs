//! Independent packet engine. Control operations do not traverse the packet path.
//! See docs/parity.json for verified support, rather than inferring it from types.
#[cfg(feature = "local-agent")]
pub mod agent;
pub mod capture;
pub mod config;
pub mod control;
pub mod framing;
#[cfg(target_os = "linux")]
pub mod linux;
pub mod natpmp;
pub mod packet;
pub mod session;
pub mod stream;
pub mod tls;

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
