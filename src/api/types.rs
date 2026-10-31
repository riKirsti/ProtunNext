use std::{net::IpAddr, time::Duration};
use zeroize::{Zeroize, ZeroizeOnDrop};
pub const CLIENT_PRIV_KEY_SIZE_BYTES: usize = 32;
pub const PEER_PUB_KEY_SIZE_BYTES: usize = 32;
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct WgClientPrivateKey(pub [u8; 32]);
impl std::fmt::Debug for WgClientPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WgClientPrivateKey([REDACTED])")
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WgPeerPublicKey(pub [u8; 32]);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IpAddress(pub IpAddr);
#[cfg(feature = "uniffi")]
uniffi::custom_type!(WgClientPrivateKey, Vec<u8>);
#[cfg(feature = "uniffi")]
uniffi::custom_type!(WgPeerPublicKey, Vec<u8>);
#[cfg(feature = "uniffi")]
uniffi::custom_type!(IpAddress, String);
impl TryFrom<Vec<u8>> for WgClientPrivateKey {
    type Error = crate::Error;
    fn try_from(mut value: Vec<u8>) -> crate::Result<Self> {
        let key = value
            .as_slice()
            .try_into()
            .map_err(|_| crate::Error::Invalid("private key length"));
        value.zeroize();
        key.map(Self)
    }
}
impl From<WgClientPrivateKey> for Vec<u8> {
    fn from(value: WgClientPrivateKey) -> Self {
        value.0.to_vec()
    }
}
impl TryFrom<Vec<u8>> for WgPeerPublicKey {
    type Error = crate::Error;
    fn try_from(value: Vec<u8>) -> crate::Result<Self> {
        Ok(Self(
            value
                .try_into()
                .map_err(|_| crate::Error::Invalid("peer key length"))?,
        ))
    }
}
impl From<WgPeerPublicKey> for Vec<u8> {
    fn from(value: WgPeerPublicKey) -> Self {
        value.0.to_vec()
    }
}
impl TryFrom<String> for IpAddress {
    type Error = std::net::AddrParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}
