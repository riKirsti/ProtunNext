//! Certificate-verifying TLS using the selected unbuffered adapter.
//! Stealth uses the same adapter with its separate inner-WireGuard trust policy.
use crate::{Error, Result};
use std::sync::Arc;

pub use crate::tls_unbuffered::UnbufferedTlsIo as TlsIo;

pub fn client(server_name: &str) -> Result<rustls::client::UnbufferedClientConnection> {
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    client_with_roots(server_name, roots)
}
pub fn client_with_roots(
    server_name: &str,
    roots: rustls::RootCertStore,
) -> Result<rustls::client::UnbufferedClientConnection> {
    rustls::client::UnbufferedClientConnection::new(
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
