//! Subscription metrics tracking.
//!
//! Tracks MRR, churn rate, and conversion funnel metrics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::tier::Tier;

/// Monthly Recurring Revenue (MRR) snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrrSnapshot {
    /// The date of this snapshot.
    pub date: DateTime<Utc>,
    /// Total MRR in USD cents.
    pub mrr_cents: u64,
    /// Number of paying customers.
    pub paying_customers: u32,
    /// MRR breakdown by tier.
    pub by_tier: MrrByTier,
}

/// MRR breakdown by subscription tier.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MrrByTier {
    /// MRR from Pro subscriptions in USD cents.
    pub pro_cents: u64,
    /// MRR from Enterprise subscriptions in USD cents.
    pub enterprise_cents: u64,
}

impl MrrSnapshot {
    /// Creates a new MRR snapshot.
    #[must_use]
    pub fn new(mrr_cents: u64, paying_customers: u32, by_tier: MrrByTier) -> Self {
        Self {
            date: Utc::now(),
            mrr_cents,
            paying_customers,
            by_tier,
        }
    }

    /// Returns MRR in dollars (as float).
    #[must_use]
    pub fn mrr_dollars(&self) -> f64 {
        self.mrr_cents as f64 / 100.0
    }

    /// Returns Average Revenue Per User (ARPU) in dollars.
    #[must_use]
    pub fn arpu_dollars(&self) -> f64 {
        if self.paying_customers == 0 {
            0.0
        } else {
            self.mrr_dollars() / f64::from(self.paying_customers)
        }
    }
}

/// Churn metrics for a given period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChurnMetrics {
    /// Period start date.
    pub period_start: DateTime<Utc>,
    /// Period end date.
    pub period_end: DateTime<Utc>,
    /// Customers at start of period.
    pub customers_start: u32,
    /// Customers at end of period.
    pub customers_end: u32,
    /// Customers churned during period.
    pub churned: u32,
    /// New customers acquired during period.
    pub acquired: u32,
}

impl ChurnMetrics {
    /// Creates new churn metrics.
    #[must_use]
    pub fn new(
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        customers_start: u32,
        churned: u32,
        acquired: u32,
    ) -> Self {
        let customers_end = customers_start.saturating_sub(churned) + acquired;
        Self {
            period_start,
            period_end,
            customers_start,
            customers_end,
            churned,
            acquired,
        }
    }

    /// Returns the churn rate as a percentage.
    #[must_use]
    pub fn churn_rate(&self) -> f64 {
        if self.customers_start == 0 {
            0.0
        } else {
            f64::from(self.churned) / f64::from(self.customers_start) * 100.0
        }
    }

    /// Returns the net growth (acquired - churned).
    #[must_use]
    pub fn net_growth(&self) -> i32 {
        self.acquired as i32 - self.churned as i32
    }

    /// Returns the retention rate as a percentage.
    #[must_use]
    pub fn retention_rate(&self) -> f64 {
        100.0 - self.churn_rate()
    }
}

/// Stage in the conversion funnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunnelStage {
    /// Visitor stage (viewed landing page).
    Visitor,
    /// Signed up for account.
    SignedUp,
    /// Started trial/free tier.
    StartedTrial,
    /// Activated (used core feature).
    Activated,
    /// Converted to paid.
    Converted,
}

impl std::fmt::Display for FunnelStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Visitor => write!(f, "visitor"),
            Self::SignedUp => write!(f, "signed_up"),
            Self::StartedTrial => write!(f, "started_trial"),
            Self::Activated => write!(f, "activated"),
            Self::Converted => write!(f, "converted"),
        }
    }
}

/// Conversion funnel metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionFunnel {
    /// Period start date.
    pub period_start: DateTime<Utc>,
    /// Period end date.
    pub period_end: DateTime<Utc>,
    /// Number of visitors.
    pub visitors: u32,
    /// Number of sign-ups.
    pub signups: u32,
    /// Number who started trial.
    pub trials_started: u32,
    /// Number who activated (used core feature).
    pub activated: u32,
    /// Number who converted to paid.
    pub converted: u32,
    /// Conversions by tier.
    pub conversions_by_tier: ConversionsByTier,
}

/// Conversions breakdown by tier.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversionsByTier {
    /// Conversions to Pro tier.
    pub pro: u32,
    /// Conversions to Enterprise tier.
    pub enterprise: u32,
}

