//! Host callbacks and outstanding statistics requests, owned by the supervisor.
use crate::{api::types::*, connection::Snapshot};
use std::time::Duration;

pub(super) struct Notifications {
    state_callback: Box<dyn StateChangedCallback>,
    event_callback: Box<dyn EventCallback>,
    state: Option<VpnState>,
    origin: Duration,
    requested_stats: Option<u64>,
}

impl Notifications {
    pub fn new(
        state_callback: Box<dyn StateChangedCallback>,
        event_callback: Box<dyn EventCallback>,
    ) -> Self {
        Self {
            state_callback,
            event_callback,
            state: None,
            origin: Duration::ZERO,
            requested_stats: None,
        }
    }
    pub fn start(&mut self, origin: Duration) {
        self.origin = origin;
    }
    pub fn state(&self) -> &Option<VpnState> {
        &self.state
    }
    pub fn publish(&mut self, interface_state: InterfaceState, connection_state: ConnectionState) {
        let state = VpnState {
            interface_state,
            connection_state,
        };
        if self.state.as_ref() != Some(&state) {
            self.state = Some(state.clone());
            self.state_callback.on_state_changed(state);
        }
    }
    pub fn event(&self, event: Event) {
        self.event_callback.on_event(event);
    }
    pub fn request_stats(&mut self, generation: u64) {
        self.requested_stats = Some(generation);
    }
    pub fn clear_stats_request(&mut self) {
        self.requested_stats = None;
    }
    pub fn observe_stats(&mut self, snapshot: &Snapshot) {
        if !self
            .requested_stats
            .is_some_and(|request| snapshot.stats_generation >= request)
        {
            return;
        }
        self.clear_stats_request();
        if let Some(stats) = &snapshot.statistics {
            self.event(Event::ConnectionStats {
                // Keep the connection's origin when a transport worker is replaced.
                mono_timestamp_ms: snapshot
                    .mono_timestamp
                    .saturating_sub(self.origin)
                    .as_millis()
                    .min(i64::MAX as u128) as i64,
                received_bytes: stats.received_bytes,
                sent_bytes: stats.sent_bytes,
                time_since_last_handshake: stats.handshake_age.unwrap_or_default(),
                estimated_loss: stats.estimated_loss,
                estimated_round_trip_time: stats.handshake_rtt.unwrap_or_default(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{connection::State, session::Statistics};
    use std::sync::{Arc, Mutex};
    #[test]
    fn coalesces_states_and_keeps_statistics_origin_across_worker_generations() {
        let states = Arc::new(Mutex::new(Vec::new()));
        let events = Arc::new(Mutex::new(Vec::new()));
        let collected_states = states.clone();
        let collected_events = events.clone();
        let mut output = Notifications::new(
            Box::new(move |s| collected_states.lock().unwrap().push(s)),
            Box::new(move |e| collected_events.lock().unwrap().push(e)),
        );
        output.start(Duration::from_secs(5));
        let state = ConnectionState::Disconnected { error: None };
        output.publish(InterfaceState::Down { last_error: None }, state.clone());
        output.publish(InterfaceState::Down { last_error: None }, state.clone());
        output.publish(InterfaceState::Up { error: None }, state);
        assert_eq!(states.lock().unwrap().len(), 2);
        let mut snapshot = Snapshot {
            sequence: 1,
            state: State::TunnelEstablished,
            elapsed: Duration::ZERO,
            mono_timestamp: Duration::from_secs(10),
            stats_generation: 1,
            statistics: Some(Statistics {
                handshake_age: None,
                sent_bytes: 7,
                received_bytes: 9,
                estimated_loss: 0.0,
                handshake_rtt: None,
            }),
            counters: Default::default(),
            tun_present: true,
            interface_error: None,
        };
        output.request_stats(2);
        output.observe_stats(&snapshot);
        assert!(
            events.lock().unwrap().is_empty(),
            "stale worker generation answered request"
        );
        snapshot.stats_generation = 2;
        output.observe_stats(&snapshot);
        output.observe_stats(&snapshot);
        assert_eq!(events.lock().unwrap().len(), 1);
        assert!(matches!(
            events.lock().unwrap()[0],
            Event::ConnectionStats {
                mono_timestamp_ms: 5000,
                sent_bytes: 7,
                received_bytes: 9,
                ..
            }
        ));
        output.request_stats(3);
        output.clear_stats_request();
        snapshot.stats_generation = 3;
        output.observe_stats(&snapshot);
        assert_eq!(
            events.lock().unwrap().len(),
            1,
            "replaced worker leaked a stale request"
        );
        output.request_stats(1);
        snapshot.stats_generation = 1;
        snapshot.mono_timestamp = Duration::from_secs(12);
        output.observe_stats(&snapshot);
        assert!(matches!(
            events.lock().unwrap()[1],
            Event::ConnectionStats {
                mono_timestamp_ms: 7000,
                ..
            }
        ));
    }
}
