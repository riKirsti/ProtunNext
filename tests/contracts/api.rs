//! Frozen external-consumer contracts for Protun v3.0.5 and existing extensions.
//! These functions are typechecked, never called: no sockets, account login or VPN.
#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use protun_next::api::{
    connection::*,
    logger::{ClientLogger, LogLevel},
};
use std::time::Duration;

fn connection_methods() {
    let _: fn(&Connection, Vec<PeerInfo>) = Connection::update_peers;
    let _: fn(&Connection, ConfigUpdate) = Connection::update;
    let _: fn(&Connection, ConnectivityEvent) = Connection::on_connectivity_change;
    let _: fn(&Connection) = Connection::disconnect;
    let _: fn(&Connection) = Connection::disconnect_and_wait;
    let _: fn(&Connection, PcapFileInfo) = Connection::start_packet_capture;
    let _: fn(&Connection) = Connection::stop_packet_capture;
    let _: fn(&Connection) = Connection::request_stats;
    let _: fn(&Connection) = Connection::debug_log;
    let _: fn(&Connection, LocalAgentSettings) = Connection::update_local_agent_settings;
    let _: fn(&Connection) = Connection::request_local_agent_stats;
    let _: fn(&Connection, ForkSelectorInfo) = Connection::provide_api_fork_selector;
    let _: fn(&Connection) = Connection::invalidate_certificate;
}

#[cfg(unix)]
fn unix_constructor() {
    let _: fn(
        InitialConnectionConfig,
        Option<i32>,
        Box<dyn StateChangedCallback>,
        Box<dyn EventCallback>,
        Option<Box<dyn protun_next::api::connection_unix::SocketProtector>>,
        Box<dyn PersistentCache>,
    ) -> Connection = Connection::unix_connect;
    let _: fn(&Connection, TunStreamInfo) = Connection::update_unix_tun;
}

// Implementing these externally catches required method, argument and return changes.
struct Callbacks;
impl StateChangedCallback for Callbacks {
    fn on_state_changed(&self, _: VpnState) {}
}
impl EventCallback for Callbacks {
    fn on_event(&self, _: Event) {}
}
impl ClientLogger for Callbacks {
    fn log(&self, _: LogLevel, _: String) {}
}
impl PersistentCache for Callbacks {
    fn put(&self, _: CacheKey, _: Vec<u8>) {}
    fn get(&self, _: CacheKey) -> Option<Vec<u8>> {
        None
    }
    fn remove(&self, _: CacheKey) {}
    fn clear_all(&self) {}
}
impl SocketProtector for Callbacks {
    fn protect_socket(&self, _: i32) -> bool {
        true
    }
}

fn compatibility_module_paths(
    _: protun_next::api::state::VpnState,
    _: protun_next::api::events::Event,
    _: protun_next::api::local_agent::LocalAgentSettings,
) {
    let _: fn(LogLevel, Box<dyn ClientLogger>) = protun_next::api::logger::init_logger;
    let _: [u8; CLIENT_PRIV_KEY_SIZE_BYTES] = [0; 32];
    let _: [u8; PEER_PUB_KEY_SIZE_BYTES] = [0; 32];
    let _: fn([u8; 32]) -> [u8; 32] = protun_next::session::Session::public_key;
    let _: protun_next::transport::Transport = protun_next::transport::Transport::Udp;
    let _: fn(&str) -> protun_next::Result<protun_next::tls::ClientConnection> =
        protun_next::tls::client;
}

#[cfg(windows)]
fn windows_contracts() {
    use protun_next::api::windows::connection_windows::*;
    use protun_next::api::windows::protun_error::ProTunFatalError;
    let _: fn(LogLevel, Box<dyn ClientLogger>) -> Result<ProTun, ProTunFatalError> =
        ProTun::initialize;
    let _: fn(&ProTun) = ProTun::delete_routes;
    let _: fn(
        InitialConnectionConfig,
        NetworkConfig,
        Box<dyn StateChangedCallback>,
        Box<dyn EventCallback>,
        Box<dyn PersistentCache>,
    ) -> Result<WindowsConnection, ProTunFatalError> = WindowsConnection::connect;
    let _: fn(&WindowsConnection) -> std::sync::Arc<Connection> = WindowsConnection::get_connection;
    let _: fn(&WindowsConnection) -> ProTunAdapterDetails = WindowsConnection::get_adapter_details;
    let _: fn(&WindowsConnection, bool) -> Result<bool, ProTunFatalError> =
        WindowsConnection::set_ipv6;
    let _: fn(&WindowsConnection, Vec<IpAddress>) -> Result<bool, ProTunFatalError> =
        WindowsConnection::set_dns;
    let _ = NetworkConfig {
        tun_adapter: AdapterConfig {
            custom_dns_server_ips: vec![],
            is_ipv6_enabled: false,
            mtu: 1280,
            buffer_size_bytes: 1048576,
        },
        udp_socket: SocketConfig {
            send_buffer_size_bytes: 0,
            receive_buffer_size_bytes: 0,
        },
    };
}

