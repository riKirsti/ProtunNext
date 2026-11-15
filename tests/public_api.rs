#[path = "contracts/api.rs"]
mod contracts;

use protun_next::api::{cache::MemoryCache, connection::*};

#[test]
fn cache_callbacks_keep_keys_independent_and_return_owned_copies() {
    let cache = MemoryCache::default();
    cache.put(CacheKey::PrivateKey, vec![7; 32]);
    cache.put(CacheKey::ApiSession, vec![9; 16]);
    let mut copy = cache.get(CacheKey::PrivateKey).unwrap();
    copy.fill(0);
    assert_eq!(cache.get(CacheKey::PrivateKey), Some(vec![7; 32]));
    cache.remove(CacheKey::PrivateKey);
    assert_eq!(cache.get(CacheKey::PrivateKey), None);
    assert_eq!(cache.get(CacheKey::ApiSession), Some(vec![9; 16]));
    cache.clear_all();
    assert_eq!(cache.get(CacheKey::ApiSession), None);
}

#[test]
fn binding_key_conversions_reject_invalid_lengths_and_keep_secret_debug_redacted() {
    for length in [0, 31, 33, 64] {
        assert!(WgClientPrivateKey::try_from(vec![7; length]).is_err());
        assert!(WgPeerPublicKey::try_from(vec![9; length]).is_err());
    }
    let private = WgClientPrivateKey::try_from(vec![7; 32]).unwrap();
    assert!(format!("{private:?}").contains("REDACTED"));
    assert_eq!(Vec::<u8>::from(private), vec![7; 32]);
    assert_eq!(
        Vec::<u8>::from(WgPeerPublicKey::try_from(vec![9; 32]).unwrap()),
        vec![9; 32]
    );
    let address = IpAddress::try_from("2001:db8::1".to_owned()).unwrap();
    assert_eq!(String::from(address), "2001:db8::1");
    assert!(IpAddress::try_from("invalid".to_owned()).is_err());
}
