//! Account runtime. Only control traffic enters the private TCP/IP stack; no
//! account locks, callbacks, JSON or async executor run on the packet thread.
use super::{
    account_material::{self as material, CookiesStore, Store},
    account_os::{Clock, Os},
    internal::InternalConnector,
    keys::Identity,
    types::*,
};
use crate::{Error, Result};
use futures::task::{LocalFutureObj, LocalSpawn, SpawnError};
use proton_vpn_local_agent::managed::{
    HandleManagedLocalAgentEvent, ManagedLocalAgent, ManagedLocalAgentEvent,
};
use proton_vpn_rcrl::{
    event::RefresherEvent,
    refresher::{CertificateRefresher, Setup},
    types::{CertificateApiDuration, CertificateRefreshSettings, Ed25519PublicKey},
};
use std::{
    sync::{Arc, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};
use tokio::sync::mpsc as async_channel;

pub enum Command {
    Settings(LocalAgentSettings, Option<String>),
    Stats,
    Fork(zeroize::Zeroizing<ForkSelectorInfo>),
    Invalidate,
    Stop,
}
pub enum Notice {
    Agent(ManagedLocalAgentEvent),
    Event(Event),
    Rekey,
    CertificateUnavailable,
    Fatal,
}
pub struct Service {
    sender: async_channel::UnboundedSender<Command>,
    notices: Option<mpsc::Receiver<Notice>>,
    join: Option<JoinHandle<()>>,
}
impl Service {
    pub fn start(
        network: InternalConnector,
        mode: ConnectionMode,
        peer: PeerInfo,
        cache: Arc<dyn PersistentCache>,
    ) -> Result<Self> {
        let (sender, commands) = async_channel::unbounded_channel();
        let (notices, receiver) = mpsc::sync_channel(128);
        let join = thread::Builder::new()
            .name("protun-next-account".into())
            .spawn(move || {
                let result = tokio::runtime::Builder::new_current_thread()
                    .enable_time()
                    .build()
                    .map_err(Error::from)
                    .and_then(|runtime| {
                        runtime.block_on(tokio::task::LocalSet::new().run_until(run(
                            network,
                            mode,
                            peer,
                            cache,
                            commands,
                            notices.clone(),
                        )))
                    });
                if result.is_err() {
                    // SDK errors can contain authentication material: never format them here.
                    log::error!("account runtime failed");
                    let _ = notices.send(Notice::Fatal);
                }
            })?;
        Ok(Self {
            sender,
            notices: Some(receiver),
            join: Some(join),
        })
    }
    pub fn send(&self, command: Command) {
        let _ = self.sender.send(command);
    }
    pub fn try_recv(&self) -> Option<Notice> {
        self.notices
            .as_ref()
            .and_then(|receiver| receiver.try_recv().ok())
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        self.send(Command::Stop);
        // Closing the receiver releases a producer blocked by a slow host
        // callback before joining the account thread. No shutdown deadlock.
        self.notices = None;
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}
#[derive(Clone, Copy)]
struct Executor;
impl LocalSpawn for Executor {
    fn spawn_local_obj(
        &self,
        future: LocalFutureObj<'static, ()>,
    ) -> std::result::Result<(), SpawnError> {
        tokio::task::spawn_local(future);
        Ok(())
    }
}
#[derive(Clone)]
struct Handler(async_channel::Sender<ManagedLocalAgentEvent>);
impl HandleManagedLocalAgentEvent for Handler {
    async fn handle(&mut self, event: ManagedLocalAgentEvent) {
        let _ = self.0.send(event).await;
    }
}
fn error_event(
    endpoint: ApiEndpoint,
    http_code: Option<u16>,
    proton_code: Option<i64>,
    refresh_token_invalid: bool,
) -> Notice {
    Notice::Event(Event::LocalAgentError {
        error: ErrorEvent::ApiError {
            endpoint,
            http_code,
            proton_code,
            message: None,
            refresh_token_invalid,
        },
    })
}
fn muon_error(endpoint: ApiEndpoint, error: &muon::Error) -> Notice {
    api_error(
        endpoint,
        error.http_code().map(|s| s.as_u16()),
        error.proton_error_code(),
        error.proton_error_reason(),
    )
}
fn reply_error(endpoint: ApiEndpoint, reply: &proton_vpn_rcrl::error::Reply) -> Notice {
    let info = reply.0.error_info();
    let (code, message) = info
        .maybe_body_error
        .map(|error| (Some(error.code), Some(error.error)))
        .unwrap_or_default();
    api_error(endpoint, Some(info.http_code.as_u16()), code, message)
}
fn api_error(
    endpoint: ApiEndpoint,
    http_code: Option<u16>,
    proton_code: Option<i64>,
    message: Option<String>,
) -> Notice {
    // This flag has a specific v3.0.5 meaning; authentication failure alone is
    // insufficient. SessionNeeded independently requests a fresh fork selector.
    Notice::Event(Event::LocalAgentError {
        error: ErrorEvent::ApiError {
            endpoint,
            http_code,
            proton_code,
            message,
            refresh_token_invalid: http_code == Some(422) && proton_code == Some(10013),
        },
    })
}
fn fork_needed(notices: &mpsc::SyncSender<Notice>) {
    let _ = notices.send(Notice::Event(Event::LocalAgentError {
        error: ErrorEvent::ForkSelectorNeeded,
    }));
}
async fn run(
    network: InternalConnector,
    mode: ConnectionMode,
    peer: PeerInfo,
    cache: Arc<dyn PersistentCache>,
    mut commands: async_channel::UnboundedReceiver<Command>,
    notices: mpsc::SyncSender<Notice>,
) -> Result<()> {
    tokio::task::spawn_local(super::local_http::run(network.factory().clone()));
    let ConnectionMode::LocalAgent {
        user_agent,
        app_version,
        mut settings,
        muon_env,
        certificate_duration,
    } = mode
    else {
        while let Some(command) = commands.recv().await {
            if matches!(command, Command::Stop) {
                break;
            }
        }
        return Ok(());
    };
    let identity = Identity::load_or_generate(cache.as_ref())?;
    let os = Os {
        network,
        clock: Clock::default(),
    };
    let cookies = CookiesStore::default();
    let app = muon::App::new(app_version)
        .map_err(|_| Error::Invalid("API app version"))?
        .with_user_agent(user_agent);
    let client = muon::Client::builder(app, material::environment(&muon_env)?)
        .with_operating_system(os.clone(), rand::rng())
        .with_local_executor(Executor)
        .with_persistence(Store(cache.clone()))
        .with_cookie_store(cookies.clone())
        .register_sdk(proton_vpn_rcrl::sdk())
        .build()
        .map_err(|_| Error::Invalid("account client configuration"))?;
    let mut cert_settings = CertificateRefreshSettings::new("ProtunNext");
    cert_settings.duration = certificate_duration.map(|duration| match duration {
        CertificateDuration::Seconds { count } => CertificateApiDuration::Seconds(count),
        CertificateDuration::Minutes { count } => CertificateApiDuration::Minutes(count),
        CertificateDuration::Hours { count } => CertificateApiDuration::Hours(count),
        CertificateDuration::Days { count } => CertificateApiDuration::Days(count),
    });
    let saved = match material::load_certificate(cache.as_ref()) {
        Ok(saved) => saved,
        Err(_) => {
            cache.remove(CacheKey::Certificate);
            None
        }
    };
    let setup = Setup::new(
        Ed25519PublicKey::new(identity.public_pem()?),
        cert_settings,
        Store(cache.clone()),
    )
    .with_features(material::features(&settings))
    .with_certificate(saved)
    .with_session(client.get_session(()).await);
    let mut refresher = CertificateRefresher::new(os.clone(), setup);
    refresher.resume();
    let (agent_events_tx, mut agent_events) = async_channel::channel(64);
    let mut agent = ManagedLocalAgent::new(Handler(agent_events_tx), identity.private_der()?);
    agent.subscribe(super::agent_state::AgentState::watched());
    let server_name = match &peer.server_name {
        Some(name) => rustls::pki_types::ServerName::try_from(name.clone())
            .map_err(|_| Error::Invalid("Local Agent server name"))?,
        None => rustls::pki_types::ServerName::IpAddress(peer.server_ip.0.into()),
    };
    let mut label = peer.exit_label;
    let mut connected_certificate: Option<String> = None;
    let mut invalidated = false;
    let (fork_tx, mut fork_rx) = async_channel::channel(1);
    let mut fork_task: Option<tokio::task::JoinHandle<()>> = None;
    let mut last_selector: Option<zeroize::Zeroizing<String>> = None;
    let mut tick = tokio::time::interval(Duration::from_secs(1));
    loop {
        // certificate() applies RCRL's API clock correction and expiry policy.
        let usable = if invalidated {
            None
        } else {
            refresher.certificate()
        };
        match usable {
            Some(certificate) if connected_certificate.as_ref() != Some(&certificate.data) => {
                let certificate_der = material::certificate_der(&certificate.data)?;
                agent.connect(
                    os.clone(),
                    &Executor,
                    certificate_der,
                    "10.2.0.1".parse().expect("constant IP"),
                    server_name.clone(),
                    &super::agent_state::settings(&settings, label.clone()),
                );
                connected_certificate = Some(certificate.data);
            }
            None if connected_certificate.take().is_some() => {
                agent.disconnect();
                let _ = notices.send(Notice::CertificateUnavailable);
            }
            _ => {}
        }
        tokio::select! {
            command = commands.recv() => match command {
                None | Some(Command::Stop) => break,
                Some(Command::Settings(new_settings, new_label)) => {
                    settings = new_settings; label = new_label;
                    refresher.change_features(material::features(&settings));
                    agent.change_settings(&super::agent_state::settings(&settings, label.clone()));
                }
                Some(Command::Stats) => for selector in super::agent_state::AgentState::stats_selectors() { agent.request_field(selector); },
                Some(Command::Invalidate) => {
                    cache.remove(CacheKey::Certificate); invalidated = true;
                    if connected_certificate.take().is_some() { agent.disconnect(); let _ = notices.send(Notice::CertificateUnavailable); }
                    refresher.refresh_now();
                }
                Some(Command::Fork(info)) => {
                    if last_selector.as_deref().is_some_and(|selector| selector == &info.selector) { continue; }
                    if let Some(task) = fork_task.take() { task.abort(); }
                    *cookies.0.lock().unwrap_or_else(|e| e.into_inner()) = Some(zeroize::Zeroizing::new(info.cookies.clone()));
                    last_selector = Some(zeroize::Zeroizing::new(info.selector.clone()));
                    let client = client.clone(); let sender = fork_tx.clone(); let notices = notices.clone();
                    fork_task = Some(tokio::task::spawn_local(async move {
                        let result = async {
                            let session = client.new_session_without_credentials(()).await?;
                            Ok::<_, muon::Error>(session.auth().from_fork().with_selector(&info.selector).await)
                        }.await;
                        match result {
                            Ok(muon::auth::WithSelectorFlow::Ok(session, _)) => { let _ = sender.send(session).await; }
                            Ok(muon::auth::WithSelectorFlow::Failed { reason, .. }) => {
                                let error: muon::Error = reason.into();
                                let _ = notices.send(muon_error(ApiEndpoint::Auth, &error)); fork_needed(&notices);
                            }
                            Err(error) => {
                                let _ = notices.send(muon_error(ApiEndpoint::Auth, &error));
                                fork_needed(&notices);
                            }
                        }
                    }));
                }
            },
            Some(session) = fork_rx.recv() => { refresher.change_session(Some(session)); refresher.resume(); },
            event = refresher.next_event(), if !refresher.is_stopped() => match event {
                Some(RefresherEvent::CertificateIssued) => invalidated = false,
                Some(RefresherEvent::SessionNeeded(error)) => {
                    cache.remove(CacheKey::ApiSession);
                    use proton_vpn_rcrl::error::SessionRejectedError;
                    match error {
                        SessionRejectedError::Reply(reply) => { let _ = notices.send(reply_error(ApiEndpoint::CertificateRefresh, &reply)); }
                        SessionRejectedError::Muon(error) => { let _ = notices.send(muon_error(ApiEndpoint::Auth, &error)); }
                        _ => {}
                    }
                    fork_needed(&notices);
                }
                Some(RefresherEvent::AttemptFailed(error)) => {
                    use proton_vpn_rcrl::error::RetriedError;
                    let event = match error {
                        RetriedError::Reply(reply) => reply_error(ApiEndpoint::CertificateRefresh, &reply),
                        RetriedError::Transport(error) | RetriedError::TokenRefresh(error) | RetriedError::RequestBuilding(error) => muon_error(ApiEndpoint::CertificateRefresh, &error),
                        _ => error_event(ApiEndpoint::CertificateRefresh, None, None, false),
                    };
                    let _ = notices.send(event);
                }
                Some(RefresherEvent::NewKeyNeeded(_)) => { let _ = notices.send(Notice::Rekey); break; }
                _ => {}
            },
            Some(event) = agent_events.recv() => {
                match &event {
                    ManagedLocalAgentEvent::ClientError(proton_vpn_local_agent::managed::UnrecoverableError::UnusableCertificate(_)) => {
                        cache.remove(CacheKey::Certificate); invalidated = true; refresher.refresh_now();
                        let _ = notices.send(Notice::CertificateUnavailable); continue;
                    }
                    ManagedLocalAgentEvent::NewCertificateNeeded(_) => { invalidated = true; refresher.refresh_now(); },
                    ManagedLocalAgentEvent::NewKeyNeeded(_) => { let _ = notices.send(Notice::Rekey); break; },
                    _ => {}
                }
                let _ = notices.send(Notice::Agent(event));
            },
            _ = tick.tick() => {}
        }
    }
    agent.disconnect();
    if let Some(task) = fork_task {
        task.abort();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refresh_token_flag_matches_upstream_http_and_proton_error_codes() {
        for (http, code, expected) in [
            (422, 10013, true),
            (401, 10013, false),
            (422, 10014, false),
            (500, 10013, false),
        ] {
            let Notice::Event(Event::LocalAgentError {
                error:
                    ErrorEvent::ApiError {
                        refresh_token_invalid,
                        message,
                        ..
                    },
            }) = api_error(
                ApiEndpoint::Auth,
                Some(http),
                Some(code),
                Some("server reason".into()),
            )
            else {
                panic!("wrong event")
            };
            assert_eq!(refresh_token_invalid, expected);
            assert_eq!(message.as_deref(), Some("server reason"));
        }
    }
}
