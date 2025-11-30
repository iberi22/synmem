//! Rate limiter for Twitter API calls
//!
//! Implements a token bucket algorithm to enforce rate limits
//! and prevent account suspension.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use super::TwitterError;

/// Rate limits for different Twitter operations
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Window duration in seconds
    pub window_seconds: u64,
    /// Minimum delay between requests in milliseconds
    pub min_delay_ms: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 50,
            window_seconds: 900, // 15 minutes
            min_delay_ms: 1000,  // 1 second between requests
        }
    }
}

impl RateLimitConfig {
    /// Configuration for posting tweets (more restrictive)
    pub fn for_post() -> Self {
        Self {
            max_requests: 25,
            window_seconds: 900,
            min_delay_ms: 2000,
        }
    }

    /// Configuration for reading operations (less restrictive)
    pub fn for_read() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 900,
            min_delay_ms: 500,
        }
    }

    /// Configuration for search operations
    pub fn for_search() -> Self {
        Self {
            max_requests: 50,
            window_seconds: 900,
            min_delay_ms: 1000,
        }
    }
}

/// Rate limiter using token bucket algorithm
#[derive(Debug)]
pub struct RateLimiter {
    config: RateLimitConfig,
    tokens: AtomicU64,
    last_request: Arc<Mutex<Instant>>,
    window_start: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            tokens: AtomicU64::new(config.max_requests as u64),
            last_request: Arc::new(Mutex::new(Instant::now())),
            window_start: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Create a rate limiter with default configuration
    pub fn default_limiter() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Acquire permission to make a request
    ///
    /// This will wait if necessary to comply with rate limits.
    /// Returns an error if rate limit would be exceeded.
    pub async fn acquire(&self) -> Result<(), TwitterError> {
        // Check and refill tokens if window has passed
        self.refill_if_needed().await;

        // Try to consume a token using compare-and-swap for thread safety
        loop {
            let current = self.tokens.load(Ordering::SeqCst);
            if current == 0 {
                let window_start = self.window_start.lock().await;
                let elapsed = window_start.elapsed().as_secs();
                let wait_seconds = self.config.window_seconds.saturating_sub(elapsed);
                return Err(TwitterError::RateLimited { wait_seconds });
            }

            // Atomically try to decrement the token count
            match self
                .tokens
                .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
            {
                Ok(_) => break,
                Err(_) => continue, // Another thread modified it, retry
            }
        }

        // Enforce minimum delay between requests
        let mut last_request = self.last_request.lock().await;
        let elapsed = last_request.elapsed();
        let min_delay = Duration::from_millis(self.config.min_delay_ms);

        if elapsed < min_delay {
            let wait_time = min_delay - elapsed;
            tokio::time::sleep(wait_time).await;
        }

        *last_request = Instant::now();
        Ok(())
    }

    /// Check if tokens should be refilled
    async fn refill_if_needed(&self) {
        let mut window_start = self.window_start.lock().await;
        let elapsed = window_start.elapsed();

        if elapsed >= Duration::from_secs(self.config.window_seconds) {
            // Reset the window
            *window_start = Instant::now();
            self.tokens
                .store(self.config.max_requests as u64, Ordering::SeqCst);
        }
    }

    /// Get the number of remaining tokens
    pub fn remaining_tokens(&self) -> u64 {
        self.tokens.load(Ordering::SeqCst)
    }

    /// Check if a request can be made without waiting
    pub fn can_proceed(&self) -> bool {
        self.tokens.load(Ordering::SeqCst) > 0
    }

    /// Get the current rate limit configuration
    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::default_limiter()
    }
}
