//! Certificate-verifying TLS with one adapter selected at compile time.
//! Stealth uses the same adapter with its separate inner-WireGuard trust policy.
use crate::{Error, Result};
use std::sync::Arc;

#[cfg(any(windows, target_vendor = "apple"))]
pub use crate::tls_buffered::BufferedTlsIo as TlsIo;
#[cfg(not(any(windows, target_vendor = "apple")))]
pub use crate::tls_unbuffered::UnbufferedTlsIo as TlsIo;

/// Native rustls connection accepted by `Transport::Tls` on this target.
/// Buffered on Windows/Apple; unbuffered on Linux/Android.
#[cfg(any(windows, target_vendor = "apple"))]
pub type ClientConnection = rustls::ClientConnection;
#[cfg(not(any(windows, target_vendor = "apple")))]
pub type ClientConnection = rustls::client::UnbufferedClientConnection;

pub(crate) fn framed<S: std::io::Read + std::io::Write>(
    socket: S,
    connection: ClientConnection,
) -> Result<crate::stream::FramedIo<TlsIo<S>>> {
    let io = TlsIo::new(socket, connection);
    #[cfg(any(windows, target_vendor = "apple"))]
    {
        crate::stream::FramedIo::new(io, 64)
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    {
        crate::stream::FramedIo::borrowing(io, 64)
    }
}

pub fn client(server_name: &str) -> Result<ClientConnection> {
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    client_with_roots(server_name, roots)
}
pub fn client_with_roots(
    server_name: &str,
    roots: rustls::RootCertStore,
) -> Result<ClientConnection> {
    ClientConnection::new(
        config_with_roots(roots)?,
        rustls::pki_types::ServerName::try_from(server_name.to_owned())
            .map_err(|_| Error::Invalid("TLS server name"))?,
    )
    .map_err(|e| Error::Protocol(e.to_string()))
}
fn config_with_roots(roots: rustls::RootCertStore) -> Result<Arc<rustls::ClientConfig>> {
    let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|e| Error::Protocol(e.to_string()))?
    .with_root_certificates(roots)
    .with_no_client_auth();
    config.alpn_protocols.clear();
    Ok(Arc::new(config))
}
