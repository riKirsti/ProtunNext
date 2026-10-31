//! Standard certificate-verifying TLS by default; no silent insecure fallback.
//! Proton Stealth's authenticated-inner-WireGuard certificate policy needs a
//! separate, explicit integration review before a live Stealth release.
use crate::{Error, Result};
use std::sync::Arc;
pub fn client(server_name: &str) -> Result<rustls::ClientConnection> {
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|e| Error::Protocol(e.to_string()))?
    .with_root_certificates(roots)
    .with_no_client_auth();
    config.alpn_protocols.clear();
    rustls::ClientConnection::new(
        Arc::new(config),
        rustls::pki_types::ServerName::try_from(server_name.to_owned())
            .map_err(|_| Error::Invalid("TLS server name"))?,
    )
    .map_err(|e| Error::Protocol(e.to_string()))
}