// Explicit fields and exhaustive matches freeze the reviewed public shapes.
fn variants_snistrategy(value: SniStrategy) {
    match value {
        SniStrategy::Random => {}
        SniStrategy::Top => {}
    }
}

fn record_initialconnectionconfig(value: InitialConnectionConfig) {
    let InitialConnectionConfig {
        peers,
        network_available,
        pcap_file,
        connection_mode,
        sni_strategy,
    } = value;
    let _: Vec<PeerInfo> = peers;
    let _: bool = network_available;
    let _: Option<PcapFileInfo> = pcap_file;
    let _: ConnectionMode = connection_mode;
    let _: SniStrategy = sni_strategy;
}

fn variants_connectionmode(value: ConnectionMode) {
    match value {
        ConnectionMode::NoLocalAgent { wg_private_key } => {
            let _: Option<WgClientPrivateKey> = wg_private_key;
        }
        ConnectionMode::LocalAgent {
            user_agent,
            app_version,
            settings,
            muon_env,
            certificate_duration,
        } => {
            let _: String = user_agent;
            let _: String = app_version;
            let _: LocalAgentSettings = settings;
            let _: MuonEnv = muon_env;
            let _: Option<CertificateDuration> = certificate_duration;
        }
    }
}

fn variants_certificateduration(value: CertificateDuration) {
    match value {
        CertificateDuration::Seconds { count } => {
            let _: u32 = count;
        }
        CertificateDuration::Minutes { count } => {
            let _: u32 = count;
        }
        CertificateDuration::Hours { count } => {
            let _: u32 = count;
        }
        CertificateDuration::Days { count } => {
            let _: u32 = count;
        }
    }
}

fn variants_muonenv(value: MuonEnv) {
    match value {
        MuonEnv::Prod => {}
        MuonEnv::Atlas { scientist } => {
            let _: Option<String> = scientist;
        }
        MuonEnv::CustomServers {
            servers,
            api_pins,
            ar_pins,
        } => {
            let _: Vec<String> = servers;
            let _: Option<Vec<String>> = api_pins;
            let _: Option<Vec<String>> = ar_pins;
        }
    }
}

fn record_cookie(value: Cookie) {
    let Cookie { name, value } = value;
    let _: String = name;
    let _: String = value;
}

fn record_cookies(value: Cookies) {
    let Cookies { items } = value;
    let _: Vec<Cookie> = items;
}

fn record_forkselectorinfo(value: ForkSelectorInfo) {
    let ForkSelectorInfo { selector, cookies } = value;
    let _: String = selector;
    let _: Cookies = cookies;
}

fn variants_connectivityevent(value: ConnectivityEvent) {
    match value {
        ConnectivityEvent::Up => {}
        ConnectivityEvent::Down => {}
        ConnectivityEvent::NetworkSwitch => {}
    }
}

fn variants_cachekey(value: CacheKey) {
    match value {
        CacheKey::Certificate => {}
        CacheKey::PrivateKey => {}
        CacheKey::ApiSession => {}
    }
}

fn record_peerinfo(value: PeerInfo) {
    let PeerInfo {
        peer_id,
        server_ip,
        server_public_key,
        udp_ports,
        tcp_ports,
        tls_ports,
        priority,
        exit_label,
        server_name,
    } = value;
    let _: String = peer_id;
    let _: IpAddress = server_ip;
    let _: WgPeerPublicKey = server_public_key;
    let _: Vec<u16> = udp_ports;
    let _: Vec<u16> = tcp_ports;
    let _: Vec<u16> = tls_ports;
    let _: i32 = priority;
    let _: Option<String> = exit_label;
    let _: Option<String> = server_name;
}

fn record_pcapfileinfo(value: PcapFileInfo) {
    let PcapFileInfo { file, max_bytes } = value;
    let _: PcapFile = file;
    let _: Option<u64> = max_bytes;
}

fn variants_pcapfile(value: PcapFile) {
    match value {
        PcapFile::Path { path, mode } => {
            let _: String = path;
            let _: FileWriteMode = mode;
        }
        PcapFile::Fd(value) => {
            let _: i32 = value;
        }
    }
}

fn variants_filewritemode(value: FileWriteMode) {
    match value {
        FileWriteMode::Append => {}
        FileWriteMode::Overwrite => {}
    }
}

