//! Account service ownership and server-observed state; worker changes stay in the supervisor.
use crate::{
    Result,
    api::{
        account::{Command, Notice, Service},
        agent_state::AgentState,
        internal::InternalConnector,
        types::*,
    },
};
use proton_vpn_local_agent::managed::ManagedLocalAgentEvent as AgentEvent;
use std::sync::Arc;
use zeroize::Zeroizing;

pub(super) enum Action {
    Continue,
    Event(Event),
    Rekey,
    Retry,
    Fatal,
}

#[derive(Default)]
pub(super) struct Account {
    service: Option<Service>,
    connector: Option<InternalConnector>,
    peer: Option<PeerInfo>,
    state: AgentState,
    pending_fork: Option<Zeroizing<ForkSelectorInfo>>,
}
impl Account {
    pub fn attach(
        &mut self,
        connector: InternalConnector,
        mode: &ConnectionMode,
        cache: &Arc<dyn PersistentCache>,
    ) -> Result<()> {
        self.connector = Some(connector);
        self.start(mode, cache)
    }
    fn start(&mut self, mode: &ConnectionMode, cache: &Arc<dyn PersistentCache>) -> Result<()> {
        if self.service.is_none() {
            let service = Service::start_waiting(
                self.connector
                    .as_ref()
                    .expect("initialized control network")
                    .clone(),
                mode.clone(),
                cache.clone(),
            )?;
            if let Some(info) = self.pending_fork.take() {
                service.send(Command::Fork(info));
            }
            self.service = Some(service);
        }
        Ok(())
    }
    pub fn restart(
        &mut self,
        mode: &ConnectionMode,
        cache: &Arc<dyn PersistentCache>,
    ) -> Result<()> {
        self.service = None;
        self.state = Default::default();
        self.start(mode, cache)
    }
    pub fn stop(&mut self) {
        // Drop/join the service before releasing its connector and control network.
        self.service = None;
        self.connector = None;
    }
    pub fn update_peer(&mut self, peer: &PeerInfo) {
        if self.peer.as_ref().is_none_or(|previous| {
            previous.server_ip != peer.server_ip
                || previous.server_public_key != peer.server_public_key
                || previous.server_name != peer.server_name
        }) {
            self.state = Default::default();
        }
        self.peer = Some(peer.clone());
        if let Some(service) = &self.service {
            service.send(Command::Peer(peer.clone()));
        }
    }
    fn send(&self, command: Command) {
        if let Some(service) = &self.service {
            service.send(command);
        }
    }
    pub fn settings(&self, mode: &ConnectionMode, active: Option<&PeerInfo>) {
        if let (Some(service), ConnectionMode::LocalAgent { settings, .. }) = (&self.service, mode)
        {
            service.send(Command::Settings(
                settings.clone(),
                active.and_then(|p| p.exit_label.clone()),
            ));
        }
    }
    pub fn invalidate(&self) {
        self.send(Command::Invalidate);
    }
    pub fn request_stats(&self) {
        self.send(Command::Stats);
    }
    pub fn fork(&mut self, info: Zeroizing<ForkSelectorInfo>) {
        if let Some(service) = &self.service {
            service.send(Command::Fork(info));
        } else {
            self.pending_fork = Some(info);
        }
    }
    pub fn connection_state(&self, peer: PeerConnectionInfo) -> ConnectionState {
        self.state.state(peer)
    }
    pub fn next_action(&mut self) -> Option<Action> {
        self.service
            .as_ref()
            .and_then(Service::try_recv)
            .map(|notice| self.accept(notice))
    }
    fn accept(&mut self, notice: Notice) -> Action {
        match notice {
            Notice::Event(event) => return Action::Event(event),
            Notice::CertificateUnavailable => self.state.connected = false,
            Notice::Fatal => return Action::Fatal,
            Notice::Rekey => return Action::Rekey,
            Notice::Agent(event) => match event {
                AgentEvent::Value(value) => {
                    if let Some(event) = self.state.accept(value) {
                        return Action::Event(event);
                    }
                }
                AgentEvent::Connected => self.state.connected = true,
                AgentEvent::Connecting | AgentEvent::NewCertificateNeeded(_) => {
                    self.state.connected = false
                }
                AgentEvent::ServerError(error) => {
                    if let Some(event) = AgentState::policy_error(&error) {
                        return Action::Event(event);
                    }
                }
                AgentEvent::ChangePeerNeeded | AgentEvent::ClientError(_) => return Action::Retry,
                AgentEvent::NewKeyNeeded(_) => {} // Service emits Rekey before forwarding.
            },
        }
        Action::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn peer() -> PeerInfo {
        PeerInfo {
            peer_id: "fixture".into(),
            server_ip: IpAddress("192.0.2.1".parse().unwrap()),
            server_public_key: WgPeerPublicKey([9; 32]),
            udp_ports: vec![7],
            tcp_ports: vec![],
            tls_ports: vec![],
            priority: 0,
            exit_label: None,
            server_name: Some("example.com".into()),
        }
    }
    #[test]
    fn same_agent_identity_keeps_observed_state_but_changed_identity_clears_it() {
        let mut account = Account::default();
        let mut peer = peer();
        account.update_peer(&peer);
        account.state.connected = true;
        account.state.info.settings.port_forwarding = Some(true);
        peer.peer_id = "renamed".into();
        peer.exit_label = Some("requested".into());
        account.update_peer(&peer);
        assert!(account.state.connected);
        assert_eq!(account.state.info.settings.port_forwarding, Some(true));
        peer.server_name = Some("different.example.com".into());
        account.update_peer(&peer);
        assert!(!account.state.connected);
        assert_eq!(account.state.info.settings.port_forwarding, None);
    }
    #[test]
    fn account_notices_keep_retry_rekey_and_disconnect_effects_distinct() {
        let mut account = Account::default();
        assert!(matches!(
            account.accept(Notice::Agent(AgentEvent::Connected)),
            Action::Continue
        ));
        assert!(account.state.connected);
        assert!(matches!(
            account.accept(Notice::CertificateUnavailable),
            Action::Continue
        ));
        assert!(!account.state.connected);
        assert!(matches!(
            account.accept(Notice::Agent(AgentEvent::ChangePeerNeeded)),
            Action::Retry
        ));
        assert!(matches!(account.accept(Notice::Rekey), Action::Rekey));
        assert!(matches!(account.accept(Notice::Fatal), Action::Fatal));
        let event = Event::LocalAgentError {
            error: ErrorEvent::ForkSelectorNeeded,
        };
        assert!(matches!(
            account.accept(Notice::Event(event)),
            Action::Event(Event::LocalAgentError {
                error: ErrorEvent::ForkSelectorNeeded
            })
        ));
    }
}
