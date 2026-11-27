//! Control commands execute on a supervisor thread, never in the packet loop.
use super::{capture_worker::CaptureState, keys::Identity, types::*};
#[cfg(feature = "local-agent")]
mod account;
mod notifications;
mod peers;
use crate::{
    Result,
    config::{PrivateKey, WireguardConfig},
    connection::{Connection as Worker, State as WorkerState},
    platform::{self, TunHandle},
    transport::Transport,
};
use notifications::Notifications;
use peers::{Candidate, PeerPlan};
use std::{
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
        let join = self.join.lock().unwrap_or_else(|e| e.into_inner()).take();
        if let Some(join) = join
            && join.thread().id() != thread::current().id()
        {
            let _ = join.join();
        }
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
    pub(crate) fn connect_platform(
        config: InitialConnectionConfig,
        owned_tun: Result<Option<TunHandle>>,
        state_change_callback: Box<dyn StateChangedCallback>,
        event_callback: Box<dyn EventCallback>,
        socket_protector: Option<Arc<dyn SocketProtector>>,
        cache: Box<dyn PersistentCache>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();
        let join = thread::Builder::new()
            .name("protun-next-control".into())
            .spawn(move || {
                let mut actor = Supervisor::new(
                    config,
                    socket_protector,
                    Arc::from(cache),
                    Notifications::new(state_change_callback, event_callback),
                );
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
    fn send(&self, command: Command) {
        let _ = self.sender.send(command);
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.disconnect_and_wait();
    }
}

#[cfg(test)]
#[test]
fn invalid_initial_capture_does_not_stop_managed_connection() {
    let directory = tempfile::tempdir().unwrap();
    let (state_tx, states) = mpsc::channel();
    let (event_tx, events) = mpsc::channel();
    let connection = Connection::connect_platform(
        InitialConnectionConfig {
            peers: Vec::new(),
            network_available: false,
            pcap_file: Some(PcapFileInfo {
                file: PcapFile::Path {
                    path: directory.path().to_str().unwrap().into(),
                    mode: FileWriteMode::Overwrite,
                },
                max_bytes: None,
            }),
            connection_mode: ConnectionMode::NoLocalAgent {
                wg_private_key: Some(WgClientPrivateKey([7; 32])),
            },
            sni_strategy: SniStrategy::Random,
        },
        Ok(None),
        Box::new(move |state| state_tx.send(state).unwrap()),
        Box::new(move |event| event_tx.send(event).unwrap()),
        None,
        Box::new(super::cache::MemoryCache::default()),
    );
    let timeout = Duration::from_secs(4);
    let state = states.recv_timeout(timeout).unwrap();
    assert_eq!(
        state.connection_state,
        ConnectionState::Connecting {
            peers: Vec::new(),
            wait_reasons: vec![PeerConnectionWaitReason::WaitingForNetwork],
        }
    );

    let path = directory.path().join("capture.pcap");
    connection.start_packet_capture(PcapFileInfo {
        file: PcapFile::Path {
            path: path.to_str().unwrap().into(),
            mode: FileWriteMode::Overwrite,
        },
        max_bytes: None,
    });
    assert!(matches!(
        events.recv_timeout(timeout).unwrap(),
        Event::PacketCaptureStarted { .. }
    ));
    connection.disconnect_and_wait();
    assert_eq!(std::fs::metadata(path).unwrap().len(), 24);
}

#[cfg(unix)]
fn take_descriptor(fd: i32) -> Result<TunHandle> {
    // Validate before changing any supervisor state, including atomic updates.
    let owned = crate::unix::duplicate_nonblocking(fd);
    // SAFETY: legacy constructor transfers the caller's original descriptor.
    unsafe {
        libc::close(fd);
    }
    owned
}

#[cfg(all(test, unix))]
#[test]
fn ownership_transfer_closes_even_a_rejected_non_tun_descriptor() {
    use std::os::fd::AsRawFd;
    let file = tempfile::tempfile().unwrap();
    // SAFETY: duplicate an owned test file at a separate high descriptor number.
    let transferred = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 128) };
    assert!(transferred >= 128);
    assert!(take_descriptor(transferred).is_err());
    // SAFETY: scalar descriptor query; no ownership is acquired or changed.
    assert_eq!(unsafe { libc::fcntl(transferred, libc::F_GETFD) }, -1);
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::EBADF)
    );
}
struct Supervisor {
    config: InitialConnectionConfig,
    tun: Option<TunHandle>,
    protector: Option<Arc<dyn SocketProtector>>,
    cache: Arc<dyn PersistentCache>,
    notifications: Notifications,
    worker: Option<Worker>,
    peers: PeerPlan,
    active: Option<Candidate>,
    attempt_at: Instant,
    key: Zeroizing<[u8; 32]>,
    capture: Arc<CaptureState>,
    #[cfg(feature = "local-agent")]
    account: account::Account,
    #[cfg(feature = "control-plane")]
    control: Option<Arc<crate::connection::ControlNetwork>>,
    #[cfg(all(feature = "control-plane", not(feature = "local-agent")))]
    local_http: Option<super::local_http::Service>,
    fatal: bool,
}
impl Supervisor {
    fn new(
        config: InitialConnectionConfig,
        protector: Option<Arc<dyn SocketProtector>>,
        cache: Arc<dyn PersistentCache>,
        notifications: Notifications,
    ) -> Self {
        Self {
            config,
            tun: None,
            protector,
            cache,
            notifications,
            worker: None,
            peers: PeerPlan::new(Instant::now()),
            active: None,
            attempt_at: Instant::now(),
            key: Zeroizing::new([0; 32]),
            capture: Arc::new(CaptureState::default()),
            #[cfg(feature = "local-agent")]
            account: account::Account::default(),
            #[cfg(feature = "control-plane")]
            control: None,
            #[cfg(all(feature = "control-plane", not(feature = "local-agent")))]
            local_http: None,
            fatal: false,
        }
    }

