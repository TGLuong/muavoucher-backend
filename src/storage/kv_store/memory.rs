use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::storage::kv_store::KVStoreTrait;

const DEFAULT_LIMIT: usize = 10485760; // 10MB

#[derive(Debug)]
pub struct CacheState {
    store: HashMap<String, String>,
    order: VecDeque<String>,
    size: usize,
    limit: usize,
}

impl CacheState {
    pub fn new(limit: usize) -> Self {
        Self {
            store: HashMap::default(),
            order: VecDeque::default(),
            size: 0,
            limit,
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), super::KVStoreError> {
        self.cleanup(&key, &value)?;
        self.size += key.len() + value.len();
        self.store.insert(key.clone(), value);
        self.order.push_back(key);
        Ok(())
    }

    pub fn pop(&mut self, key: &str) -> Option<String> {
        self.order.retain(|e| e != key);
        let data = self.store.remove(key);
        if let Some(value) = data.as_ref() {
            self.size -= key.len() + value.len();
        }
        data
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    pub fn cleanup(&mut self, key: &str, value: &str) -> Result<(), super::KVStoreError> {
        let incoming_size = key.len() + value.len();

        if incoming_size > self.limit {
            return Err(super::KVStoreError::ReachLimit);
        }

        if let Some(old_value) = self.store.remove(key) {
            self.size -= key.len() + old_value.len();
            self.order.retain(|stored_key| stored_key != key);
        }

        while self.size + incoming_size > self.limit && !self.order.is_empty() {
            if let Some(key) = self.order.pop_front() {
                if let Some(value) = self.store.remove(&key) {
                    self.size -= key.len() + value.len();
                }
            }
        }

        Ok(())
    }
}

impl Default for CacheState {
    fn default() -> Self {
        Self::new(DEFAULT_LIMIT)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryCache {
    cache: Arc<RwLock<CacheState>>,
}

impl MemoryCache {
    pub fn new(limit: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(CacheState::new(limit))),
        }
    }
}

impl KVStoreTrait for MemoryCache {
    async fn set(&self, key: impl Into<String> + Send, value: impl Into<String>) -> Result<(), super::KVStoreError> {
        let mut cache = self.cache.write().await;
        cache.insert(key.into(), value.into())
    }

    async fn get(&self, key: impl Into<String>) -> Result<Option<String>, super::KVStoreError> {
        let cache = self.cache.read().await;
        Ok(cache.get(&key.into()))
    }

    async fn pop(&self, key: impl Into<String> + Send) -> Result<Option<String>, super::KVStoreError> {
        let mut cache = self.cache.write().await;
        let data = cache.pop(&key.into());
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::kv_store::{KVStoreError, KVStoreTrait};

    #[tokio::test]
    async fn get_returns_none_for_missing_key() {
        let cache = MemoryCache::new(100);

        let value = cache.get("missing").await.unwrap();

        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn set_then_get_returns_stored_value() {
        let cache = MemoryCache::new(100);

        cache.set("key".to_string(), "value".to_string()).await.unwrap();

        assert_eq!(cache.get("key").await.unwrap(), Some("value".to_string()));
    }

    #[tokio::test]
    async fn cloned_cache_shares_the_same_state() {
        let cache = MemoryCache::new(100);
        let cloned = cache.clone();

        cache.set("key".to_string(), "value".to_string()).await.unwrap();

        assert_eq!(cloned.get("key").await.unwrap(), Some("value".to_string()));
    }

    #[tokio::test]
    async fn keeps_entries_when_total_size_is_within_limit() {
        let cache = MemoryCache::new(10);

        cache.set("a".to_string(), "1".to_string()).await.unwrap();
        cache.set("b".to_string(), "2".to_string()).await.unwrap();

        assert_eq!(cache.get("a").await.unwrap(), Some("1".to_string()));
        assert_eq!(cache.get("b").await.unwrap(), Some("2".to_string()));
    }

    #[tokio::test]
    async fn evicts_oldest_entries_when_limit_is_exceeded() {
        let cache = MemoryCache::new(4);

        cache.set("a".to_string(), "1".to_string()).await.unwrap();
        cache.set("b".to_string(), "2".to_string()).await.unwrap();
        cache.set("c".to_string(), "3".to_string()).await.unwrap();

        assert_eq!(cache.get("a").await.unwrap(), None);
        assert_eq!(cache.get("b").await.unwrap(), Some("2".to_string()));
        assert_eq!(cache.get("c").await.unwrap(), Some("3".to_string()));
    }

    #[tokio::test]
    async fn replaces_existing_key_without_duplicating_size() {
        let cache = MemoryCache::new(4);

        cache.set("a".to_string(), "1".to_string()).await.unwrap();
        cache.set("a".to_string(), "2".to_string()).await.unwrap();
        cache.set("b".to_string(), "3".to_string()).await.unwrap();

        assert_eq!(cache.get("a").await.unwrap(), Some("2".to_string()));
        assert_eq!(cache.get("b").await.unwrap(), Some("3".to_string()));
    }

    #[tokio::test]
    async fn rejects_entry_larger_than_limit_without_changing_existing_value() {
        let cache = MemoryCache::new(4);

        cache.set("a".to_string(), "1".to_string()).await.unwrap();

        let result = cache.set("a".to_string(), "large".to_string()).await;

        assert!(matches!(result, Err(KVStoreError::ReachLimit)));
        assert_eq!(cache.get("a").await.unwrap(), Some("1".to_string()));
    }
}
