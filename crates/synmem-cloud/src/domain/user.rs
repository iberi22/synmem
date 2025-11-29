//! User model for SynMem Cloud

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Tier;

/// A user account in SynMem Cloud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: Uuid,
    /// User's email address
    pub email: String,
    /// User's display name
    pub name: Option<String>,
    /// Subscription tier
    pub tier: Tier,
    /// External authentication provider ID (e.g., from Clerk/Auth0)
    pub external_auth_id: Option<String>,
    /// Stripe customer ID for billing
    pub stripe_customer_id: Option<String>,
    /// Current month's scrape count
    pub scrape_count: u32,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Creates a new free-tier user
    #[must_use]
    pub fn new(email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            name: None,
            tier: Tier::Free,
            external_auth_id: None,
            stripe_customer_id: None,
            scrape_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Checks if the user can perform another scrape based on their tier limits
    #[must_use]
    pub fn can_scrape(&self) -> bool {
        match self.tier.scrape_limit() {
            Some(limit) => self.scrape_count < limit,
            None => true, // Unlimited
        }
    }

    /// Returns the remaining scrapes for this billing period
    #[must_use]
    pub fn remaining_scrapes(&self) -> Option<u32> {
        self.tier
            .scrape_limit()
            .map(|limit| limit.saturating_sub(self.scrape_count))
    }

    /// Increments the scrape counter
    pub fn increment_scrape_count(&mut self) {
        self.scrape_count = self.scrape_count.saturating_add(1);
        self.updated_at = Utc::now();
    }

    /// Resets the scrape counter (typically at the start of a new billing period)
    pub fn reset_scrape_count(&mut self) {
        self.scrape_count = 0;
        self.updated_at = Utc::now();
    }

    /// Upgrades the user to a new tier
    pub fn upgrade_tier(&mut self, tier: Tier) {
        self.tier = tier;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new("test@example.com".to_string());
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.tier, Tier::Free);
        assert_eq!(user.scrape_count, 0);
        assert!(user.can_scrape());
    }

    #[test]
    fn test_scrape_limit_enforcement() {
        let mut user = User::new("test@example.com".to_string());
        user.scrape_count = 99;
        assert!(user.can_scrape());
        assert_eq!(user.remaining_scrapes(), Some(1));

        user.increment_scrape_count();
        assert!(!user.can_scrape());
        assert_eq!(user.remaining_scrapes(), Some(0));
    }

    #[test]
    fn test_pro_user_unlimited() {
        let mut user = User::new("pro@example.com".to_string());
        user.tier = Tier::Pro;
        user.scrape_count = 10000;
        assert!(user.can_scrape());
        assert_eq!(user.remaining_scrapes(), None);
    }

    #[test]
    fn test_reset_scrape_count() {
        let mut user = User::new("test@example.com".to_string());
        user.scrape_count = 50;
        user.reset_scrape_count();
        assert_eq!(user.scrape_count, 0);
        assert!(user.can_scrape());
    }
}