impl From<IpAddress> for String {
    fn from(value: IpAddress) -> Self {
        value.0.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum SniStrategy {
    Random,
    Top,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct InitialConnectionConfig {
    pub peers: Vec<PeerInfo>,
    pub network_available: bool,
    pub pcap_file: Option<PcapFileInfo>,
    pub connection_mode: ConnectionMode,
    pub sni_strategy: SniStrategy,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ConnectionMode {
    NoLocalAgent {
        wg_private_key: Option<WgClientPrivateKey>,
    },
    LocalAgent {
        user_agent: String,
        app_version: String,
        settings: LocalAgentSettings,
        muon_env: MuonEnv,
        certificate_duration: Option<CertificateDuration>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CertificateDuration {
    Seconds { count: u32 },
    Minutes { count: u32 },
    Hours { count: u32 },
    Days { count: u32 },
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum MuonEnv {
    Prod,
    Atlas {
        scientist: Option<String>,
    },
    CustomServers {
        servers: Vec<String>,
        api_pins: Option<Vec<String>>,
        ar_pins: Option<Vec<String>>,
    },
}
#[derive(Clone, Zeroize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Cookie {
    pub name: String,
    pub value: String,
}
impl std::fmt::Debug for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cookie")
            .field("name", &self.name)
            .field("value", &"[REDACTED]")
            .finish()
    }
}
#[derive(Debug, Clone, Zeroize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Cookies {
    pub items: Vec<Cookie>,
}
#[derive(Clone, Zeroize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ForkSelectorInfo {
    pub selector: String,
    pub cookies: Cookies,
}
impl std::fmt::Debug for ForkSelectorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ForkSelectorInfo([REDACTED])")
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ConnectivityEvent {
    Up,
    Down,
    NetworkSwitch,
}
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait StateChangedCallback: Send + Sync {
    fn on_state_changed(&self, state: VpnState);
}
impl<F: Fn(VpnState) + Send + Sync> StateChangedCallback for F {
    fn on_state_changed(&self, state: VpnState) {
        self(state)
    }
}
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait EventCallback: Send + Sync {
    fn on_event(&self, event: Event);
}
impl<F: Fn(Event) + Send + Sync> EventCallback for F {
    fn on_event(&self, event: Event) {
        self(event)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CacheKey {
    Certificate,
    PrivateKey,
    ApiSession,
}
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait PersistentCache: Send + Sync {
    fn put(&self, key: CacheKey, bytes: Vec<u8>);
    fn get(&self, key: CacheKey) -> Option<Vec<u8>>;
    fn remove(&self, key: CacheKey);
    fn clear_all(&self);
}
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait SocketProtector: Send + Sync {
    fn protect_socket(&self, socket_fd: i32) -> bool;
}
impl<F: Fn(i32) -> bool + Send + Sync> SocketProtector for F {
    fn protect_socket(&self, fd: i32) -> bool {
        self(fd)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PeerInfo {
    pub peer_id: String,
    pub server_ip: IpAddress,
    pub server_public_key: WgPeerPublicKey,
    pub udp_ports: Vec<u16>,
    pub tcp_ports: Vec<u16>,
    pub tls_ports: Vec<u16>,
    pub priority: i32,
    pub exit_label: Option<String>,
    pub server_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PcapFileInfo {
    pub file: PcapFile,
    pub max_bytes: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum PcapFile {
    Path { path: String, mode: FileWriteMode },
    Fd(i32),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum FileWriteMode {
    Append,
    Overwrite,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum TunStreamInfo {
    NoTun,
    TunFd(i32),
}
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ConfigUpdate {
    pub peers: Option<Vec<PeerInfo>>,
    pub settings: Option<LocalAgentSettings>,
    pub tun: Option<TunStreamInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LocalAgentSettings {
    pub split_tcp: Option<bool>,
    pub netshield_level: Option<NetshieldLevel>,
    pub soft_jail: Option<bool>,
    pub port_forwarding: Option<bool>,
    pub random_nat: Option<bool>,
    pub circumvention_routing: Option<bool>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum NetshieldLevel {
    None,
    MalwareFilter,
    AdsAndMalwareFilter,
    AdultAndAdsAndMalwareFilter,
}
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AgentConnectionInfo {
    pub server_exit_v4: Option<IpAddress>,
    pub server_exit_v6: Option<IpAddress>,
    pub server_mtu: Option<u16>,
    pub user_isp_ip: Option<String>,
    pub user_isp_country_code: Option<String>,
    pub user_isp_name: Option<String>,
    pub user_isp_coordinates: Option<Coordinates>,
    pub restrictions: Vec<Restriction>,
    pub groups: Vec<String>,
    pub settings: LocalAgentSettings,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct WaitJail {
    pub reason: WaitJailReason,
    pub code: u64,
    pub message: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum WaitJailReason {
    BadUserBehavior,
    DisabledUser,
    LowPlan,
    Need2FA,
    PendingInvoice,
    SessionOverLimit,
    WaitingClientChallengeReply,
    Internal,
    Other,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Restriction {
    Streaming { reason: String },
    Torrent { reason: String },
    Other { name: String, reason: String },
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct VpnState {
    pub interface_state: InterfaceState,
    pub connection_state: ConnectionState,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum InterfaceState {
    Up { error: Option<InterfaceError> },
    Down { last_error: Option<InterfaceError> },
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum InterfaceError {
    IoError { error: String },
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ConnectionState {
    Disconnected {
        error: Option<DisconnectReason>,
    },
    Connecting {
        peers: Vec<PeerConnectionInfo>,
        wait_reasons: Vec<PeerConnectionWaitReason>,
    },
    ConnectingToLocalAgent {
        peer: PeerConnectionInfo,
        wait_reason: Option<AgentConnectionWaitReason>,
    },
    Connected {
        peer: PeerConnectionInfo,
        agent_info: Option<AgentConnectionInfo>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PeerConnectionInfo {
    pub peer_id: String,
    pub entry_ip: IpAddress,
    pub protocol: Protocol,
    pub port: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Protocol {
    WireguardUdp,
    WireguardTcp,
    Stealth,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum PeerConnectionWaitReason {
    WaitingForNetwork,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum AgentConnectionWaitReason {
    SoftJailed,
    HardJailed { jails: Vec<WaitJail> },
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum DisconnectReason {
    TunEstablishError { message: String },
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Event {
    ConnectionStats {
        mono_timestamp_ms: i64,
        received_bytes: u64,
        sent_bytes: u64,
        time_since_last_handshake: Duration,
        estimated_loss: f32,
        estimated_round_trip_time: Duration,
    },
    PacketCaptureStarted {
        info: PcapFileInfo,
    },
    PacketCaptureStopped {
        reason: CaptureStopReason,
    },
    LocalAgentStats {
        bytes_received: Option<u64>,
        bytes_sent: Option<u64>,
        malicious_blocked: Option<u64>,
        ads_blocked: Option<u64>,
        trackers_blocked: Option<u64>,
        adult_content_blocked: Option<u64>,
        data_saved: Option<u64>,
    },
    LocalAgentError {
        error: ErrorEvent,
    },
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CaptureStopReason {
    Request { file: PcapFileInfo },
    MaxSizeReached { file: PcapFileInfo },
    Disconnected { file: PcapFileInfo },
    AlreadyStopped,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ErrorEvent {
    ForkSelectorNeeded,
    ApiError {
        endpoint: ApiEndpoint,
        http_code: Option<u16>,
        proton_code: Option<i64>,
        message: Option<String>,
        refresh_token_invalid: bool,
    },
    LocalAgentSettingPolicyRefused {
        setting: LocalAgentSettingType,
    },
    CertificateRefreshFatalError,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ApiEndpoint {
    Auth,
    CertificateRefresh,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum LocalAgentSettingType {
    NetshieldLevel,
    Bouncing,
    PortForwarding,
    SplitTcp,
    SafeMode,
    RandomNat,
}

impl From<&LocalAgentSettings> for crate::control::AgentSettings {
    fn from(s: &LocalAgentSettings) -> Self {
        Self {
            split_tcp: s.split_tcp,
            netshield: s.netshield_level.map(|n| match n {
                NetshieldLevel::None => crate::control::Netshield::Off,
                NetshieldLevel::MalwareFilter => crate::control::Netshield::Malware,
                NetshieldLevel::AdsAndMalwareFilter => crate::control::Netshield::AdsAndMalware,
                NetshieldLevel::AdultAndAdsAndMalwareFilter => {
                    crate::control::Netshield::AdultAdsAndMalware
                }
            }),
            soft_jail: s.soft_jail,
            port_forwarding: s.port_forwarding,
            random_nat: s.random_nat,
            circumvention_routing: s.circumvention_routing,
        }
    }
}
