//! Control commands execute on a supervisor thread, never in the packet loop.
use super::{keys::Identity, types::*};
use crate::{
    Result,
    config::{PrivateKey, WireguardConfig},
    connection::{CaptureState, Connection as Worker, State as WorkerState},
    linux::Transport,
};
use std::{
    fs::File,
    os::fd::AsFd,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use zeroize::Zeroizing;

pub(crate) enum Command {
    Update(ConfigUpdate),
    Connectivity(ConnectivityEvent),
    Disconnect,
    Capture(PcapFileInfo),
    StopCapture,
    Stats,
    AgentStats,
    Fork(Zeroizing<ForkSelectorInfo>),
    InvalidateCertificate,
    Debug,
}
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Connection {
    sender: mpsc::Sender<Command>,
    join: Mutex<Option<JoinHandle<()>>>,
}
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl Connection {
    /// v3.0.5 constructor transfers ownership of the supplied raw descriptor.
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn unix_connect(
        config: InitialConnectionConfig,
        tun_fd: Option<i32>,
        state_change_callback: Box<dyn StateChangedCallback>,
        event_callback: Box<dyn EventCallback>,
        socket_protector: Option<Box<dyn SocketProtector>>,
        cache: Box<dyn PersistentCache>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let join = thread::Builder::new()
            .name("protun-next-control".into())
            .spawn(move || {
                let owned_tun = tun_fd.map(take_descriptor).transpose();
                let mut actor = Supervisor {
                    config,
                    tun: None,
                    protector: socket_protector.map(Arc::from),
                    cache: Arc::from(cache),
                    state_callback: state_change_callback,
                    event_callback,
                    state: None,
                    worker: None,
                    plan: Vec::new(),
                    active: None,
                    next: 0,
                    retry_at: Instant::now(),
                    attempt_at: Instant::now(),
                    attempts: 0,
                    requested_stats: None,
                    key: Zeroizing::new([0; 32]),
                    capture: Arc::new(CaptureState::default()),
                    #[cfg(feature = "local-agent")]
                    account: None,
                    #[cfg(feature = "local-agent")]
                    agent_state: Default::default(),
                    #[cfg(feature = "local-agent")]
                    pending_fork: None,
                    fatal: false,
                };
                match owned_tun {
                    Ok(tun) => actor.tun = tun,
                    Err(error) => {
                        actor.disconnected(Some(error.to_string()));
                        return;
                    }
                }
                if let Err(error) = actor.initialize() {
                    actor.disconnected(Some(error.to_string()));
                    return;
                }
                actor.run(receiver);
            })
            .expect("control thread creation");
        Self {
            sender,
            join: Mutex::new(Some(join)),
        }
    }
    pub fn update_peers(&self, peers: Vec<PeerInfo>) {
        self.update(ConfigUpdate {
            peers: Some(peers),
            ..Default::default()
        });
    }
    pub fn update(&self, update: ConfigUpdate) {
        self.send(Command::Update(update));
    }
    pub fn on_connectivity_change(&self, event: ConnectivityEvent) {
        self.send(Command::Connectivity(event));
    }
    pub fn disconnect(&self) {
        self.send(Command::Disconnect);
    }
    pub fn disconnect_and_wait(&self) {
        self.disconnect();
        if let Some(join) = self.join.lock().unwrap_or_else(|e| e.into_inner()).take()
            && join.thread().id() != thread::current().id()
        {
            let _ = join.join();
        }
    }
    pub fn update_unix_tun(&self, tun_fd: TunStreamInfo) {
        self.update(ConfigUpdate {
            tun: Some(tun_fd),
            ..Default::default()
        });
    }
    pub fn start_packet_capture(&self, pcap_file: PcapFileInfo) {
        self.send(Command::Capture(pcap_file));
    }
    pub fn stop_packet_capture(&self) {
        self.send(Command::StopCapture);
    }
    pub fn request_stats(&self) {
        self.send(Command::Stats);
    }
    pub fn debug_log(&self) {
        self.send(Command::Debug);
    }
    pub fn update_local_agent_settings(&self, settings: LocalAgentSettings) {
        self.update(ConfigUpdate {
            settings: Some(settings),
            ..Default::default()
        });
    }
    pub fn request_local_agent_stats(&self) {
        self.send(Command::AgentStats);
    }
    pub fn provide_api_fork_selector(&self, fork_selector_info: ForkSelectorInfo) {
        self.send(Command::Fork(Zeroizing::new(fork_selector_info)));
    }
    pub fn invalidate_certificate(&self) {
        self.send(Command::InvalidateCertificate);
    }
}
impl Connection {
    fn send(&self, command: Command) {
        let _ = self.sender.send(command);
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.disconnect_and_wait();
    }
}
fn take_descriptor(fd: i32) -> Result<File> {
    // Validate before changing any supervisor state, including atomic updates.
    let owned = crate::linux::duplicate_nonblocking(fd)?;
    // SAFETY: legacy constructor transfers the caller's original descriptor.
    unsafe {
        libc::close(fd);
    }
    Ok(owned)
}
#[derive(Clone)]
struct Candidate {
    peer: PeerInfo,
    protocol: Protocol,
    port: u16,
}
impl Candidate {
    fn info(&self) -> PeerConnectionInfo {
        PeerConnectionInfo {
            peer_id: self.peer.peer_id.clone(),
            entry_ip: self.peer.server_ip,
            protocol: self.protocol,
            port: self.port,
        }
    }
    fn same_tunnel(&self, other: &Self) -> bool {
        self.peer.server_ip == other.peer.server_ip
            && self.peer.server_public_key == other.peer.server_public_key
            && self.protocol == other.protocol
            && self.port == other.port
    }
}
pub(crate) fn sanitize(mut peers: Vec<PeerInfo>) -> Vec<PeerInfo> {
    // Lower numbers win, matching the v3.0.5 sanitizer; stable ties keep first.
    peers.sort_by_key(|p| p.priority);
    let mut seen = std::collections::HashSet::new();
    peers.retain(|p| seen.insert(p.server_ip));
    for peer in &mut peers {
        for ports in [
            &mut peer.udp_ports,
            &mut peer.tcp_ports,
            &mut peer.tls_ports,
        ] {
            ports.retain(|p| *p != 0);
            let mut seen = std::collections::HashSet::new();
            ports.retain(|p| seen.insert(*p));
        }
    }
    peers
}
struct Supervisor {
    config: InitialConnectionConfig,
    tun: Option<File>,
    protector: Option<Arc<dyn SocketProtector>>,
    cache: Arc<dyn PersistentCache>,
    state_callback: Box<dyn StateChangedCallback>,
    event_callback: Box<dyn EventCallback>,
    state: Option<VpnState>,
    worker: Option<Worker>,
    plan: Vec<Candidate>,
    active: Option<Candidate>,
    next: usize,
    retry_at: Instant,
    attempt_at: Instant,
    attempts: u32,
    requested_stats: Option<u64>,
    key: Zeroizing<[u8; 32]>,
    capture: Arc<CaptureState>,
    #[cfg(feature = "local-agent")]
    account: Option<super::account::Service>,
    #[cfg(feature = "local-agent")]
    agent_state: super::agent_state::AgentState,
    #[cfg(feature = "local-agent")]
    pending_fork: Option<Zeroizing<ForkSelectorInfo>>,
    fatal: bool,
}
impl Supervisor {
    fn initialize(&mut self) -> Result<()> {
        #[cfg(not(feature = "local-agent"))]
        if matches!(
            self.config.connection_mode,
            ConnectionMode::LocalAgent { .. }
        ) {
            return Err(crate::Error::Invalid("build requires local-agent feature"));
        }
        self.key = Zeroizing::new(match &self.config.connection_mode {
            ConnectionMode::NoLocalAgent {
                wg_private_key: Some(key),
            } => key.0,
            _ => Identity::load_or_generate(self.cache.as_ref())?.wireguard_key(),
        });
        if let Some(info) = self.config.pcap_file.clone() {
            self.capture.start(info)?;
        }
        self.rebuild();
        Ok(())
    }
    fn rebuild(&mut self) {
        self.config.peers = sanitize(std::mem::take(&mut self.config.peers));
        self.plan.clear();
        self.next = 0;
        for peer in &self.config.peers {
            for (protocol, ports) in [
                (Protocol::WireguardUdp, &peer.udp_ports),
                (Protocol::WireguardTcp, &peer.tcp_ports),
                (Protocol::Stealth, &peer.tls_ports),
            ] {
                for &port in ports {
                    self.plan.push(Candidate {
                        peer: peer.clone(),
                        protocol,
                        port,
                    });
                }
            }
        }
    }
    fn publish(&mut self, connection_state: ConnectionState) {
        let interface_state = match self.worker.as_ref().map(Worker::snapshot) {
            Some(snapshot) if snapshot.tun_present => InterfaceState::Up {
                error: snapshot
                    .interface_error
                    .map(|error| InterfaceError::IoError { error }),
            },
            Some(snapshot) => InterfaceState::Down {
                last_error: snapshot
                    .interface_error
                    .map(|error| InterfaceError::IoError { error }),
            },
            None if self.tun.is_some() => InterfaceState::Up { error: None },
            _ => InterfaceState::Down { last_error: None },
        };
        let state = VpnState {
            interface_state,
            connection_state,
        };
        if self.state.as_ref() != Some(&state) {
            self.state = Some(state.clone());
            self.state_callback.on_state_changed(state);
        }
    }
    fn disconnected(&mut self, error: Option<String>) {
        self.publish(ConnectionState::Disconnected {
            error: error.map(|message| DisconnectReason::TunEstablishError { message }),
        });
    }
    fn stop_worker(&mut self) {
        #[cfg(feature = "local-agent")]
        {
            self.account = None;
            self.agent_state = Default::default();
        }
        if let Some(mut worker) = self.worker.take() {
            let _ = worker.disconnect_and_wait();
            for event in worker.take_events() {
                self.event_callback.on_event(event);
            }
        }
        self.active = None;
        self.requested_stats = None;
    }
    fn start_next(&mut self) -> Result<()> {
        if self.plan.is_empty() {
            return Ok(());
        }
        let candidate = self.plan[self.next % self.plan.len()].clone();
        self.next = self.next.wrapping_add(1);
        let transport = match candidate.protocol {
            Protocol::WireguardUdp => Transport::Udp,
            Protocol::WireguardTcp => Transport::Tcp,
            Protocol::Stealth => Transport::Stealth(match self.config.sni_strategy {
                SniStrategy::Random => crate::stealth::SniStrategy::Random,
                SniStrategy::Top => crate::stealth::SniStrategy::Top,
            }),
        };
        let config = WireguardConfig {
            private_key: PrivateKey::from_bytes(*self.key)?,
            peer_key: candidate.peer.server_public_key.0,
            endpoint: std::net::SocketAddr::new(candidate.peer.server_ip.0, candidate.port),
            addresses: vec!["10.2.0.2/32".into()],
            allowed_ips: vec!["0.0.0.0/0".into(), "::/0".into()],
            dns: Vec::new(),
            mtu: 1420,
            keepalive: Some(25),
        };
        let worker = Worker::start_with_capture(
            self.tun.as_ref().map(AsFd::as_fd),
            config,
            String::new(),
            transport,
            self.protector.clone(),
            Some(self.capture.clone()),
        )?;
        self.worker = Some(worker);
        self.active = Some(candidate);
        self.attempt_at = Instant::now();
        Ok(())
    }
    fn run(&mut self, receiver: mpsc::Receiver<Command>) {
        loop {
            match receiver.recv_timeout(Duration::from_millis(20)) {
                Ok(Command::Disconnect) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Ok(command) => {
                    if let Err(error) = self.command(command) {
                        log::error!("control request failed: {error}");
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            if !self.config.network_available {
                self.publish(ConnectionState::Connecting {
                    peers: Vec::new(),
                    wait_reasons: vec![PeerConnectionWaitReason::WaitingForNetwork],
                });
                continue;
            }
            if self.fatal {
                break;
            }
            if self.worker.is_none()
                && Instant::now() >= self.retry_at
                && let Err(error) = self.start_next()
            {
                log::warn!("candidate start failed: {error}");
                self.schedule_retry();
            }
            let snapshot = self.worker.as_ref().map(Worker::snapshot);
            for event in self.capture.take_events() {
                self.event_callback.on_event(event);
            }
            match snapshot {
                Some(snapshot) if snapshot.state == WorkerState::TunnelEstablished => {
                    self.attempts = 0;
                    let peer = self.active.as_ref().expect("active worker").info();
                    #[cfg(feature = "local-agent")]
                    {
                        if let Err(error) = self.poll_account() {
                            log::error!("cannot start control path: {error}");
                            self.fatal = true;
                        }
                        if self.active.is_none() {
                            continue;
                        }
                    }
                    match self.config.connection_mode {
                        ConnectionMode::NoLocalAgent { .. } => {
                            self.publish(ConnectionState::Connected {
                                peer,
                                agent_info: None,
                            })
                        }
                        ConnectionMode::LocalAgent { .. } => {
                            #[cfg(feature = "local-agent")]
                            {
                                self.publish(self.agent_state.state(peer));
                            }
                        }
                    }
                    if self
                        .requested_stats
                        .is_some_and(|request| snapshot.stats_generation >= request)
                    {
                        self.requested_stats = None;
                        if let Some(stats) = snapshot.statistics {
                            self.event_callback.on_event(Event::ConnectionStats {
                                mono_timestamp_ms: snapshot
                                    .elapsed
                                    .as_millis()
                                    .min(i64::MAX as u128)
                                    as i64,
                                received_bytes: stats.received_bytes,
                                sent_bytes: stats.sent_bytes,
                                time_since_last_handshake: stats.handshake_age.unwrap_or_default(),
                                estimated_loss: stats.estimated_loss,
                                estimated_round_trip_time: stats.handshake_rtt.unwrap_or_default(),
                            });
                        }
                    }
                }
                Some(snapshot)
                    if matches!(
                        snapshot.state,
                        WorkerState::Failed(_)
                            | WorkerState::Stopped
                            | WorkerState::TunnelUnavailable
                    ) || self.attempt_at.elapsed() >= Duration::from_secs(15) =>
                {
                    self.stop_worker();
                    self.schedule_retry();
                }
                _ => self.publish(ConnectionState::Connecting {
                    peers: self.active.iter().map(Candidate::info).collect(),
                    wait_reasons: Vec::new(),
                }),
            }
        }
        self.stop_worker();
        let _ = self.capture.stop(true);
        for event in self.capture.take_events() {
            self.event_callback.on_event(event);
        }
        self.disconnected(None);
    }
    fn schedule_retry(&mut self) {
        self.retry_at = Instant::now() + crate::control::Candidates::retry_delay(self.attempts);
        self.attempts = self.attempts.saturating_add(1);
    }
    fn command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Update(update) => {
                // Validate and take replacement descriptor before applying any
                // portion of an atomic update.
                let replacement = update
                    .tun
                    .map(|tun| match tun {
                        TunStreamInfo::NoTun => Ok(None),
                        TunStreamInfo::TunFd(fd) => take_descriptor(fd).map(Some),
                    })
                    .transpose()?;
                let worker_tun = replacement
                    .as_ref()
                    .map(|tun| tun.as_ref().map(File::try_clone).transpose())
                    .transpose()?;
                if let Some(peers) = update.peers {
                    self.config.peers = peers;
                    self.rebuild();
                    let retained = self
                        .active
                        .as_ref()
                        .and_then(|active| self.plan.iter().find(|new| new.same_tunnel(active)))
                        .cloned();
                    if retained.is_none() {
                        self.stop_worker();
                        self.retry_at = Instant::now();
                    } else {
                        self.active = retained;
                    }
                }
                if let Some(settings) = update.settings
                    && let ConnectionMode::LocalAgent {
                        settings: current, ..
                    } = &mut self.config.connection_mode
                {
                    *current = settings;
                }
                #[cfg(feature = "local-agent")]
                if let (Some(account), ConnectionMode::LocalAgent { settings, .. }) =
                    (&self.account, &self.config.connection_mode)
                {
                    account.send(super::account::Command::Settings(
                        settings.clone(),
                        self.active.as_ref().and_then(|p| p.peer.exit_label.clone()),
                    ));
                }
                if let Some(tun) = replacement {
                    if let Some(worker) = &self.worker {
                        worker.replace_owned_tun(worker_tun.expect("prepared update"));
                    }
                    self.tun = tun;
                }
            }
            Command::Connectivity(event) => {
                self.config.network_available = event != ConnectivityEvent::Down;
                self.stop_worker();
                self.retry_at = Instant::now();
                self.next = 0;
            }
            Command::Capture(info) => {
                self.capture.start(info)?;
            }
            Command::StopCapture => {
                self.capture.stop(false)?;
            }
            Command::Stats => {
                if let Some(worker) = &self.worker {
                    self.requested_stats = Some(worker.request_stats());
                }
            }
            Command::Debug => log::debug!("connection state: {:?}", self.state),
            Command::InvalidateCertificate => {
                self.cache.remove(CacheKey::Certificate);
                #[cfg(feature = "local-agent")]
                if let Some(account) = &self.account {
                    account.send(super::account::Command::Invalidate);
                }
            }
            Command::Fork(info) => {
                #[cfg(feature = "local-agent")]
                if let Some(account) = &self.account {
                    account.send(super::account::Command::Fork(info));
                } else {
                    self.pending_fork = Some(info);
                }
                #[cfg(not(feature = "local-agent"))]
                {
                    let _ = info;
                    return Err(crate::Error::Invalid("build requires local-agent feature"));
                }
            }
            Command::AgentStats =>
            {
                #[cfg(feature = "local-agent")]
                if let Some(account) = &self.account {
                    account.send(super::account::Command::Stats);
                }
            }
            Command::Disconnect => {}
        }
        Ok(())
    }
    #[cfg(feature = "local-agent")]
    fn poll_account(&mut self) -> Result<()> {
        use super::account::{Command as AccountCommand, Notice, Service};
        use proton_vpn_local_agent::managed::ManagedLocalAgentEvent as AgentEvent;
        if self.account.is_none() {
            let network = self
                .worker
                .as_ref()
                .expect("active worker")
                .create_control_network()?;
            let account = Service::start(
                network,
                self.config.connection_mode.clone(),
                self.active.as_ref().expect("active peer").peer.clone(),
                self.cache.clone(),
            )?;
            if let Some(info) = self.pending_fork.take() {
                account.send(AccountCommand::Fork(info));
            }
            self.account = Some(account);
        }
        // Each pass is bounded; a noisy server cannot starve connectivity commands.
        for _ in 0..64 {
            let notice = self.account.as_ref().and_then(Service::try_recv);
            let Some(notice) = notice else { break };
            match notice {
                Notice::Event(event) => self.event_callback.on_event(event),
                Notice::CertificateUnavailable => self.agent_state.connected = false,
                Notice::Fatal => {
                    self.event_callback.on_event(Event::LocalAgentError {
                        error: ErrorEvent::CertificateRefreshFatalError,
                    });
                    self.fatal = true;
                    break;
                }
                Notice::Rekey => {
                    self.cache.remove(CacheKey::PrivateKey);
                    self.cache.remove(CacheKey::Certificate);
                    self.key = Zeroizing::new(
                        Identity::load_or_generate(self.cache.as_ref())?.wireguard_key(),
                    );
                    self.stop_worker();
                    self.retry_at = Instant::now();
                    break;
                }
                Notice::Agent(event) => match event {
                    AgentEvent::Value(value) => {
                        if let Some(event) = self.agent_state.accept(value) {
                            self.event_callback.on_event(event);
                        }
                    }
                    AgentEvent::Connected => self.agent_state.connected = true,
                    AgentEvent::Connecting | AgentEvent::NewCertificateNeeded(_) => {
                        self.agent_state.connected = false
                    }
                    AgentEvent::ServerError(error) => {
                        if let Some(event) = super::agent_state::AgentState::policy_error(&error) {
                            self.event_callback.on_event(event);
                        }
                    }
                    AgentEvent::ChangePeerNeeded | AgentEvent::ClientError(_) => {
                        self.stop_worker();
                        self.schedule_retry();
                        break;
                    }
                    AgentEvent::NewKeyNeeded(_) => {} // Service emits Rekey before forwarding.
                },
            }
        }
        Ok(())
    }
}
