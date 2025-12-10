// Rate Limiter for Package Registry API
// Provides token bucket rate limiting per IP/user

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst: u32,
}

impl RateLimit {
    pub fn new(requests_per_minute: u32, burst: u32) -> Self {
        Self {
            requests_per_minute,
            burst,
        }
    }

    pub fn default_api() -> Self {
        Self::new(100, 10) // 100 req/min, burst of 10
    }

    pub fn default_download() -> Self {
        Self::new(50, 5) // 50 downloads/min, burst of 5
    }

    pub fn default_upload() -> Self {
        Self::new(10, 2) // 10 uploads/min, burst of 2
    }
}

#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate_per_minute: u32) -> Self {
        Self {
            tokens: capacity as f64,
            last_refill: Instant::now(),
            capacity: capacity as f64,
            refill_rate: refill_rate_per_minute as f64 / 60.0,
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;

        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill = now;
    }

    fn try_consume(&mut self, count: f64) -> bool {
        self.refill();

        if self.tokens >= count {
            self.tokens -= count;
            true
        } else {
            false
        }
    }

    fn available(&mut self) -> u32 {
        self.refill();
        self.tokens.floor() as u32
    }
}

pub struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    default_limit: RateLimit,
}

impl RateLimiter {
    pub fn new(default_limit: RateLimit) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            default_limit,
        }
    }

    /// Check if request is allowed for given key (IP or user ID)
    pub async fn check_rate_limit(&self, key: &str) -> Result<(), RateLimitError> {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(
                self.default_limit.burst,
                self.default_limit.requests_per_minute,
            )
        });

        if bucket.try_consume(1.0) {
            Ok(())
        } else {
            Err(RateLimitError {
                retry_after: Duration::from_secs(60),
                limit: self.default_limit.requests_per_minute,
            })
        }
    }

    /// Get remaining requests for key
    pub async fn get_remaining(&self, key: &str) -> u32 {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(
                self.default_limit.burst,
                self.default_limit.requests_per_minute,
            )
        });

        bucket.available()
    }

    /// Reset rate limit for key
    pub async fn reset(&self, key: &str) {
        let mut buckets = self.buckets.write().await;
        buckets.remove(key);
    }

    /// Clean up old buckets (call periodically)
    pub async fn cleanup(&self) {
        let mut buckets = self.buckets.write().await;
        buckets.retain(|_, bucket| {
            // Keep buckets that were accessed in last hour
            bucket.last_refill.elapsed() < Duration::from_secs(3600)
        });
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitError {
    pub retry_after: Duration,
    pub limit: u32,
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rate limit exceeded. Limit: {} requests/minute. Retry after: {:?}",
            self.limit, self.retry_after
        )
    }
}

impl std::error::Error for RateLimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_allows_within_limit() {
        let limiter = RateLimiter::new(RateLimit::new(100, 10));

        // Should allow first 10 requests (burst)
        for _ in 0..10 {
            assert!(limiter.check_rate_limit("test-ip").await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limit_blocks_over_limit() {
        let limiter = RateLimiter::new(RateLimit::new(100, 5));

        // Consume burst
        for _ in 0..5 {
            limiter.check_rate_limit("test-ip").await.unwrap();
        }

        // Next request should fail
        assert!(limiter.check_rate_limit("test-ip").await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limit_refills() {
        let limiter = RateLimiter::new(RateLimit::new(60, 2)); // 1 per second

        // Consume burst
        limiter.check_rate_limit("test-ip").await.unwrap();
        limiter.check_rate_limit("test-ip").await.unwrap();

        // Should fail immediately
        assert!(limiter.check_rate_limit("test-ip").await.is_err());

        // Wait for refill
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Should succeed after refill
        assert!(limiter.check_rate_limit("test-ip").await.is_ok());
    }

    #[tokio::test]
    async fn test_get_remaining() {
        let limiter = RateLimiter::new(RateLimit::new(100, 10));

        let remaining = limiter.get_remaining("test-ip").await;
        assert_eq!(remaining, 10);

        limiter.check_rate_limit("test-ip").await.unwrap();

        let remaining = limiter.get_remaining("test-ip").await;
        assert_eq!(remaining, 9);
    }

    #[tokio::test]
    async fn test_reset() {
        let limiter = RateLimiter::new(RateLimit::new(100, 2));

        // Consume all
        limiter.check_rate_limit("test-ip").await.unwrap();
        limiter.check_rate_limit("test-ip").await.unwrap();

        // Should fail
        assert!(limiter.check_rate_limit("test-ip").await.is_err());

        // Reset
        limiter.reset("test-ip").await;

        // Should succeed after reset
        assert!(limiter.check_rate_limit("test-ip").await.is_ok());
    }
}