    fn initialize(&mut self) -> Result<()> {
        self.notifications.start(platform::boottime()?);
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
        if let Some(info) = self.config.pcap_file.take()
            && let Err(error) = self.capture.start(info)
        {
            log::error!("failed to start initial packet capture: {error}");
        }
        self.peers.rebuild(&mut self.config.peers);
        #[cfg(feature = "control-plane")]
        {
            let (control, connector) =
                crate::connection::ControlNetwork::create(platform::addresses(self.tun.as_ref()))?;
            self.control = Some(control);
            #[cfg(feature = "local-agent")]
            {
                self.account
                    .attach(connector, &self.config.connection_mode, &self.cache)?;
            }
            #[cfg(not(feature = "local-agent"))]
            {
                self.local_http = Some(super::local_http::Service::start(connector)?);
            }
        }
        Ok(())
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
        self.notifications
            .publish(interface_state, connection_state);
    }
    fn disconnected(&mut self, error: Option<String>) {
        self.publish(ConnectionState::Disconnected {
            error: error.map(|message| DisconnectReason::TunEstablishError { message }),
        });
    }
    fn stop_worker(&mut self) {
        if let Some(mut worker) = self.worker.take() {
            let _ = worker.disconnect_and_wait();
            for event in worker.take_events() {
                self.notifications.event(event);
            }
        }
        self.active = None;
        self.notifications.clear_stats_request();
    }
    fn start_next(&mut self) -> Result<()> {
        let Some(candidate) = self.peers.next() else {
            return Ok(());
        };
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
            addresses: vec![format!(
                "{}/32",
                platform::addresses(self.tun.as_ref()).v4(2)
            )],
            allowed_ips: vec!["0.0.0.0/0".into(), "::/0".into()],
            dns: Vec::new(),
            mtu: 1420,
            keepalive: Some(25),
        };
        #[cfg(feature = "local-agent")]
        self.account.update_peer(&candidate.peer);
        let worker = Worker::start_runtime(
            self.tun.as_ref().map(platform::clone_tun).transpose()?,
            Some((config, String::new(), transport)),
            self.protector.clone(),
            self.runtime_services(),
        )?;
        self.worker = Some(worker);
        self.active = Some(candidate);
        self.attempt_at = Instant::now();
        Ok(())
    }
    fn runtime_services(&self) -> crate::connection::RuntimeServices {
        crate::connection::RuntimeServices {
            compatibility_policy: true,
            capture: Some(self.capture.clone()),
            #[cfg(feature = "control-plane")]
            control: self.control.clone(),
        }
    }
    fn start_offline(&mut self) -> Result<()> {
        self.worker = Some(Worker::start_runtime(
            self.tun.as_ref().map(platform::clone_tun).transpose()?,
            None,
            self.protector.clone(),
            self.runtime_services(),
        )?);
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
            #[cfg(feature = "local-agent")]
            if let Err(error) = self.poll_account() {
                log::error!("cannot maintain control path: {error}");
                self.fatal = true;
            }
            if self.fatal {
                break;
            }
            for event in self.capture.take_events() {
                self.notifications.event(event);
            }
            let ready = self
                .peers
                .ready(self.config.network_available, Instant::now());
            if self.active.is_none() && ready {
                self.stop_worker();
                if let Err(error) = self.start_next() {
                    log::warn!("candidate start failed: {error}");
                    self.schedule_retry();
                }
            }
            if self.worker.is_none()
                && let Err(error) = self.start_offline()
            {
                self.disconnected(Some(error.to_string()));
                break;
            }
            let snapshot = self.worker.as_ref().map(Worker::snapshot);
            if let Some(snapshot) = &snapshot {
                self.notifications.observe_stats(snapshot);
            }
            if self.active.is_none() {
                if let Some(WorkerState::Failed(error)) =
                    self.worker.as_ref().map(|w| w.snapshot().state)
                {
                    log::error!("local packet worker failed: {error}");
                    self.fatal = true;
                    continue;
                }
                self.publish(ConnectionState::Connecting {
                    peers: Vec::new(),
                    wait_reasons: if self.config.network_available {
                        Vec::new()
                    } else {
                        vec![PeerConnectionWaitReason::WaitingForNetwork]
                    },
                });
                continue;
            }
            match snapshot {
                Some(snapshot) if snapshot.state == WorkerState::TunnelEstablished => {
                    self.peers.connected();
                    let peer = self.active.as_ref().expect("active worker").info();
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
                                self.publish(self.account.connection_state(peer));
                            }
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
        #[cfg(feature = "local-agent")]
        {
            self.account.stop();
        }
        #[cfg(all(feature = "control-plane", not(feature = "local-agent")))]
        {
            self.local_http = None;
        }
        #[cfg(feature = "control-plane")]
        {
            self.control = None;
        }
        let _ = self.capture.stop(true);
        for event in self.capture.take_events() {
            self.notifications.event(event);
        }
        self.disconnected(None);
    }
    fn schedule_retry(&mut self) {
        self.peers.retry(Instant::now());
    }
    fn command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Update(update) => {
                // Validate and take replacement descriptor before applying any
                // portion of an atomic update.
                let replacement = update.tun.map(take_tun_update).transpose()?;
                let worker_tun = replacement
                    .as_ref()
                    .map(|tun| tun.as_ref().map(platform::clone_tun).transpose())
                    .transpose()?;
                #[cfg(feature = "local-agent")]
                let update_account = update.peers.is_some() || update.settings.is_some();
                if let Some(peers) = update.peers {
                    self.config.peers = peers;
                    self.peers.rebuild(&mut self.config.peers);
                    let retained = self
                        .active
                        .as_ref()
                        .and_then(|active| self.peers.retained(active));
                    if retained.is_none() {
                        self.stop_worker();
                        self.peers.retry_now(Instant::now());
                    } else {
                        self.active = retained;
                        #[cfg(feature = "local-agent")]
                        if let Some(peer) =
                            self.active.as_ref().map(|candidate| candidate.peer.clone())
                        {
                            self.account.update_peer(&peer);
                        }
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
                if update_account {
                    self.account.settings(
                        &self.config.connection_mode,
                        self.active.as_ref().map(|p| &p.peer),
                    );
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
                self.peers.restart(Instant::now());
            }
            Command::Capture(info) => {
                self.capture.start(info)?;
            }
            Command::StopCapture => {
                self.capture.stop(false)?;
            }
            Command::Stats => {
                if let Some(worker) = &self.worker {
                    self.notifications.request_stats(worker.request_stats());
                }
            }
            Command::Debug => {
                log::debug!("connection state: {:?}", self.notifications.state());
                if let Some(worker) = &self.worker {
                    log::debug!("packet worker: {:?}", worker.snapshot());
                }
            }
            Command::InvalidateCertificate => {
                self.cache.remove(CacheKey::Certificate);
                #[cfg(feature = "local-agent")]
                self.account.invalidate();
            }
            Command::Fork(info) => {
                #[cfg(feature = "local-agent")]
                self.account.fork(info);
                #[cfg(not(feature = "local-agent"))]
                {
                    let _ = info;
                    return Err(crate::Error::Invalid("build requires local-agent feature"));
                }
            }
            Command::AgentStats => {
                #[cfg(feature = "local-agent")]
                self.account.request_stats();
            }
            Command::Disconnect => {}
        }
        Ok(())
    }
    #[cfg(feature = "local-agent")]
    fn poll_account(&mut self) -> Result<()> {
        use account::Action;
        // A noisy server must not starve connectivity commands.
        for _ in 0..64 {
            let Some(action) = self.account.next_action() else {
                break;
            };
            match action {
                Action::Continue => {}
                Action::Event(event) => self.notifications.event(event),
                Action::Fatal => {
                    self.notifications.event(Event::LocalAgentError {
                        error: ErrorEvent::CertificateRefreshFatalError,
                    });
                    self.fatal = true;
                    break;
                }
                Action::Rekey => {
                    self.cache.remove(CacheKey::PrivateKey);
                    self.cache.remove(CacheKey::Certificate);
                    self.key = Zeroizing::new(
                        Identity::load_or_generate(self.cache.as_ref())?.wireguard_key(),
                    );
                    self.stop_worker();
                    self.account
                        .restart(&self.config.connection_mode, &self.cache)?;
                    self.peers.retry_now(Instant::now());
                    break;
                }
                Action::Retry => {
                    self.stop_worker();
                    self.schedule_retry();
                    break;
                }
            }
        }
        Ok(())
    }
}

fn take_tun_update(info: TunStreamInfo) -> Result<Option<TunHandle>> {
    #[cfg(unix)]
    {
        match info {
            TunStreamInfo::NoTun => Ok(None),
            TunStreamInfo::TunFd(fd) => take_descriptor(fd).map(Some),
        }
    }
    #[cfg(windows)]
    {
        let _ = info;
        Err(crate::Error::Unsupported(
            "Unix descriptor update on Windows",
        ))
    }
}

#[cfg(unix)]
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
        Self::connect_platform(
            config,
            tun_fd.map(take_descriptor).transpose(),
            state_change_callback,
            event_callback,
            socket_protector.map(Arc::from),
            cache,
        )
    }
    pub fn update_unix_tun(&self, tun_fd: TunStreamInfo) {
        self.update(ConfigUpdate {
            tun: Some(tun_fd),
            ..Default::default()
        });
    }
}
