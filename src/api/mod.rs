//! v3.0.5-facing contracts. Protocol work stays in the independent packet engine.
//! API names/field shapes are compatibility requirements from Proton's GPL-3.0
//! source at a498c38ae6ffb0e68e7190b2831ef985d73fa6da, not imported execution code.
#[cfg(feature = "local-agent")]
pub mod account;
#[cfg(feature = "local-agent")]
pub(crate) mod account_material;
#[cfg(feature = "local-agent")]
pub(crate) mod account_os;
#[cfg(feature = "local-agent")]
pub mod agent_state;
pub mod cache;
pub mod capture;
#[cfg(feature = "local-agent")]
pub mod internal;
pub mod keys;
pub mod logger;
#[cfg(target_os = "linux")]
pub(crate) mod supervisor;
pub mod types;
pub mod connection {
    #[cfg(target_os = "linux")]
    pub use super::supervisor::Connection;
    pub use super::types::*;
}
#[cfg(unix)]
pub mod connection_unix {
    pub use super::types::SocketProtector;
}
pub mod state {
    pub use super::types::{
        AgentConnectionWaitReason, ConnectionState, DisconnectReason, InterfaceError,
        InterfaceState, PeerConnectionInfo, PeerConnectionWaitReason, Protocol, VpnState,
    };
}
pub mod events {
    pub use super::types::{
        ApiEndpoint, CaptureStopReason, ErrorEvent, Event, LocalAgentSettingType,
    };
}
pub mod local_agent {
    pub use super::types::{
        AgentConnectionInfo, Coordinates, LocalAgentSettings, NetshieldLevel, Restriction,
        WaitJail, WaitJailReason,
    };
}
