//! Linux worker ownership and observation, independent of account/control I/O.
//! This implements start/stop/join and real session statistics. Peer updates,
//! failover, live TUN replacement and account lifecycle are separate work items.
use crate::api::{
    capture::CaptureController,
    types::{Event, PcapFileInfo, SocketProtector},
};
use crate::{
    Error, Result,
    config::WireguardConfig,
    linux::{self, Counters, Transport},
    session::{Session, Statistics},
};
use std::{collections::VecDeque, fs::File};
use std::{
    os::fd::{AsRawFd, BorrowedFd},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum State {
    Starting,
    /// Inner WireGuard handshake exists; says nothing about account/Plus policy.
    TunnelEstablished,
    TunnelUnavailable,
    Stopping,
    Stopped,
    Failed(String),
}
impl State {
    fn terminal(&self) -> bool {
        matches!(self, Self::Stopped | Self::Failed(_))
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Snapshot {
    pub sequence: u64,
    pub state: State,
    pub elapsed: Duration,
    pub stats_generation: u64,
    pub statistics: Option<Statistics>,
    pub counters: Counters,
    pub tun_present: bool,
    pub interface_error: Option<String>,
}
struct Shared {
    snapshot: Mutex<Snapshot>,
    changed: Condvar,
    stop: AtomicBool,
    stats_requested: AtomicU64,
    stats_observed: AtomicU64,
    established: AtomicBool,
    next_sample_ms: AtomicU64,
    capture: Arc<CaptureState>,
    owns_capture: bool,
    compatibility_policy: bool,
    tun_update: Mutex<Option<Option<File>>>,
    protector: Option<Arc<dyn SocketProtector>>,
    waker: Mutex<Option<Arc<mio::Waker>>>,
    #[cfg(feature = "local-agent")]
    internal: Mutex<Option<crate::api::internal::InternalNetwork>>,
    #[cfg(feature = "local-agent")]
    internal_enabled: AtomicBool,
}
/// A supervisor retains one sink across peer/socket replacement, so failover
/// neither truncates capture files nor repeats descriptor ownership transfer.
pub(crate) struct CaptureState {
    controller: Mutex<CaptureController>,
    capturing: AtomicBool,
    events: Mutex<VecDeque<Event>>,
}
impl Default for CaptureState {
    fn default() -> Self {
        Self {
            controller: Mutex::new(CaptureController::new()),
            capturing: AtomicBool::new(false),
            events: Mutex::new(VecDeque::new()),
        }
    }
}
impl CaptureState {
    fn event(&self, event: Event) {
        let mut events = self.events.lock().unwrap_or_else(|e| e.into_inner());
        if events.len() == 128 {
            events.pop_front();
        }
        events.push_back(event);
    }
    pub(crate) fn start(&self, info: PcapFileInfo) -> Result<()> {
        let mut capture = self.controller.lock().unwrap_or_else(|e| e.into_inner());
        let events = capture.start(info)?;
        self.capturing.store(capture.active(), Ordering::Relaxed);
        for event in events {
            self.event(event);
        }
        Ok(())
    }
    pub(crate) fn stop(&self, disconnected: bool) -> Result<()> {
        self.capturing.store(false, Ordering::Relaxed);
        let mut capture = self.controller.lock().unwrap_or_else(|e| e.into_inner());
        if disconnected && !capture.active() {
            return Ok(());
        }
        self.event(capture.stop(disconnected)?);
        Ok(())
    }
    pub(crate) fn take_events(&self) -> Vec<Event> {
        self.events
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .drain(..)
            .collect()
    }
}
pub(crate) struct Observer {
    shared: Arc<Shared>,
    start: Duration,
}
impl Observer {
    pub(crate) fn compatibility_policy(&self) -> bool {
        self.shared.compatibility_policy
    }
    pub(crate) fn set_waker(&self, waker: Arc<mio::Waker>) {
        *self.shared.waker.lock().unwrap_or_else(|e| e.into_inner()) = Some(waker);
    }
    #[cfg(feature = "local-agent")]
    pub(crate) fn internal_read(&self, output: &mut [u8], now: Duration) -> Result<Option<usize>> {
        if !self.shared.internal_enabled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        match self
            .shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            Some(network) => network.read_packet(output, now),
            None => Ok(None),
        }
    }
    #[cfg(feature = "local-agent")]
    pub(crate) fn internal_receive(&self, bytes: &mut [u8], now: Duration) -> Result<bool> {
        if !self.shared.internal_enabled.load(Ordering::Relaxed)
            || !crate::api::internal::InternalNetwork::might_receive(bytes)
        {
            return Ok(false);
        }
        match self
            .shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            Some(network) => network.receive(bytes, now),
            None => Ok(false),
        }
    }
    #[cfg(feature = "local-agent")]
    pub(crate) fn customer_outbound(&self, bytes: &mut [u8]) -> Result<()> {
        if !self.shared.compatibility_policy
            || !self.shared.internal_enabled.load(Ordering::Relaxed)
            || !crate::api::internal::InternalNetwork::might_send(bytes)
        {
            return Ok(());
        }
        if let Some(network) = self
            .shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            network.customer_outbound(bytes)?;
        }
        Ok(())
    }
    #[cfg(feature = "local-agent")]
    pub(crate) fn internal_local(&self, bytes: &[u8], now: Duration) -> Result<bool> {
        if !self.shared.internal_enabled.load(Ordering::Relaxed)
            || !crate::api::internal::local_service_destination(bytes)
        {
            return Ok(false);
        }
        match self
            .shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            Some(network) => network.receive_local(bytes, now),
            None => Ok(false),
        }
    }
    #[cfg(feature = "local-agent")]
    pub(crate) fn internal_wakeup(&self, now: Duration) -> Option<Duration> {
        if !self.shared.internal_enabled.load(Ordering::Relaxed) {
            return None;
        }
        self.shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .and_then(|n| n.next_wakeup(now))
    }
    pub(crate) fn protector(&self) -> Option<&dyn SocketProtector> {
        self.shared.protector.as_deref()
    }
    pub(crate) fn take_tun_update(&self) -> Option<Option<File>> {
        self.shared
            .tun_update
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
    }
    pub(crate) fn interface(&self, present: bool, error: Option<String>) {
        let mut snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        snapshot.tun_present = present;
        snapshot.interface_error = error;
        snapshot.sequence = snapshot.sequence.saturating_add(1);
        self.shared.changed.notify_all();
    }
    pub(crate) fn capture(&self, bytes: &[u8]) {
        if !self.shared.capture.capturing.load(Ordering::Relaxed) {
            return;
        }
        let wall = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let mut capture = self
            .shared
            .capture
            .controller
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        match capture.packet(wall, bytes) {
            Ok(Some(event)) => {
                self.shared
                    .capture
                    .capturing
                    .store(false, Ordering::Relaxed);
                self.shared.capture.event(event);
            }
            Ok(None) => {}
            Err(error) => {
                log::error!("capture write failed: {error}");
                self.shared
                    .capture
                    .capturing
                    .store(false, Ordering::Relaxed);
            }
        }
    }
    pub(crate) fn capture_wireguard(&self, bytes: &[u8], outbound: bool) {
        if !self.shared.capture.capturing.load(Ordering::Relaxed) || bytes.len() > 65507 {
            return;
        }
        // Same diagnostic convention as v3.0.5: synthetic UDP between
        // 10.2.0.2:51821 and 10.2.0.1:51820, also for TCP/Stealth payloads.
        // It deliberately describes inner WG frames, not real outer TLS/IP.
        let mut packet = [0u8; 65535];
        let n = bytes.len() + 28;
        packet[0] = 0x45;
        packet[2..4].copy_from_slice(&(n as u16).to_be_bytes());
        packet[8] = 125;
        packet[9] = 17;
        let (source, destination, sport, dport) = if outbound {
            (2, 1, 51821u16, 51820u16)
        } else {
            (1, 2, 51820, 51821)
        };
        packet[12..16].copy_from_slice(&[10, 2, 0, source]);
        packet[16..20].copy_from_slice(&[10, 2, 0, destination]);
        packet[20..22].copy_from_slice(&sport.to_be_bytes());
        packet[22..24].copy_from_slice(&dport.to_be_bytes());
        packet[24..26].copy_from_slice(&((bytes.len() + 8) as u16).to_be_bytes());
        let mut sum: u32 = packet[..20]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| u16::from_be_bytes([p[0], p[1]]) as u32)
            .sum();
        while sum > 65535 {
            sum = (sum & 65535) + (sum >> 16);
        }
        packet[10..12].copy_from_slice(&(!(sum as u16)).to_be_bytes());
        // A zero UDP checksum is permitted for IPv4 (RFC 768).
        packet[28..n].copy_from_slice(bytes);
        self.capture(&packet[..n]);
    }
    /// No per-packet locks, messages or user callbacks. A snapshot is published
    /// at most once/second, on request, or when the first handshake completes.
    pub(crate) fn observe(&self, session: &Session, counters: &Counters, now: Duration) {
        let request = self.shared.stats_requested.load(Ordering::Relaxed);
        let requested = request != self.shared.stats_observed.load(Ordering::Relaxed);
        let ms = now.as_millis().min(u64::MAX as u128) as u64;
        let due = ms >= self.shared.next_sample_ms.load(Ordering::Relaxed);
        if !requested && !due && self.shared.established.load(Ordering::Relaxed) {
            return;
        }
        let statistics = session.statistics();
        let connected = statistics.handshake_age.is_some();
        let previous = self.shared.established.swap(connected, Ordering::Relaxed);
        let state_changed = connected != previous;
        if !requested && !due && !state_changed {
            return;
        }
        self.shared
            .next_sample_ms
            .store(ms.saturating_add(1000), Ordering::Relaxed);
        let mut snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if state_changed && !snapshot.state.terminal() && snapshot.state != State::Stopping {
            snapshot.state = if connected {
                State::TunnelEstablished
            } else {
                State::TunnelUnavailable
            };
        }
        snapshot.elapsed = now.saturating_sub(self.start);
        snapshot.statistics = Some(statistics);
        snapshot.stats_generation = request;
        self.shared.stats_observed.store(request, Ordering::Relaxed);
        snapshot.counters = counters.clone();
        snapshot.sequence = snapshot.sequence.saturating_add(1);
        self.shared.changed.notify_all();
    }
    fn finish(&self, result: &Result<Counters>) {
        if self.shared.owns_capture {
            let _ = self.shared.capture.stop(true);
        }
        let mut snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        snapshot.state = match result {
            Ok(_) => State::Stopped,
            Err(e) => State::Failed(e.to_string()),
        };
        if let Ok(counters) = result {
            snapshot.counters = counters.clone();
        }
        if let Ok(now) = linux::boottime() {
            snapshot.elapsed = now.saturating_sub(self.start);
        }
        snapshot.sequence = snapshot.sequence.saturating_add(1);
        self.shared.changed.notify_all();
    }
}
pub struct Connection {
    shared: Arc<Shared>,
    worker: Option<JoinHandle<Result<Counters>>>,
}
impl Connection {
    /// Duplicates/validates the supplied TUN before spawning. Like run_udp, this
    /// enables O_NONBLOCK on the shared file description. No routes/DNS change.
    pub fn start(
        tun: BorrowedFd<'_>,
        config: WireguardConfig,
        uplink: String,
        transport: Transport,
    ) -> Result<Self> {
        Self::start_optional(Some(tun), config, uplink, transport, None)
    }
    pub fn start_optional(
        tun: Option<BorrowedFd<'_>>,
        config: WireguardConfig,
        uplink: String,
        transport: Transport,
        protector: Option<Arc<dyn SocketProtector>>,
    ) -> Result<Self> {
        Self::start_with_capture(tun, config, uplink, transport, protector, None)
    }
    pub(crate) fn start_with_capture(
        tun: Option<BorrowedFd<'_>>,
        config: WireguardConfig,
        uplink: String,
        transport: Transport,
        protector: Option<Arc<dyn SocketProtector>>,
        capture: Option<Arc<CaptureState>>,
    ) -> Result<Self> {
        let owned_tun = tun
            .map(|fd| linux::duplicate_nonblocking(fd.as_raw_fd()))
            .transpose()?;
        let start = linux::boottime()?;
        let shared = Arc::new(Shared {
            snapshot: Mutex::new(Snapshot {
                sequence: 1,
                state: State::Starting,
                elapsed: Duration::ZERO,
                stats_generation: 0,
                statistics: None,
                counters: Counters::default(),
                tun_present: owned_tun.is_some(),
                interface_error: None,
            }),
            changed: Condvar::new(),
            stop: AtomicBool::new(false),
            stats_requested: AtomicU64::new(0),
            stats_observed: AtomicU64::new(0),
            established: AtomicBool::new(false),
            next_sample_ms: AtomicU64::new(0),
            owns_capture: capture.is_none(),
            compatibility_policy: capture.is_some(),
            capture: capture.unwrap_or_else(|| Arc::new(CaptureState::default())),
            tun_update: Mutex::new(None),
            protector,
            waker: Mutex::new(None),
            #[cfg(feature = "local-agent")]
            internal: Mutex::new(None),
            #[cfg(feature = "local-agent")]
            internal_enabled: AtomicBool::new(false),
        });
        let observer = Observer {
            shared: shared.clone(),
            start,
        };
        let worker = thread::Builder::new()
            .name("protun-next-packets".into())
            .spawn(move || {
                let result = linux::run_transport(
                    owned_tun.as_ref().map(AsRawFd::as_raw_fd),
                    &config,
                    &uplink,
                    transport,
                    &observer.shared.stop,
                    Some(&observer),
                );
                let result = match result {
                    Err(Error::Io(e))
                        if observer.shared.stop.load(Ordering::Relaxed)
                            && e.kind() == std::io::ErrorKind::Interrupted =>
                    {
                        Ok(Counters::default())
                    }
                    other => other,
                };
                observer.finish(&result);
                result
            })?;
        Ok(Self {
            shared,
            worker: Some(worker),
        })
    }
    pub fn snapshot(&self) -> Snapshot {
        self.shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
    pub fn replace_tun(&self, tun: Option<BorrowedFd<'_>>) -> Result<()> {
        let file = tun
            .map(|fd| linux::duplicate_nonblocking(fd.as_raw_fd()))
            .transpose()?;
        *self
            .shared
            .tun_update
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(file);
        self.wake();
        Ok(())
    }
    fn wake(&self) {
        if let Some(waker) = self
            .shared
            .waker
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            let _ = waker.wake();
        }
    }
    #[cfg(feature = "local-agent")]
    pub fn create_control_network(&self) -> Result<crate::api::internal::InternalConnector> {
        struct WakeControl(std::sync::Weak<Shared>);
        impl std::task::Wake for WakeControl {
            fn wake(self: Arc<Self>) {
                if let Some(shared) = self.0.upgrade()
                    && let Some(waker) = shared
                        .waker
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .as_ref()
                {
                    let _ = waker.wake();
                }
            }
        }
        let (connector, network) = crate::api::internal::InternalNetwork::new(
            linux::boottime()?,
            WakeControl(Arc::downgrade(&self.shared)),
        )?;
        *self
            .shared
            .internal
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(network);
        self.shared.internal_enabled.store(true, Ordering::Relaxed);
        self.wake();
        Ok(connector)
    }
    pub fn start_packet_capture(&self, info: PcapFileInfo) -> Result<()> {
        self.shared.capture.start(info)
    }
    pub fn stop_packet_capture(&self) -> Result<()> {
        self.shared.capture.stop(false)
    }
    pub fn take_events(&self) -> Vec<Event> {
        self.shared.capture.take_events()
    }
    /// Bounded latest-state observation. Slow consumers never block packet I/O
    /// and do not build an unbounded event queue; sequence detects coalescing.
    pub fn wait_after(&self, sequence: u64, timeout: Duration) -> Snapshot {
        let snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let (snapshot, _) = self
            .shared
            .changed
            .wait_timeout_while(snapshot, timeout, |s| {
                s.sequence <= sequence && !s.state.terminal()
            })
            .unwrap_or_else(|e| e.into_inner());
        snapshot.clone()
    }
    pub fn request_stats(&self) -> u64 {
        let generation = self
            .shared
            .stats_requested
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_add(1);
        self.wake();
        generation
    }
    /// Waits for this request or a newer coalesced request to be sampled.
    pub fn wait_for_stats(&self, request: u64, timeout: Duration) -> Snapshot {
        let snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let (snapshot, _) = self
            .shared
            .changed
            .wait_timeout_while(snapshot, timeout, |s| {
                s.stats_generation < request && !s.state.terminal()
            })
            .unwrap_or_else(|e| e.into_inner());
        snapshot.clone()
    }
    /// Signals shutdown. Worker polling observes this within at most 250ms,
    /// excluding the current bounded I/O/crypto work; connect polls every 100ms.
    pub fn disconnect(&self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.wake();
        let mut snapshot = self
            .shared
            .snapshot
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if !snapshot.state.terminal() && snapshot.state != State::Stopping {
            snapshot.state = State::Stopping;
            snapshot.sequence = snapshot.sequence.saturating_add(1);
            self.shared.changed.notify_all();
        }
    }
    pub fn disconnect_and_wait(&mut self) -> Result<Counters> {
        self.disconnect();
        match self.worker.take() {
            Some(worker) => worker
                .join()
                .map_err(|_| Error::Protocol("packet worker panicked".into()))?,
            None => match self.snapshot().state {
                State::Failed(message) => Err(Error::Protocol(message)),
                _ => Ok(self.snapshot().counters),
            },
        }
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.disconnect_and_wait();
    }
}
