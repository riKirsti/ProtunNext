//! Proton Stealth's outer TLS policy, usable only through the inner WG worker.
//! Protocol evidence: pvpnclient 4.4.1 wireguard/tls.rs and proton-pfff-module
//! 0.8.10 tls/config.rs from Proton's checksum-verified registry archives.
//!
//! The outer certificate is not an authenticated identity. Its handshake
//! signature is still checked. Only the configured WireGuard peer key can
//! authenticate inner packets delivered to TUN. Never use this policy for HTTP,
//! account APIs, Local Agent or any application plaintext.
use crate::{Error, Result};
use std::net::IpAddr;

#[derive(Debug, Clone, Default)]
pub enum SniStrategy {
    #[default]
    Random,
    Top,
    /// An explicit DNS SNI, not an expected authenticated server identity.
    Explicit(String),
}
impl SniStrategy {
    pub fn for_peer(&self, peer: IpAddr) -> Result<String> {
        let name = match self {
            Self::Explicit(name) => name.clone(),
            Self::Top => {
                // Stable non-security mapping. Wire compatibility depends on a
                // valid SNI from this list, not Rust's unspecified DefaultHasher.
                let mut h = 0xcbf29ce484222325u64;
                for byte in peer.to_string().bytes() {
                    h = (h ^ byte as u64).wrapping_mul(0x100000001b3);
                }
                TOP_SNI[h as usize % TOP_SNI.len()].to_owned()
            }
            Self::Random => {
                // Match the published alphabet/label-length/TLD family. OS
                // randomness is used at connection creation, never per packet.
                const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
                const TLDS: &[&str] = &[
                    "com", "net", "org", "it", "fr", "me", "ru", "cn", "es", "tr", "top", "xyz",
                    "info",
                ];
                let mut random = [0u8; 16];
                getrandom::fill(&mut random).map_err(|e| Error::Protocol(e.to_string()))?;
                let length = 3 + random[0] as usize % 11;
                let mut name = String::with_capacity(32);
                for byte in &random[1..=length] {
                    name.push(LETTERS[*byte as usize % LETTERS.len()] as char);
                }
                name.push('.');
                name.push_str(TLDS[random[15] as usize % TLDS.len()]);
                name
            }
        };
        rustls::pki_types::DnsName::try_from(name.as_str())
            .map_err(|_| Error::Invalid("Stealth DNS SNI"))?;
        Ok(name)
    }
}

#[cfg(unix)]
mod policy {
    use super::*;
    use rustls::{
        DigitallySignedStruct, SignatureScheme,
        client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        crypto::{CryptoProvider, verify_tls12_signature, verify_tls13_signature},
        pki_types::{CertificateDer, ServerName, UnixTime},
    };
    use std::sync::Arc;
    #[derive(Debug)]
    struct InnerWireguardIdentity {
        provider: Arc<CryptoProvider>,
    }
    impl ServerCertVerifier for InnerWireguardIdentity {
        fn verify_server_cert(
            &self,
            _: &CertificateDer<'_>,
            _: &[CertificateDer<'_>],
            _: &ServerName<'_>,
            _: &[u8],
            _: UnixTime,
        ) -> std::result::Result<ServerCertVerified, rustls::Error> {
            Ok(ServerCertVerified::assertion())
        }
        fn verify_tls12_signature(
            &self,
            message: &[u8],
            certificate: &CertificateDer<'_>,
            signature: &DigitallySignedStruct,
        ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls12_signature(
                message,
                certificate,
                signature,
                &self.provider.signature_verification_algorithms,
            )
        }
        fn verify_tls13_signature(
            &self,
            message: &[u8],
            certificate: &CertificateDer<'_>,
            signature: &DigitallySignedStruct,
        ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls13_signature(
                message,
                certificate,
                signature,
                &self.provider.signature_verification_algorithms,
            )
        }
        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            self.provider
                .signature_verification_algorithms
                .supported_schemes()
        }
    }
    // Intentionally crate-private; all public entry points couple it to WireGuard.
    pub(crate) fn client(sni: String) -> Result<rustls::ClientConnection> {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let config = rustls::ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .map_err(|e| Error::Protocol(e.to_string()))?
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(InnerWireguardIdentity { provider }))
            .with_no_client_auth();
        rustls::ClientConnection::new(
            Arc::new(config),
            ServerName::try_from(sni).map_err(|_| Error::Invalid("Stealth SNI"))?,
        )
        .map_err(|e| Error::Protocol(e.to_string()))
    }
}
#[cfg(unix)]
pub(crate) use policy::client;

include!("stealth_top_sni.rs");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sni_strategies_produce_dns_names_and_stable_top_selection() {
        let peer = "192.0.2.1".parse().unwrap();
        for _ in 0..128 {
            assert!(SniStrategy::Random.for_peer(peer).is_ok());
        }
        assert_eq!(
            SniStrategy::Top.for_peer(peer).unwrap(),
            SniStrategy::Top.for_peer(peer).unwrap()
        );
        assert!(TOP_SNI.contains(&SniStrategy::Top.for_peer(peer).unwrap().as_str()));
        assert!(
            SniStrategy::Explicit("not a DNS name".into())
                .for_peer(peer)
                .is_err()
        );
    }
}
