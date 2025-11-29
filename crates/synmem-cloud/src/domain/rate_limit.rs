//! Rate limiting for SynMem Cloud

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

use super::Tier;

/// Rate limit tracking for a user or API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// The entity being rate limited (user ID or API key ID)
    pub entity_id: Uuid,
    /// Timestamps of recent requests within the window
    pub request_timestamps: VecDeque<DateTime<Utc>>,
    /// Maximum requests per minute based on tier
    pub limit_per_minute: u32,
    /// Time window for rate limiting
    pub window: Duration,
}

impl RateLimit {
    /// Creates a new rate limiter for a user tier
    #[must_use]
    pub fn new(entity_id: Uuid, tier: Tier) -> Self {
        Self {
            entity_id,
            request_timestamps: VecDeque::new(),
            limit_per_minute: tier.rate_limit_per_minute(),
            window: Duration::minutes(1),
        }
    }

    /// Creates a custom rate limiter with specific limits
    #[must_use]
    pub fn with_custom_limit(entity_id: Uuid, limit_per_minute: u32) -> Self {
        Self {
            entity_id,
            request_timestamps: VecDeque::new(),
            limit_per_minute,
            window: Duration::minutes(1),
        }
    }

    /// Checks if a request is allowed under the rate limit
    #[must_use]
    pub fn is_allowed(&self) -> bool {
        let count = self.current_request_count();
        count < self.limit_per_minute
    }

    /// Records a new request and returns whether it was allowed
    pub fn record_request(&mut self) -> bool {
        self.cleanup_old_requests();

        if self.is_allowed() {
            self.request_timestamps.push_back(Utc::now());
            true
        } else {
            false
        }
    }

    /// Returns the current request count within the window
    #[must_use]
    pub fn current_request_count(&self) -> u32 {
        let cutoff = Utc::now() - self.window;
        self.request_timestamps
            .iter()
            .filter(|&&ts| ts > cutoff)
            .count() as u32
    }

    /// Returns the number of remaining requests in the current window
    #[must_use]
    pub fn remaining_requests(&self) -> u32 {
        self.limit_per_minute.saturating_sub(self.current_request_count())
    }

    /// Returns when the rate limit will reset (oldest request expires)
    #[must_use]
    pub fn reset_at(&self) -> Option<DateTime<Utc>> {
        self.request_timestamps
            .front()
            .map(|&ts| ts + self.window)
    }

    /// Cleans up old requests outside the time window
    fn cleanup_old_requests(&mut self) {
        let cutoff = Utc::now() - self.window;
        while let Some(&ts) = self.request_timestamps.front() {
            if ts <= cutoff {
                self.request_timestamps.pop_front();
            } else {
                break;
            }
        }
    }

    /// Updates the rate limit based on a new tier
    pub fn update_tier(&mut self, tier: Tier) {
        self.limit_per_minute = tier.rate_limit_per_minute();
    }
}

/// Response headers for rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitHeaders {
    /// Maximum requests allowed
    pub limit: u32,
    /// Remaining requests in current window
    pub remaining: u32,
    /// Unix timestamp when the window resets
    pub reset: i64,
}

impl From<&RateLimit> for RateLimitHeaders {
    fn from(rate_limit: &RateLimit) -> Self {
        Self {
            limit: rate_limit.limit_per_minute,
            remaining: rate_limit.remaining_requests(),
            reset: rate_limit
                .reset_at()
                .map(|dt| dt.timestamp())
                .unwrap_or_else(|| Utc::now().timestamp() + 60),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_creation() {
        let entity_id = Uuid::new_v4();
        let rate_limit = RateLimit::new(entity_id, Tier::Free);

        assert_eq!(rate_limit.limit_per_minute, 10);
        assert!(rate_limit.is_allowed());
        assert_eq!(rate_limit.remaining_requests(), 10);
    }

    #[test]
    fn test_rate_limit_enforcement() {
        let entity_id = Uuid::new_v4();
        let mut rate_limit = RateLimit::new(entity_id, Tier::Free);

        // Make 10 requests (should all succeed)
        for _ in 0..10 {
            assert!(rate_limit.record_request());
        }

        // 11th request should fail
        assert!(!rate_limit.record_request());
        assert_eq!(rate_limit.remaining_requests(), 0);
    }

    #[test]
    fn test_pro_tier_limits() {
        let entity_id = Uuid::new_v4();
        let rate_limit = RateLimit::new(entity_id, Tier::Pro);

        assert_eq!(rate_limit.limit_per_minute, 100);
        assert_eq!(rate_limit.remaining_requests(), 100);
    }

    #[test]
    fn test_enterprise_tier_limits() {
        let entity_id = Uuid::new_v4();
        let rate_limit = RateLimit::new(entity_id, Tier::Enterprise);

        assert_eq!(rate_limit.limit_per_minute, 1000);
    }

    #[test]
    fn test_rate_limit_headers() {
        let entity_id = Uuid::new_v4();
        let rate_limit = RateLimit::new(entity_id, Tier::Free);
        let headers = RateLimitHeaders::from(&rate_limit);

        assert_eq!(headers.limit, 10);
        assert_eq!(headers.remaining, 10);
    }

    #[test]
    fn test_tier_upgrade() {
        let entity_id = Uuid::new_v4();
        let mut rate_limit = RateLimit::new(entity_id, Tier::Free);

        assert_eq!(rate_limit.limit_per_minute, 10);

        rate_limit.update_tier(Tier::Pro);
        assert_eq!(rate_limit.limit_per_minute, 100);
    }
}
