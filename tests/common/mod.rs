#![allow(dead_code)]
//! Public test certificates only; never deployment credentials.
use rustls::{
    RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};
use std::sync::Arc;
pub fn roots() -> RootCertStore {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(
            include_bytes!("../fixtures/root-TEST-ONLY.der").to_vec(),
        ))
        .unwrap();
    roots
}
pub fn server() -> Arc<ServerConfig> {
    Arc::new(
        ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(
                    include_bytes!("../fixtures/server-TEST-ONLY.der").to_vec(),
                )],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
                    include_bytes!("../fixtures/private-PUBLIC-TEST-ONLY.der").to_vec(),
                )),
            )
            .unwrap(),
    )
}
