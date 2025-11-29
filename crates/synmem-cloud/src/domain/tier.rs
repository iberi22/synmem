//! Subscription tier definitions for SynMem Cloud

use serde::{Deserialize, Serialize};

/// Subscription tier defining feature limits and access levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// Free tier: 100 scrapes/month, local only
    #[default]
    Free,
    /// Pro tier: Unlimited scrapes, cloud sessions, API access
    Pro,
    /// Enterprise tier: Custom limits, SLA, dedicated support
    Enterprise,
}

impl Tier {
    /// Returns the monthly scrape limit for this tier
    ///
    /// # Returns
    /// - `Some(limit)` for tiers with a fixed limit
    /// - `None` for unlimited tiers
    #[must_use]
    pub const fn scrape_limit(&self) -> Option<u32> {
        match self {
            Self::Free => Some(100),
            Self::Pro | Self::Enterprise => None, // Unlimited
        }
    }

    /// Returns whether cloud sessions are available for this tier
    #[must_use]
    pub const fn has_cloud_sessions(&self) -> bool {
        matches!(self, Self::Pro | Self::Enterprise)
    }

    /// Returns whether API access is available for this tier
    #[must_use]
    pub const fn has_api_access(&self) -> bool {
        matches!(self, Self::Pro | Self::Enterprise)
    }

    /// Returns whether this tier has SLA guarantees
    #[must_use]
    pub const fn has_sla(&self) -> bool {
        matches!(self, Self::Enterprise)
    }

    /// Returns whether this tier has dedicated support
    #[must_use]
    pub const fn has_dedicated_support(&self) -> bool {
        matches!(self, Self::Enterprise)
    }

    /// Returns the maximum concurrent browser sessions for this tier
    #[must_use]
    pub const fn max_concurrent_sessions(&self) -> u32 {
        match self {
            Self::Free => 0,      // Local only
            Self::Pro => 5,       // 5 concurrent cloud sessions
            Self::Enterprise => 50, // 50 concurrent cloud sessions
        }
    }

    /// Returns the rate limit (requests per minute) for this tier
    #[must_use]
    pub const fn rate_limit_per_minute(&self) -> u32 {
        match self {
            Self::Free => 10,
            Self::Pro => 100,
            Self::Enterprise => 1000,
        }
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "Free"),
            Self::Pro => write!(f, "Pro"),
            Self::Enterprise => write!(f, "Enterprise"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_tier_limits() {
        let tier = Tier::Free;
        assert_eq!(tier.scrape_limit(), Some(100));
        assert!(!tier.has_cloud_sessions());
        assert!(!tier.has_api_access());
        assert!(!tier.has_sla());
        assert_eq!(tier.max_concurrent_sessions(), 0);
        assert_eq!(tier.rate_limit_per_minute(), 10);
    }

    #[test]
    fn test_pro_tier_limits() {
        let tier = Tier::Pro;
        assert_eq!(tier.scrape_limit(), None);
        assert!(tier.has_cloud_sessions());
        assert!(tier.has_api_access());
        assert!(!tier.has_sla());
        assert_eq!(tier.max_concurrent_sessions(), 5);
        assert_eq!(tier.rate_limit_per_minute(), 100);
    }

    #[test]
    fn test_enterprise_tier_limits() {
        let tier = Tier::Enterprise;
        assert_eq!(tier.scrape_limit(), None);
        assert!(tier.has_cloud_sessions());
        assert!(tier.has_api_access());
        assert!(tier.has_sla());
        assert!(tier.has_dedicated_support());
        assert_eq!(tier.max_concurrent_sessions(), 50);
        assert_eq!(tier.rate_limit_per_minute(), 1000);
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(Tier::Free.to_string(), "Free");
        assert_eq!(Tier::Pro.to_string(), "Pro");
        assert_eq!(Tier::Enterprise.to_string(), "Enterprise");
    }

    #[test]
    fn test_tier_serialization() {
        assert_eq!(
            serde_json::to_string(&Tier::Free).unwrap(),
            r#""free""#
        );
        assert_eq!(
            serde_json::to_string(&Tier::Pro).unwrap(),
            r#""pro""#
        );
        assert_eq!(
            serde_json::to_string(&Tier::Enterprise).unwrap(),
            r#""enterprise""#
        );
    }
}