fn variants_tunstreaminfo(value: TunStreamInfo) {
    match value {
        TunStreamInfo::NoTun => {}
        TunStreamInfo::TunFd(value) => {
            let _: i32 = value;
        }
    }
}

fn record_configupdate(value: ConfigUpdate) {
    let ConfigUpdate {
        peers,
        settings,
        tun,
    } = value;
    let _: Option<Vec<PeerInfo>> = peers;
    let _: Option<LocalAgentSettings> = settings;
    let _: Option<TunStreamInfo> = tun;
}

fn record_localagentsettings(value: LocalAgentSettings) {
    let LocalAgentSettings {
        split_tcp,
        netshield_level,
        soft_jail,
        port_forwarding,
        random_nat,
        circumvention_routing,
    } = value;
    let _: Option<bool> = split_tcp;
    let _: Option<NetshieldLevel> = netshield_level;
    let _: Option<bool> = soft_jail;
    let _: Option<bool> = port_forwarding;
    let _: Option<bool> = random_nat;
    let _: Option<bool> = circumvention_routing;
}

fn variants_netshieldlevel(value: NetshieldLevel) {
    match value {
        NetshieldLevel::None => {}
        NetshieldLevel::MalwareFilter => {}
        NetshieldLevel::AdsAndMalwareFilter => {}
        NetshieldLevel::AdultAndAdsAndMalwareFilter => {}
    }
}

fn record_agentconnectioninfo(value: AgentConnectionInfo) {
    let AgentConnectionInfo {
        server_exit_v4,
        server_exit_v6,
        server_mtu,
        user_isp_ip,
        user_isp_country_code,
        user_isp_name,
        user_isp_coordinates,
        restrictions,
        groups,
        settings,
    } = value;
    let _: Option<IpAddress> = server_exit_v4;
    let _: Option<IpAddress> = server_exit_v6;
    let _: Option<u16> = server_mtu;
    let _: Option<String> = user_isp_ip;
    let _: Option<String> = user_isp_country_code;
    let _: Option<String> = user_isp_name;
    let _: Option<Coordinates> = user_isp_coordinates;
    let _: Vec<Restriction> = restrictions;
    let _: Vec<String> = groups;
    let _: LocalAgentSettings = settings;
}

fn record_coordinates(value: Coordinates) {
    let Coordinates {
        latitude,
        longitude,
    } = value;
    let _: f64 = latitude;
    let _: f64 = longitude;
}

fn record_waitjail(value: WaitJail) {
    let WaitJail {
        reason,
        code,
        message,
    } = value;
    let _: WaitJailReason = reason;
    let _: u64 = code;
    let _: String = message;
}

fn variants_waitjailreason(value: WaitJailReason) {
    match value {
        WaitJailReason::BadUserBehavior => {}
        WaitJailReason::DisabledUser => {}
        WaitJailReason::LowPlan => {}
        WaitJailReason::Need2FA => {}
        WaitJailReason::PendingInvoice => {}
        WaitJailReason::SessionOverLimit => {}
        WaitJailReason::WaitingClientChallengeReply => {}
        WaitJailReason::Internal => {}
        WaitJailReason::Other => {}
    }
}

fn variants_restriction(value: Restriction) {
    match value {
        Restriction::Streaming { reason } => {
            let _: String = reason;
        }
        Restriction::Torrent { reason } => {
            let _: String = reason;
        }
        Restriction::Other { name, reason } => {
            let _: String = name;
            let _: String = reason;
        }
    }
}

fn record_vpnstate(value: VpnState) {
    let VpnState {
        interface_state,
        connection_state,
    } = value;
    let _: InterfaceState = interface_state;
    let _: ConnectionState = connection_state;
}

fn variants_interfacestate(value: InterfaceState) {
    match value {
        InterfaceState::Up { error } => {
            let _: Option<InterfaceError> = error;
        }
        InterfaceState::Down { last_error } => {
            let _: Option<InterfaceError> = last_error;
        }
    }
}

fn variants_interfaceerror(value: InterfaceError) {
    match value {
        InterfaceError::IoError { error } => {
            let _: String = error;
        }
    }
}

fn variants_connectionstate(value: ConnectionState) {
    match value {
        ConnectionState::Disconnected { error } => {
            let _: Option<DisconnectReason> = error;
        }
        ConnectionState::Connecting {
            peers,
            wait_reasons,
        } => {
            let _: Vec<PeerConnectionInfo> = peers;
            let _: Vec<PeerConnectionWaitReason> = wait_reasons;
        }
        ConnectionState::ConnectingToLocalAgent { peer, wait_reason } => {
            let _: PeerConnectionInfo = peer;
            let _: Option<AgentConnectionWaitReason> = wait_reason;
        }
        ConnectionState::Connected { peer, agent_info } => {
            let _: PeerConnectionInfo = peer;
            let _: Option<AgentConnectionInfo> = agent_info;
        }
    }
}

