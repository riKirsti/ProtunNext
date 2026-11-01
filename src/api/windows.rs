//! v3.0.5 Windows-facing functions backed by the shared independent worker.
use super::{
    logger::{ClientLogger, LogLevel},
    supervisor::Connection,
    types::*,
};
use crate::windows::{Interface, Monitor};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AdapterConfig {
    pub custom_dns_server_ips: Vec<IpAddress>,
    pub is_ipv6_enabled: bool,
    pub mtu: u16,
    pub buffer_size_bytes: u32,
}
#[derive(Clone, Default)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SocketConfig {
    pub send_buffer_size_bytes: u32,
    pub receive_buffer_size_bytes: u32,
}
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct NetworkConfig {
    pub tun_adapter: AdapterConfig,
    pub udp_socket: SocketConfig,
}
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ProTunAdapterDetails {
    pub interface_luid: u64,
    pub interface_index: u32,
    pub client_ipv4_addr: String,
    pub server_ipv4_addr: String,
    pub client_ipv6_addr: String,
    pub server_ipv6_addr: String,
}
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum ProTunFatalError {
    #[error("Local IP address not found: {0}")]
    NoLocalIp(String),
    #[error("Windows HANDLE creation failed: {0}")]
    HandleCreationFailed(String),
    #[error("Winsock startup failed: {0}")]
    WinsockStartFailed(String),
    #[error("Wintun library loading failed: {0}")]
    WintunLibraryLoadingFailed(String),
    #[error("Wintun interface creation failed: {0}")]
    WintunInterfaceCreationFailed(String),
    #[error("Wintun adapter index failed: {0}")]
    WintunAdapterIndexFetchFailed(String),
    #[error("Wintun session creation failed: {0}")]
    WintunSessionCreationFailed(String),
    #[error("Wintun IP address setup failed: {0}")]
    WintunIpAddressSetupFailed(String),
    #[error("Wintun session event creation failed: {0}")]
    WintunSessionHandleCreationFailed(String),
}
impl From<std::io::Error> for ProTunFatalError {
    fn from(error: std::io::Error) -> Self {
        Self::WintunIpAddressSetupFailed(error.to_string())
    }
}
struct Winsock;
impl Drop for Winsock {
    fn drop(&mut self) {
        // SAFETY: balances only our successful WSAStartup reference.
        unsafe {
            windows_sys::Win32::Networking::WinSock::WSACleanup();
        }
    }
}
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct ProTun {
    _winsock: Winsock,
}
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl ProTun {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn initialize(
        log_level: LogLevel,
        logger_callback: Box<dyn ClientLogger>,
    ) -> Result<Self, ProTunFatalError> {
        super::logger::init_logger(log_level, logger_callback);
        // SAFETY: correctly sized initialized WSADATA output for Winsock 2.2.
        let mut data = unsafe { std::mem::zeroed() };
        // SAFETY: valid output object; this call acquires one Winsock reference.
        let result =
            unsafe { windows_sys::Win32::Networking::WinSock::WSAStartup(0x202, &mut data) };
        if result != 0 {
            return Err(ProTunFatalError::WinsockStartFailed(result.to_string()));
        }
        Ok(Self { _winsock: Winsock })
    }
    pub fn delete_routes(&self) {
        crate::windows::delete_owned_routes();
    }
}
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct WindowsConnection {
    monitor: Mutex<Option<Monitor>>,
    connection: Arc<Connection>,
    tun: Arc<Interface>,
}
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl WindowsConnection {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn connect(
        mut connection_config: InitialConnectionConfig,
        network_config: NetworkConfig,
        client_state_change_callback: Box<dyn StateChangedCallback>,
        event_callback: Box<dyn EventCallback>,
        cache: Box<dyn PersistentCache>,
    ) -> Result<Self, ProTunFatalError> {
        let tun = Interface::create(network_config.tun_adapter, network_config.udp_socket)?;
        connection_config.network_available = crate::windows::network_available(tun.session.index);
        let connection = Arc::new(Connection::connect_platform(
            connection_config,
            Ok(Some(tun.clone())),
            client_state_change_callback,
            event_callback,
            None,
            cache,
        ));
        // SAFETY: Value reads the complete integer identity of the valid LUID.
        let luid = unsafe { tun.session.luid.Value };
        let monitor = Monitor::new(connection.clone(), tun.session.index, luid)
            .map_err(|error| ProTunFatalError::HandleCreationFailed(error.to_string()))?;
        Ok(Self {
            monitor: Mutex::new(Some(monitor)),
            connection,
            tun,
        })
    }
    pub fn get_connection(&self) -> Arc<Connection> {
        self.connection.clone()
    }
    pub fn get_adapter_details(&self) -> ProTunAdapterDetails {
        let ipv6 = self.tun.ipv6();
        ProTunAdapterDetails {
            // SAFETY: Value is the complete integer representation of this LUID.
            interface_luid: unsafe { self.tun.session.luid.Value },
            interface_index: self.tun.session.index,
            client_ipv4_addr: "10.2.0.2".into(),
            server_ipv4_addr: "10.2.0.1".into(),
            client_ipv6_addr: if ipv6 { "2a07:b944::2:2" } else { "" }.into(),
            server_ipv6_addr: if ipv6 { "2a07:b944::2:1" } else { "" }.into(),
        }
    }
    pub fn set_ipv6(&self, is_enabled: bool) -> Result<bool, ProTunFatalError> {
        Ok(self.tun.set_ipv6(is_enabled).is_ok())
    }
    pub fn set_dns(&self, custom_dns_server_ips: Vec<IpAddress>) -> Result<bool, ProTunFatalError> {
        Ok(self.tun.set_dns(custom_dns_server_ips).is_ok())
    }
}
impl Drop for WindowsConnection {
    fn drop(&mut self) {
        self.monitor
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
        self.connection.disconnect_and_wait();
    }
}
pub mod connection_windows {
    pub use super::{
        AdapterConfig, NetworkConfig, ProTun, ProTunAdapterDetails, SocketConfig, WindowsConnection,
    };
}
pub mod protun_error {
    pub use super::ProTunFatalError;
}
