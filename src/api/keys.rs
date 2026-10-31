//! Account identity is an Ed25519 seed; its X25519 scalar is the WireGuard key.
//! This is the derivation used by v3.0.5's published crypto utility. PKCS#8/PEM
//! and curve operations use dalek rather than hand-written cryptography.
use super::{
    cache,
    types::{CacheKey, PersistentCache},
};
use crate::{Error, Result};
use ed25519_dalek::{
    SigningKey,
    pkcs8::{DecodePrivateKey, EncodePublicKey},
};
use pkcs8::LineEnding;
use zeroize::Zeroizing;
pub struct Identity {
    signing: SigningKey,
}
impl Identity {
    pub fn load_or_generate(cache: &dyn PersistentCache) -> Result<Self> {
        if let Some(pem) = cache::load(cache, CacheKey::PrivateKey)? {
            return Self::from_pem(&pem);
        }
        let mut seed = Zeroizing::new([0u8; 32]);
        getrandom::fill(seed.as_mut()).map_err(|e| Error::Protocol(e.to_string()))?;
        let identity = Self {
            signing: SigningKey::from_bytes(&seed),
        };
        cache.put(
            CacheKey::PrivateKey,
            identity.private_pem()?.as_bytes().to_vec(),
        );
        Ok(identity)
    }
    pub fn from_pem(pem: &[u8]) -> Result<Self> {
        let text = std::str::from_utf8(pem).map_err(|_| Error::Invalid("identity PEM UTF-8"))?;
        Ok(Self {
            signing: SigningKey::from_pkcs8_pem(text)
                .map_err(|_| Error::Invalid("Ed25519 PKCS#8 identity"))?,
        })
    }
    pub fn wireguard_key(&self) -> [u8; 32] {
        self.signing.to_scalar_bytes()
    }
    pub fn public_pem(&self) -> Result<String> {
        self.signing
            .verifying_key()
            .to_public_key_pem(LineEnding::LF)
            .map_err(|_| Error::Invalid("public key encoding"))
    }
    pub fn private_pem(&self) -> Result<Zeroizing<String>> {
        let der = self.private_der()?;
        pkcs8::der::pem::encode_string("PRIVATE KEY", LineEnding::LF, der.secret_der())
            .map(Zeroizing::new)
            .map_err(|_| Error::Invalid("private key encoding"))
    }
    pub fn private_der(&self) -> Result<rustls::pki_types::PrivateKeyDer<'static>> {
        // RFC 8410 PKCS#8 v1 matches the exact envelope v3.0.5's cache reader
        // assumes. Dalek's default v2 adds a public-key field, so encode v1.
        let mut der = vec![
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20,
        ];
        let seed = Zeroizing::new(self.signing.to_bytes());
        der.extend_from_slice(seed.as_ref());
        Ok(rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(der),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persisted_ed25519_identity_preserves_wg_and_mtls_identity() {
        let cache = super::super::cache::MemoryCache::default();
        let first = Identity::load_or_generate(&cache).unwrap();
        let second = Identity::load_or_generate(&cache).unwrap();
        assert_eq!(first.private_der().unwrap().secret_der().len(), 48);
        assert_eq!(first.wireguard_key(), second.wireguard_key());
        assert_eq!(first.public_pem().unwrap(), second.public_pem().unwrap());
        // Published dalek conversion agrees with the actual WG crypto adapter.
        let converted = first.signing.verifying_key().to_montgomery().to_bytes();
        assert_eq!(
            converted,
            crate::session::Session::public_key(first.wireguard_key())
        );
        cache.remove(CacheKey::PrivateKey);
        assert_ne!(
            first.public_pem().unwrap(),
            Identity::load_or_generate(&cache)
                .unwrap()
                .public_pem()
                .unwrap()
        );
    }
}
