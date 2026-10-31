//! Account persistence and environment adapters, independent of the packet worker.
use super::{cache, types::*};
use crate::{Error, Result};
use muon::{auth::Auth, env::Env, tls::pins::TlsPinSet};
use proton_vpn_rcrl::types::{Certificate, CertificateRefreshParameters};
use rustls::pki_types::{CertificateDer, pem::PemObject};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, UNIX_EPOCH},
};

#[derive(Clone)]
pub struct Store(pub Arc<dyn PersistentCache>);
impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccountStore([REDACTED])")
    }
}
impl muon::store::Store for Store {
    type Key = ();
    fn set_auth(&mut self, _: (), auth: Auth) {
        if matches!(auth, Auth::None) {
            self.0.remove(CacheKey::ApiSession);
            return;
        }
        match serde_json::to_vec(&auth) {
            Ok(bytes) => self.0.put(CacheKey::ApiSession, bytes),
            Err(_) => log::error!("cannot encode account session for cache"),
        }
    }
    fn remove_auth(&mut self, _: &()) {
        self.0.remove(CacheKey::ApiSession);
    }
    fn get_all_auth(&self) -> HashMap<(), Auth> {
        cache::load(self.0.as_ref(), CacheKey::ApiSession)
            .ok()
            .flatten()
            .and_then(|data| serde_json::from_slice(&data).ok())
            .filter(|auth| !matches!(auth, Auth::None))
            .map(|auth| HashMap::from([((), auth)]))
            .unwrap_or_default()
    }
}
impl proton_vpn_rcrl::storage::Storage for Store {
    type Error = std::io::Error;
    fn store(&mut self, cert: &Certificate) -> std::io::Result<()> {
        self.0.put(CacheKey::Certificate, encode_certificate(cert));
        Ok(())
    }
}
pub fn encode_certificate(cert: &Certificate) -> Vec<u8> {
    let mut text = String::new();
    for (name, value) in [
        ("Issue-Date", cert.issue_time),
        ("Refresh-Date", cert.refresh_time),
    ] {
        if let Some(time) = value.and_then(|t| UNIX_EPOCH.checked_add(Duration::from_secs(t))) {
            text.push_str(&format!("{name}: {}\n", httpdate::fmt_http_date(time)));
        }
    }
    text.push_str(&cert.data);
    text.into_bytes()
}
pub fn load_certificate(store: &dyn PersistentCache) -> Result<Option<Certificate>> {
    cache::load(store, CacheKey::Certificate)?
        .map(|bytes| decode_certificate(&bytes))
        .transpose()
}
pub fn decode_certificate(bytes: &[u8]) -> Result<Certificate> {
    let text = std::str::from_utf8(bytes).map_err(|_| Error::Invalid("certificate cache UTF-8"))?;
    let start = text
        .find("-----BEGIN CERTIFICATE-----")
        .ok_or(Error::Invalid("certificate cache PEM"))?;
    let pem = &text[start..];
    let der = certificate_der(pem)?;
    let (remaining, parsed) = x509_parser::parse_x509_certificate(der.as_ref())
        .map_err(|_| Error::Invalid("certificate X.509"))?;
    if !remaining.is_empty() {
        return Err(Error::Invalid("certificate trailing DER"));
    }
    let expiry = parsed
        .validity()
        .not_after
        .timestamp()
        .try_into()
        .map_err(|_| Error::Invalid("certificate expired before Unix epoch"))?;
    let mut cert = Certificate::new(pem.to_owned(), expiry);
    for line in text[..start].lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let seconds = httpdate::parse_http_date(value.trim())
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        match name {
            "Issue-Date" => cert.issue_time = seconds,
            "Refresh-Date" => cert.refresh_time = seconds,
            _ => {}
        }
    }
    Ok(cert)
}
pub fn certificate_der(pem: &str) -> Result<CertificateDer<'static>> {
    CertificateDer::from_pem_slice(pem.as_bytes()).map_err(|_| Error::Invalid("certificate PEM"))
}
pub fn features(settings: &LocalAgentSettings) -> CertificateRefreshParameters {
    let mut p = CertificateRefreshParameters::default();
    p.net_shield_level = settings.netshield_level.map(|n| match n {
        NetshieldLevel::None => 0,
        NetshieldLevel::MalwareFilter => 1,
        NetshieldLevel::AdsAndMalwareFilter => 2,
        NetshieldLevel::AdultAndAdsAndMalwareFilter => 3,
    });
    p.split_tcp = settings.split_tcp;
    p.random_nat = settings.random_nat;
    p.port_forwarding = settings.port_forwarding;
    p
}

