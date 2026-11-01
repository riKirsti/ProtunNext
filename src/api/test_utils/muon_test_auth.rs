//! Explicit development login/fork utility, matching the pinned SDK examples.
//! This is the sole opt-in host-network login helper, never a VPN fallback.
use crate::api::{account_material, account_os::Clock, types::*};
use futures::{
    AsyncRead, AsyncWrite,
    task::{LocalFutureObj, LocalSpawn, SpawnError},
};
use muon::rt;
use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum TestAuthError {
    #[error("invalid development login configuration")]
    InvalidConfiguration,
    #[error("development login failed")]
    AuthenticationFailed,
    #[error("this development helper requires an account without two-factor authentication")]
    TwoFactorRequired,
    #[error("development session fork failed")]
    ForkFailed,
    #[error("development login runtime failed")]
    RuntimeFailed,
}
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn get_session_fork_selector(
    app: &str,
    user: &str,
    pass: &str,
    child: &str,
    muon_env: MuonEnv,
) -> Result<ForkSelectorInfo, TestAuthError> {
    let (app, user, pass, child) = (
        app.to_owned(),
        zeroize::Zeroizing::new(user.to_owned()),
        zeroize::Zeroizing::new(pass.to_owned()),
        child.to_owned(),
    );
    // Independent thread also permits a synchronous call from an async host.
    std::thread::Builder::new()
        .name("protun-next-test-login".into())
        .spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| TestAuthError::RuntimeFailed)?
                .block_on(get_session_fork_selector_async(
                    &app, &user, &pass, &child, muon_env,
                ))
        })
        .map_err(|_| TestAuthError::RuntimeFailed)?
        .join()
        .map_err(|_| TestAuthError::RuntimeFailed)?
}
pub async fn get_session_fork_selector_async(
    app: &str,
    user: &str,
    pass: &str,
    child: &str,
    muon_env: MuonEnv,
) -> Result<ForkSelectorInfo, TestAuthError> {
    tokio::task::LocalSet::new()
        .run_until(async {
            let app = muon::App::new(app).map_err(|_| TestAuthError::InvalidConfiguration)?;
            let child = muon::App::new(child).map_err(|_| TestAuthError::InvalidConfiguration)?;
            let child_name = child
                .app_version()
                .name()
                .ok_or(TestAuthError::InvalidConfiguration)?
                .to_string();
            let environment = account_material::environment(&muon_env)
                .map_err(|_| TestAuthError::InvalidConfiguration)?;
            let cookies = Jar::default();
            let session = muon::Client::builder(app, environment)
                .with_operating_system(Host::default(), rand::rng())
                .with_local_executor(Executor)
                .without_persistence::<()>()
                .with_cookie_store(cookies.clone())
                .build()
                .map_err(|_| TestAuthError::InvalidConfiguration)?
                .new_session_without_credentials(())
                .await
                .map_err(|_| TestAuthError::AuthenticationFailed)?;
            let session = match session.auth().login(user, pass).await {
                muon::auth::LoginFlow::Ok(session, _) => session,
                muon::auth::LoginFlow::TwoFactor(_, _) => {
                    return Err(TestAuthError::TwoFactorRequired);
                }
                muon::auth::LoginFlow::Failed { .. } => {
                    return Err(TestAuthError::AuthenticationFailed);
                }
            };
            let selector = match session.fork(child_name).send().await {
                muon::auth::ForkFlowResult::Success(_, selector) => selector,
                muon::auth::ForkFlowResult::Failure { .. } => {
                    return Err(TestAuthError::ForkFailed);
                }
            };
            let items = cookies
                .0
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .iter_unexpired()
                .map(|cookie| Cookie {
                    name: cookie.name().into(),
                    value: cookie.value().into(),
                })
                .collect();
            Ok(ForkSelectorInfo {
                selector,
                cookies: Cookies { items },
            })
        })
        .await
}
#[derive(Clone, Copy)]
struct Executor;
impl LocalSpawn for Executor {
    fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        tokio::task::spawn_local(future);
        Ok(())
    }
}
#[derive(Default, Clone)]
struct Jar(Arc<Mutex<cookie_store::CookieStore>>);
impl std::fmt::Debug for Jar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LoginCookies([REDACTED])")
    }
}
impl muon::cookie_store::CookieStore for Jar {
    fn set(
        &mut self,
        cookie: cookie_store::Cookie,
        url: &url::Url,
    ) -> muon::cookie_store::CookieStoreSetResult {
        let _ = self
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(cookie.into_owned(), url);
        muon::cookie_store::CookieStoreSetResult::Pass
    }
    fn get<'a>(&'a self, url: &url::Url) -> Vec<cookie_store::RawCookie<'a>> {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get_request_values(url)
            .map(|(name, value)| cookie_store::RawCookie::new(name.to_owned(), value.to_owned()))
            .collect()
    }
}
#[derive(Debug, Default, Clone)]
struct Host {
    clock: Clock,
    network: HostNetwork,
}
impl rt::OperatingSystem for Host {
    type Time = Clock;
    type TcpConnector = HostNetwork;
    type Resolver = HostNetwork;
    fn get_time_capabilities(&self) -> &Clock {
        &self.clock
    }
    fn get_tcp_connector(&self) -> &HostNetwork {
        &self.network
    }
    fn get_resolver(&self) -> &HostNetwork {
        &self.network
    }
}
#[derive(Debug, Default, Clone)]
struct HostNetwork;
impl rt::Resolve for HostNetwork {
    type Err = io::Error;
    async fn resolve(&self, host: &str) -> io::Result<Vec<std::net::IpAddr>> {
        Ok(tokio::net::lookup_host((host, 443))
            .await?
            .map(|address| address.ip())
            .collect())
    }
}
impl rt::TcpConnect for HostNetwork {
    type Socket = Stream;
    type Err = io::Error;
    async fn tcp_connect(&self, address: SocketAddr) -> io::Result<Stream> {
        Ok(Stream(tokio::net::TcpStream::connect(address).await?))
    }
}
struct Stream(tokio::net::TcpStream);
impl AsyncRead for Stream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut buffer = tokio::io::ReadBuf::new(bytes);
        match tokio::io::AsyncRead::poll_read(Pin::new(&mut self.0), cx, &mut buffer) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(buffer.filled().len())),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Pending => Poll::Pending,
        }
    }
}
impl AsyncWrite for Stream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        tokio::io::AsyncWrite::poll_write(Pin::new(&mut self.0), cx, bytes)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(Pin::new(&mut self.0), cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(Pin::new(&mut self.0), cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{AsyncReadExt, AsyncWriteExt};
    #[test]
    fn invalid_login_configuration_fails_before_opening_any_network_connection() {
        assert!(matches!(
            get_session_fork_selector("", "public-test", "public-test", "", MuonEnv::Prod),
            Err(TestAuthError::InvalidConfiguration)
        ));
    }
    #[test]
    fn host_adapter_reads_and_writes_real_loopback_tcp() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = std::thread::spawn(move || {
            use std::io::{Read, Write};
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .unwrap();
            let mut bytes = [0; 6];
            socket.read_exact(&mut bytes).unwrap();
            assert_eq!(&bytes, b"public");
            socket.write_all(&bytes).unwrap();
        });
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut stream = rt::TcpConnect::tcp_connect(&HostNetwork, address)
                    .await
                    .unwrap();
                stream.write_all(b"public").await.unwrap();
                stream.flush().await.unwrap();
                let mut bytes = [0; 6];
                stream.read_exact(&mut bytes).await.unwrap();
                assert_eq!(&bytes, b"public");
                stream.close().await.unwrap();
            });
        peer.join().unwrap();
    }
}
