//! Customer and subscription entities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::tier::Tier;

/// A customer in the subscription system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Unique customer identifier.
    pub id: Uuid,
    /// Customer email address.
    pub email: String,
    /// Stripe customer ID (if connected).
    pub stripe_customer_id: Option<String>,
    /// When the customer was created.
    pub created_at: DateTime<Utc>,
    /// When the customer was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Customer {
    /// Creates a new customer with the given email.
    #[must_use]
    pub fn new(email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            stripe_customer_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a customer with an existing Stripe customer ID.
    #[must_use]
    pub fn with_stripe_id(email: String, stripe_customer_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            stripe_customer_id: Some(stripe_customer_id),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Status of a subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription is active and paid.
    Active,
    /// Subscription is in trial period.
    Trialing,
    /// Payment failed, subscription at risk.
    PastDue,
    /// Subscription has been cancelled.
    Canceled,
    /// Subscription has been paused.
    Paused,
    /// Subscription ended (not renewed).
    Expired,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Trialing => write!(f, "trialing"),
            Self::PastDue => write!(f, "past_due"),
            Self::Canceled => write!(f, "canceled"),
            Self::Paused => write!(f, "paused"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

/// A subscription for a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique subscription identifier.
    pub id: Uuid,
    /// Customer ID this subscription belongs to.
    pub customer_id: Uuid,
    /// Subscription tier.
    pub tier: Tier,
    /// Current status of the subscription.
    pub status: SubscriptionStatus,
    /// Stripe subscription ID (if applicable).
    pub stripe_subscription_id: Option<String>,
    /// When the current period started.
    pub current_period_start: DateTime<Utc>,
    /// When the current period ends.
    pub current_period_end: DateTime<Utc>,
    /// Whether the subscription will cancel at period end.
    pub cancel_at_period_end: bool,
    /// When the subscription was created.
    pub created_at: DateTime<Utc>,
    /// When the subscription was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    /// Creates a new subscription for a customer.
    #[must_use]
    pub fn new(
        customer_id: Uuid,
        tier: Tier,
        current_period_start: DateTime<Utc>,
        current_period_end: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            customer_id,
            tier,
            status: SubscriptionStatus::Active,
            stripe_subscription_id: None,
            current_period_start,
            current_period_end,
            cancel_at_period_end: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a free tier subscription.
    #[must_use]
    pub fn free(customer_id: Uuid) -> Self {
        let now = Utc::now();
        // Free tier has no expiration, set far future date
        let far_future = now + chrono::Duration::days(365 * 100);
        Self::new(customer_id, Tier::Free, now, far_future)
    }

    /// Returns whether this subscription is currently active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            SubscriptionStatus::Active | SubscriptionStatus::Trialing
        )
    }

    /// Returns whether this subscription can access premium features.
    #[must_use]
    pub fn has_premium_access(&self) -> bool {
        self.is_active() && matches!(self.tier, Tier::Pro | Tier::Enterprise)
    }

    /// Returns the number of scrapes remaining in the current period.
    /// Returns `None` for unlimited.
    #[must_use]
    pub fn scrapes_limit(&self) -> Option<u32> {
        self.tier.scrapes_per_month()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_customer_creation() {
        let customer = Customer::new("test@example.com".to_string());
        assert_eq!(customer.email, "test@example.com");
        assert!(customer.stripe_customer_id.is_none());
    }

    #[test]
    fn test_customer_with_stripe() {
        let customer =
            Customer::with_stripe_id("test@example.com".to_string(), "cus_123".to_string());
        assert_eq!(customer.stripe_customer_id, Some("cus_123".to_string()));
    }

    #[test]
    fn test_subscription_creation() {
        let customer = Customer::new("test@example.com".to_string());
        let now = Utc::now();
        let period_end = now + Duration::days(30);

        let subscription = Subscription::new(customer.id, Tier::Pro, now, period_end);
        assert_eq!(subscription.tier, Tier::Pro);
        assert!(subscription.is_active());
        assert!(subscription.has_premium_access());
        assert_eq!(subscription.scrapes_limit(), None); // Pro has unlimited
    }

    #[test]
    fn test_free_subscription() {
        let customer = Customer::new("test@example.com".to_string());
        let subscription = Subscription::free(customer.id);

        assert_eq!(subscription.tier, Tier::Free);
        assert!(subscription.is_active());
        assert!(!subscription.has_premium_access());
        assert_eq!(subscription.scrapes_limit(), Some(100));
    }

    #[test]
    fn test_subscription_status() {
        let customer = Customer::new("test@example.com".to_string());
        let now = Utc::now();
        let period_end = now + Duration::days(30);

        let mut subscription = Subscription::new(customer.id, Tier::Pro, now, period_end);

        assert!(subscription.is_active());

        subscription.status = SubscriptionStatus::Canceled;
        assert!(!subscription.is_active());
        assert!(!subscription.has_premium_access());
    }

    #[test]
    fn test_subscription_status_display() {
        assert_eq!(SubscriptionStatus::Active.to_string(), "active");
        assert_eq!(SubscriptionStatus::PastDue.to_string(), "past_due");
        assert_eq!(SubscriptionStatus::Canceled.to_string(), "canceled");
    }
}