fn record_peerconnectioninfo(value: PeerConnectionInfo) {
    let PeerConnectionInfo {
        peer_id,
        entry_ip,
        protocol,
        port,
    } = value;
    let _: String = peer_id;
    let _: IpAddress = entry_ip;
    let _: Protocol = protocol;
    let _: u16 = port;
}

fn variants_protocol(value: Protocol) {
    match value {
        Protocol::WireguardUdp => {}
        Protocol::WireguardTcp => {}
        Protocol::Stealth => {}
    }
}

fn variants_peerconnectionwaitreason(value: PeerConnectionWaitReason) {
    match value {
        PeerConnectionWaitReason::WaitingForNetwork => {}
    }
}

fn variants_agentconnectionwaitreason(value: AgentConnectionWaitReason) {
    match value {
        AgentConnectionWaitReason::SoftJailed => {}
        AgentConnectionWaitReason::HardJailed { jails } => {
            let _: Vec<WaitJail> = jails;
        }
    }
}

fn variants_disconnectreason(value: DisconnectReason) {
    match value {
        DisconnectReason::TunEstablishError { message } => {
            let _: String = message;
        }
    }
}

fn variants_event(value: Event) {
    match value {
        Event::ConnectionStats {
            mono_timestamp_ms,
            received_bytes,
            sent_bytes,
            time_since_last_handshake,
            estimated_loss,
            estimated_round_trip_time,
        } => {
            let _: i64 = mono_timestamp_ms;
            let _: u64 = received_bytes;
            let _: u64 = sent_bytes;
            let _: Duration = time_since_last_handshake;
            let _: f32 = estimated_loss;
            let _: Duration = estimated_round_trip_time;
        }
        Event::PacketCaptureStarted { info } => {
            let _: PcapFileInfo = info;
        }
        Event::PacketCaptureStopped { reason } => {
            let _: CaptureStopReason = reason;
        }
        Event::LocalAgentStats {
            bytes_received,
            bytes_sent,
            malicious_blocked,
            ads_blocked,
            trackers_blocked,
            adult_content_blocked,
            data_saved,
        } => {
            let _: Option<u64> = bytes_received;
            let _: Option<u64> = bytes_sent;
            let _: Option<u64> = malicious_blocked;
            let _: Option<u64> = ads_blocked;
            let _: Option<u64> = trackers_blocked;
            let _: Option<u64> = adult_content_blocked;
            let _: Option<u64> = data_saved;
        }
        Event::LocalAgentError { error } => {
            let _: ErrorEvent = error;
        }
    }
}

fn variants_capturestopreason(value: CaptureStopReason) {
    match value {
        CaptureStopReason::Request { file } => {
            let _: PcapFileInfo = file;
        }
        CaptureStopReason::MaxSizeReached { file } => {
            let _: PcapFileInfo = file;
        }
        CaptureStopReason::Disconnected { file } => {
            let _: PcapFileInfo = file;
        }
        CaptureStopReason::AlreadyStopped => {}
    }
}

fn variants_errorevent(value: ErrorEvent) {
    match value {
        ErrorEvent::ForkSelectorNeeded => {}
        ErrorEvent::ApiError {
            endpoint,
            http_code,
            proton_code,
            message,
            refresh_token_invalid,
        } => {
            let _: ApiEndpoint = endpoint;
            let _: Option<u16> = http_code;
            let _: Option<i64> = proton_code;
            let _: Option<String> = message;
            let _: bool = refresh_token_invalid;
        }
        ErrorEvent::LocalAgentSettingPolicyRefused { setting } => {
            let _: LocalAgentSettingType = setting;
        }
        ErrorEvent::CertificateRefreshFatalError => {}
    }
}

fn variants_apiendpoint(value: ApiEndpoint) {
    match value {
        ApiEndpoint::Auth => {}
        ApiEndpoint::CertificateRefresh => {}
    }
}

fn variants_localagentsettingtype(value: LocalAgentSettingType) {
    match value {
        LocalAgentSettingType::NetshieldLevel => {}
        LocalAgentSettingType::Bouncing => {}
        LocalAgentSettingType::PortForwarding => {}
        LocalAgentSettingType::SplitTcp => {}
        LocalAgentSettingType::SafeMode => {}
        LocalAgentSettingType::RandomNat => {}
    }
}