#[derive(Clone, Default)]
pub struct CookiesStore(pub Arc<Mutex<Option<Cookies>>>);
impl std::fmt::Debug for CookiesStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CookiesStore([REDACTED])")
    }
}
impl muon::cookie_store::CookieStore for CookiesStore {
    fn set(
        &mut self,
        _: cookie_store::Cookie,
        _: &url::Url,
    ) -> muon::cookie_store::CookieStoreSetResult {
        muon::cookie_store::CookieStoreSetResult::Pass
    }
    fn get<'a>(&'a self, _: &url::Url) -> Vec<cookie_store::RawCookie<'a>> {
        self.0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(|cookies| {
                cookies
                    .items
                    .iter()
                    .map(|c| cookie_store::RawCookie::new(c.name.clone(), c.value.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
}
struct CustomEnv {
    servers: Vec<muon::common::Server>,
    api: Option<TlsPinSet>,
    ar: Option<TlsPinSet>,
}
impl Env for CustomEnv {
    fn servers(&self, _: &muon::app::AppVersion) -> Vec<muon::common::Server> {
        self.servers.clone()
    }
    fn api_pins(&self) -> Option<&TlsPinSet> {
        self.api.as_ref()
    }
    fn ar_pins(&self) -> Option<&TlsPinSet> {
        self.ar.as_ref()
    }
}
pub fn environment(env: &MuonEnv) -> Result<muon::Environment> {
    let prod = muon::env::Prod::default();
    let pins =
        |p: &Option<Vec<String>>, default: Option<&TlsPinSet>| -> Result<Option<TlsPinSet>> {
            match p {
                Some(values) => TlsPinSet::from_b64(values)
                    .map(Some)
                    .map_err(|_| Error::Invalid("TLS pin encoding")),
                None => Ok(default.cloned()),
            }
        };
    let custom = |servers: &[String], api: Option<TlsPinSet>, ar: Option<TlsPinSet>| -> Result<_> {
        if servers.is_empty() {
            return Err(Error::Invalid("no API servers"));
        }
        let servers = servers
            .iter()
            .map(|s| s.parse().map_err(|_| Error::Invalid("API server URL")))
            .collect::<Result<Vec<_>>>()?;
        Ok(muon::Environment::new_custom(CustomEnv {
            servers,
            api,
            ar,
        }))
    };
    match env {
        MuonEnv::Prod => custom(
            &["https://vpn-api.proton.me/".into()],
            prod.api_pins().cloned(),
            prod.ar_pins().cloned(),
        ),
        MuonEnv::Atlas {
            scientist: Some(name),
        } => Ok(muon::Environment::new_atlas_name(name.clone())),
        MuonEnv::Atlas { scientist: None } => Ok(muon::Environment::new_atlas()),
        MuonEnv::CustomServers {
            servers,
            api_pins,
            ar_pins,
        } => custom(
            servers,
            pins(api_pins, prod.api_pins())?,
            pins(ar_pins, prod.ar_pins())?,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_pem_headers_and_unadorned_certificates_roundtrip() {
        let der = include_bytes!("../../tests/fixtures/server-TEST-ONLY.der");
        let pem =
            pkcs8::der::pem::encode_string("CERTIFICATE", pkcs8::LineEnding::LF, der).unwrap();
        let mut certificate = decode_certificate(pem.as_bytes()).unwrap();
        assert_eq!(certificate.data, pem);
        certificate.issue_time = Some(1_783_336_885);
        certificate.refresh_time = Some(1_783_376_485);
        assert_eq!(
            decode_certificate(&encode_certificate(&certificate)).unwrap(),
            certificate
        );
        let bad_header = format!("Refresh-Date: soon\n{pem}");
        assert_eq!(
            decode_certificate(bad_header.as_bytes())
                .unwrap()
                .refresh_time,
            None
        );
        assert!(decode_certificate(b"not PEM").is_err());
    }
    #[test]
    fn environments_reject_empty_hosts_and_bad_pins_and_preserve_production_pins() {
        let env = environment(&MuonEnv::Prod).unwrap();
        assert!(env.api_pins().is_some());
        assert!(env.ar_pins().is_some());
        assert!(
            environment(&MuonEnv::CustomServers {
                servers: vec![],
                api_pins: None,
                ar_pins: None
            })
            .is_err()
        );
        assert!(
            environment(&MuonEnv::CustomServers {
                servers: vec!["https://example.invalid".into()],
                api_pins: Some(vec!["bad pin".into()]),
                ar_pins: None
            })
            .is_err()
        );
    }
    #[test]
    fn cleared_auth_is_removed_from_persistence_and_not_restored() {
        use muon::store::Store as _;
        let cache = Arc::new(super::super::cache::MemoryCache::default());
        cache.put(CacheKey::ApiSession, b"bad credentials".to_vec());
        let mut store = Store(cache.clone());
        assert!(store.get_all_auth().is_empty());
        store.set_auth((), Auth::None);
        assert!(cache.get(CacheKey::ApiSession).is_none());
    }
}