impl ConversionFunnel {
    /// Creates a new conversion funnel.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        visitors: u32,
        signups: u32,
        trials_started: u32,
        activated: u32,
        converted: u32,
        conversions_by_tier: ConversionsByTier,
    ) -> Self {
        Self {
            period_start,
            period_end,
            visitors,
            signups,
            trials_started,
            activated,
            converted,
            conversions_by_tier,
        }
    }

    /// Returns conversion rate from stage A to stage B as percentage.
    #[must_use]
    pub fn conversion_rate(&self, from: FunnelStage, to: FunnelStage) -> f64 {
        let from_count = self.count_at_stage(from);
        let to_count = self.count_at_stage(to);

        if from_count == 0 {
            0.0
        } else {
            f64::from(to_count) / f64::from(from_count) * 100.0
        }
    }

    /// Returns the count at a specific funnel stage.
    #[must_use]
    pub fn count_at_stage(&self, stage: FunnelStage) -> u32 {
        match stage {
            FunnelStage::Visitor => self.visitors,
            FunnelStage::SignedUp => self.signups,
            FunnelStage::StartedTrial => self.trials_started,
            FunnelStage::Activated => self.activated,
            FunnelStage::Converted => self.converted,
        }
    }

    /// Returns overall visitor-to-paid conversion rate.
    #[must_use]
    pub fn overall_conversion_rate(&self) -> f64 {
        self.conversion_rate(FunnelStage::Visitor, FunnelStage::Converted)
    }

    /// Returns signup-to-paid conversion rate.
    #[must_use]
    pub fn signup_to_paid_rate(&self) -> f64 {
        self.conversion_rate(FunnelStage::SignedUp, FunnelStage::Converted)
    }
}

/// Event for tracking subscription changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionEvent {
    /// Unique event identifier.
    pub id: uuid::Uuid,
    /// Customer identifier.
    pub customer_id: uuid::Uuid,
    /// Type of event.
    pub event_type: SubscriptionEventType,
    /// Previous tier (if applicable).
    pub previous_tier: Option<Tier>,
    /// New tier (if applicable).
    pub new_tier: Option<Tier>,
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Additional metadata.
    pub metadata: Option<serde_json::Value>,
}

/// Types of subscription events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionEventType {
    /// Customer created new subscription.
    Created,
    /// Subscription upgraded to higher tier.
    Upgraded,
    /// Subscription downgraded to lower tier.
    Downgraded,
    /// Subscription was renewed.
    Renewed,
    /// Subscription was canceled.
    Canceled,
    /// Subscription expired.
    Expired,
    /// Subscription was reactivated.
    Reactivated,
    /// Payment failed.
    PaymentFailed,
    /// Payment succeeded.
    PaymentSucceeded,
}

impl SubscriptionEvent {
    /// Creates a new subscription event.
    #[must_use]
    pub fn new(
        customer_id: uuid::Uuid,
        event_type: SubscriptionEventType,
        previous_tier: Option<Tier>,
        new_tier: Option<Tier>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            customer_id,
            event_type,
            previous_tier,
            new_tier,
            timestamp: Utc::now(),
            metadata: None,
        }
    }

    /// Adds metadata to the event.
    #[must_use]
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mrr_snapshot() {
        let by_tier = MrrByTier {
            pro_cents: 19000,       // $190 from Pro
            enterprise_cents: 5000, // $50 from Enterprise
        };
        let snapshot = MrrSnapshot::new(24000, 12, by_tier);

        assert_eq!(snapshot.mrr_dollars(), 240.0);
        assert_eq!(snapshot.arpu_dollars(), 20.0);
    }

    #[test]
    fn test_churn_metrics() {
        let now = Utc::now();
        let metrics = ChurnMetrics::new(
            now - chrono::Duration::days(30),
            now,
            100, // started with 100 customers
            5,   // 5 churned
            10,  // 10 acquired
        );

        assert_eq!(metrics.churn_rate(), 5.0);
        assert_eq!(metrics.retention_rate(), 95.0);
        assert_eq!(metrics.net_growth(), 5);
        assert_eq!(metrics.customers_end, 105);
    }

    #[test]
    fn test_conversion_funnel() {
        let now = Utc::now();
        let conversions = ConversionsByTier {
            pro: 45,
            enterprise: 5,
        };
        let funnel = ConversionFunnel::new(
            now - chrono::Duration::days(30),
            now,
            10000, // visitors
            1000,  // signups (10% of visitors)
            500,   // trials (50% of signups)
            250,   // activated (50% of trials)
            50,    // converted (20% of activated)
            conversions,
        );

        assert_eq!(
            funnel.conversion_rate(FunnelStage::Visitor, FunnelStage::SignedUp),
            10.0
        );
        assert_eq!(
            funnel.conversion_rate(FunnelStage::SignedUp, FunnelStage::StartedTrial),
            50.0
        );
        assert_eq!(funnel.overall_conversion_rate(), 0.5);
        assert_eq!(funnel.signup_to_paid_rate(), 5.0);
    }

    #[test]
    fn test_subscription_event() {
        let customer_id = uuid::Uuid::new_v4();
        let event = SubscriptionEvent::new(
            customer_id,
            SubscriptionEventType::Upgraded,
            Some(Tier::Free),
            Some(Tier::Pro),
        );

        assert_eq!(event.customer_id, customer_id);
        assert_eq!(event.event_type, SubscriptionEventType::Upgraded);
        assert_eq!(event.previous_tier, Some(Tier::Free));
        assert_eq!(event.new_tier, Some(Tier::Pro));
    }

    #[test]
    fn test_funnel_stage_display() {
        assert_eq!(FunnelStage::Visitor.to_string(), "visitor");
        assert_eq!(FunnelStage::SignedUp.to_string(), "signed_up");
        assert_eq!(FunnelStage::Converted.to_string(), "converted");
    }
}
