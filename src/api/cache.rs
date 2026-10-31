//! Host-owned persistence contract. Secret values are never formatted in logs.
use super::types::{CacheKey, PersistentCache};
use std::{collections::HashMap, sync::Mutex};
use zeroize::Zeroizing;
#[derive(Default)]
pub struct MemoryCache {
    entries: Mutex<HashMap<CacheKey, Zeroizing<Vec<u8>>>>,
}
impl PersistentCache for MemoryCache {
    fn put(&self, key: CacheKey, bytes: Vec<u8>) {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key, Zeroizing::new(bytes));
    }
    fn get(&self, key: CacheKey) -> Option<Vec<u8>> {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&key)
            .map(|v| v.to_vec())
    }
    fn remove(&self, key: CacheKey) {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&key);
    }
    fn clear_all(&self) {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
    }
}
pub(crate) fn load(
    cache: &dyn PersistentCache,
    key: CacheKey,
) -> crate::Result<Option<Zeroizing<Vec<u8>>>> {
    let bytes = cache.get(key).map(Zeroizing::new);
    if bytes.as_ref().is_some_and(|b| b.len() > 1024 * 1024) {
        return Err(crate::Error::Invalid("cache record too large"));
    }
    Ok(bytes)
}
