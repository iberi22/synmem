//! Domain layer for the subscription system.
//!
//! Contains core entities and business logic.

pub mod license;
pub mod metrics;
pub mod subscription;
pub mod tier;

pub use license::{License, LicenseKeyPair};
pub use metrics::{
    ChurnMetrics, ConversionFunnel, ConversionsByTier, FunnelStage, MrrByTier, MrrSnapshot,
    SubscriptionEvent, SubscriptionEventType,
};
pub use subscription::{Customer, Subscription, SubscriptionStatus};
pub use tier::Tier;
