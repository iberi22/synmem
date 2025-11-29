//! Subscription tier definitions for SynMem.
//!
//! Defines the available subscription tiers: Free, Pro, and Enterprise.

use serde::{Deserialize, Serialize};

/// Subscription tier levels available in SynMem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// Free tier with limited features.
    #[default]
    Free,
    /// Pro tier with full features.
    Pro,
    /// Enterprise tier with custom pricing and dedicated support.
    Enterprise,
}

impl Tier {
    /// Returns the monthly price in USD cents for this tier.
    /// Returns `None` for Enterprise (custom pricing).
    #[must_use]
    pub const fn price_cents(&self) -> Option<u32> {
        match self {
            Self::Free => Some(0),
            Self::Pro => Some(1900), // $19.00
            Self::Enterprise => None,
        }
    }

    /// Returns the maximum scrapes per month for this tier.
    /// Returns `None` for unlimited scrapes.
    #[must_use]
    pub const fn scrapes_per_month(&self) -> Option<u32> {
        match self {
            Self::Free => Some(100),
            Self::Pro | Self::Enterprise => None,
        }
    }

    /// Returns whether cloud sessions are enabled for this tier.
    #[must_use]
    pub const fn cloud_sessions(&self) -> bool {
        matches!(self, Self::Pro | Self::Enterprise)
    }

    /// Returns whether API access is enabled for this tier.
    #[must_use]
    pub const fn api_access(&self) -> bool {
        matches!(self, Self::Pro | Self::Enterprise)
    }

    /// Returns whether dedicated support is available for this tier.
    #[must_use]
    pub const fn dedicated_support(&self) -> bool {
        matches!(self, Self::Enterprise)
    }

    /// Returns whether SLA is available for this tier.
    #[must_use]
    pub const fn sla(&self) -> bool {
        matches!(self, Self::Enterprise)
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "free"),
            Self::Pro => write!(f, "pro"),
            Self::Enterprise => write!(f, "enterprise"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_prices() {
        assert_eq!(Tier::Free.price_cents(), Some(0));
        assert_eq!(Tier::Pro.price_cents(), Some(1900));
        assert_eq!(Tier::Enterprise.price_cents(), None);
    }

    #[test]
    fn test_tier_scrapes() {
        assert_eq!(Tier::Free.scrapes_per_month(), Some(100));
        assert_eq!(Tier::Pro.scrapes_per_month(), None);
        assert_eq!(Tier::Enterprise.scrapes_per_month(), None);
    }

    #[test]
    fn test_tier_features() {
        // Free tier
        assert!(!Tier::Free.cloud_sessions());
        assert!(!Tier::Free.api_access());
        assert!(!Tier::Free.dedicated_support());
        assert!(!Tier::Free.sla());

        // Pro tier
        assert!(Tier::Pro.cloud_sessions());
        assert!(Tier::Pro.api_access());
        assert!(!Tier::Pro.dedicated_support());
        assert!(!Tier::Pro.sla());

        // Enterprise tier
        assert!(Tier::Enterprise.cloud_sessions());
        assert!(Tier::Enterprise.api_access());
        assert!(Tier::Enterprise.dedicated_support());
        assert!(Tier::Enterprise.sla());
    }

    #[test]
    fn test_tier_serialization() {
        let tier = Tier::Pro;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, "\"pro\"");

        let parsed: Tier = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Tier::Pro);
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(Tier::Free.to_string(), "free");
        assert_eq!(Tier::Pro.to_string(), "pro");
        assert_eq!(Tier::Enterprise.to_string(), "enterprise");
    }
}
