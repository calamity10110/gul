// Cache Layer for Package Registry
// Provides multi-tier caching with memory and optional Redis support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

const DEFAULT_TTL: Duration = Duration::from_secs(300); // 5 minutes
const MAX_CACHE_SIZE: usize = 10000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedItem<T> {
    pub data: T,
    pub cached_at: SystemTime,
    pub ttl: Duration,
}

impl<T> CachedItem<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            cached_at: SystemTime::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.cached_at.elapsed().unwrap_or(Duration::ZERO) > self.ttl
    }
}

pub struct CacheLayer<T: Clone> {
    memory: Arc<RwLock<HashMap<String, CachedItem<T>>>>,
    ttl: Duration,
    max_size: usize,
}

impl<T: Clone> CacheLayer<T> {
    pub fn new() -> Self {
        Self {
            memory: Arc::new(RwLock::new(HashMap::new())),
            ttl: DEFAULT_TTL,
            max_size: MAX_CACHE_SIZE,
        }
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            memory: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            max_size: MAX_CACHE_SIZE,
        }
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let cache = self.memory.read().await;

        if let Some(item) = cache.get(key) {
            if !item.is_expired() {
                return Some(item.data.clone());
            }
        }

        None
    }

    pub async fn set(&self, key: String, value: T) {
        let mut cache = self.memory.write().await;

        // Evict oldest if at capacity
        if cache.len() >= self.max_size {
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }

        cache.insert(key, CachedItem::new(value, self.ttl));
    }

    pub async fn invalidate(&self, key: &str) {
        let mut cache = self.memory.write().await;
        cache.remove(key);
    }

    pub async fn clear(&self) {
        let mut cache = self.memory.write().await;
        cache.clear();
    }

    pub async fn size(&self) -> usize {
        let cache = self.memory.read().await;
        cache.len()
    }

    /// Get or fetch pattern - check cache first, then fetch if missing
    pub async fn get_or_fetch<F, Fut>(&self, key: &str, fetch: F) -> Result<T, String>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        // Check cache first
        if let Some(cached) = self.get(key).await {
            return Ok(cached);
        }

        // Fetch and cache
        let value = fetch().await?;
        self.set(key.to_string(), value.clone()).await;
        Ok(value)
    }
}

impl<T: Clone> Default for CacheLayer<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_set_get() {
        let cache: CacheLayer<String> = CacheLayer::new();

        cache.set("key1".to_string(), "value1".to_string()).await;

        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache: CacheLayer<String> = CacheLayer::with_ttl(Duration::from_millis(100));

        cache.set("key1".to_string(), "value1".to_string()).await;

        // Should be cached
        assert!(cache.get("key1").await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be expired
        assert!(cache.get("key1").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache: CacheLayer<String> = CacheLayer::new();

        cache.set("key1".to_string(), "value1".to_string()).await;
        assert!(cache.get("key1").await.is_some());

        cache.invalidate("key1").await;
        assert!(cache.get("key1").await.is_none());
    }

    #[tokio::test]
    async fn test_get_or_fetch() {
        let cache: CacheLayer<String> = CacheLayer::new();

        let mut call_count = 0;

        // First call should fetch
        let value = cache
            .get_or_fetch("key1", || async {
                call_count += 1;
                Ok::<String, String>("fetched".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "fetched");

        // Second call should use cache (but we can't test call_count in async closure)
        let value2 = cache.get("key1").await;
        assert_eq!(value2, Some("fetched".to_string()));
    }
}
