//! Linux worker ownership and observation, independent of account/control I/O.
//! This implements start/stop/join and real session statistics. Peer updates,
//! failover, live TUN replacement and account lifecycle are separate work items.
use crate::{
    Error, Result,
    config::WireguardConfig,
    linux::{self, Counters, Transport},
    session::{Session, Statistics},
};
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
}
struct Shared {
    snapshot: Mutex<Snapshot>,
    changed: Condvar,
    stop: AtomicBool,
    stats_requested: AtomicU64,
    stats_observed: AtomicU64,
    established: AtomicBool,
    next_sample_ms: AtomicU64,
}
pub(crate) struct Observer {
    shared: Arc<Shared>,
    start: Duration,
}
impl Observer {
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
        let owned_tun = linux::duplicate_nonblocking(tun.as_raw_fd())?;
        let start = linux::boottime()?;
        let shared = Arc::new(Shared {
            snapshot: Mutex::new(Snapshot {
                sequence: 1,
                state: State::Starting,
                elapsed: Duration::ZERO,
                stats_generation: 0,
                statistics: None,
                counters: Counters::default(),
            }),
            changed: Condvar::new(),
            stop: AtomicBool::new(false),
            stats_requested: AtomicU64::new(0),
            stats_observed: AtomicU64::new(0),
            established: AtomicBool::new(false),
            next_sample_ms: AtomicU64::new(0),
        });
        let observer = Observer {
            shared: shared.clone(),
            start,
        };
        let worker = thread::Builder::new()
            .name("protun-next-packets".into())
            .spawn(move || {
                let result = linux::run_transport(
                    owned_tun.as_raw_fd(),
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
        self.shared
            .stats_requested
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_add(1)
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
