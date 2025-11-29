//! Scraper package entity - the core marketplace item.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::payout::PayoutRecord;
use super::schema_def::SchemaDefinition;

/// Pricing model for scraper packages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PricingModel {
    /// Free community-contributed scraper
    Free,
    /// One-time purchase price in USD
    Paid { price: f64 },
}

impl Default for PricingModel {
    fn default() -> Self {
        Self::Free
    }
}

/// Status of a scraper package in the marketplace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScraperStatus {
    /// Submitted for review
    PendingReview,
    /// Approved and published
    Published,
    /// Rejected during review
    Rejected,
    /// Temporarily suspended
    Suspended,
    /// Deprecated by author
    Deprecated,
}

impl Default for ScraperStatus {
    fn default() -> Self {
        Self::PendingReview
    }
}

/// Metadata about a scraper package author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperPackageMetadata {
    /// Total number of installs
    pub install_count: u64,
    /// Average rating (1-5)
    pub average_rating: Option<f32>,
    /// Number of reviews
    pub review_count: u64,
    /// Date when the scraper was first published
    pub published_at: Option<DateTime<Utc>>,
    /// Date of last update
    pub updated_at: DateTime<Utc>,
}

impl Default for ScraperPackageMetadata {
    fn default() -> Self {
        Self {
            install_count: 0,
            average_rating: None,
            review_count: 0,
            published_at: None,
            updated_at: Utc::now(),
        }
    }
}

/// A scraper package available in the marketplace.
///
/// # Example
///
/// ```
/// use synmem_marketplace::{ScraperPackage, PricingModel, SchemaDefinition};
///
/// let package = ScraperPackage::new(
///     "linkedin-profile-scraper".to_string(),
///     "1.0.0".to_string(),
///     "alice".to_string(),
///     vec!["linkedin.com".to_string()],
///     "Extract profile data from LinkedIn".to_string(),
///     SchemaDefinition::default(),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperPackage {
    /// Unique identifier for the package
    pub id: Uuid,
    /// Package name (must be unique, lowercase, hyphen-separated)
    pub name: String,
    /// Semantic version (e.g., "1.0.0")
    pub version: String,
    /// Author's username
    pub author: String,
    /// Pricing model
    pub pricing: PricingModel,
    /// List of supported sites/domains
    pub sites: Vec<String>,
    /// Human-readable description
    pub description: String,
    /// Output schema definition
    pub schema: SchemaDefinition,
    /// Package status
    pub status: ScraperStatus,
    /// Additional metadata
    pub metadata: ScraperPackageMetadata,
    /// Optional repository URL
    pub repository_url: Option<String>,
    /// Optional documentation URL
    pub documentation_url: Option<String>,
    /// Optional list of tags for discovery
    pub tags: Vec<String>,
}

impl ScraperPackage {
    /// Creates a new scraper package with default values.
    pub fn new(
        name: String,
        version: String,
        author: String,
        sites: Vec<String>,
        description: String,
        schema: SchemaDefinition,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            version,
            author,
            pricing: PricingModel::default(),
            sites,
            description,
            schema,
            status: ScraperStatus::default(),
            metadata: ScraperPackageMetadata::default(),
            repository_url: None,
            documentation_url: None,
            tags: Vec::new(),
        }
    }

    /// Creates a new paid scraper package.
    pub fn new_paid(
        name: String,
        version: String,
        author: String,
        sites: Vec<String>,
        description: String,
        schema: SchemaDefinition,
        price: f64,
    ) -> Self {
        let mut package = Self::new(name, version, author, sites, description, schema);
        package.pricing = PricingModel::Paid { price };
        package
    }

    /// Returns the price if the package is paid, otherwise None.
    pub fn price(&self) -> Option<f64> {
        match &self.pricing {
            PricingModel::Free => None,
            PricingModel::Paid { price } => Some(*price),
        }
    }

    /// Returns true if the package is free.
    pub fn is_free(&self) -> bool {
        matches!(self.pricing, PricingModel::Free)
    }

    /// Returns true if the package is published and available.
    pub fn is_available(&self) -> bool {
        self.status == ScraperStatus::Published
    }

    /// Calculates creator payout for a purchase (70% revenue share).
    pub fn creator_payout(&self) -> Option<f64> {
        self.price().map(|p| p * PayoutRecord::CREATOR_SHARE)
    }

    /// Calculates platform fee for a purchase (30% revenue share).
    pub fn platform_fee(&self) -> Option<f64> {
        self.price().map(|p| p * PayoutRecord::PLATFORM_SHARE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SchemaDefinition;

    #[test]
    fn test_new_free_scraper() {
        let package = ScraperPackage::new(
            "test-scraper".to_string(),
            "1.0.0".to_string(),
            "alice".to_string(),
            vec!["example.com".to_string()],
            "A test scraper".to_string(),
            SchemaDefinition::default(),
        );

        assert!(package.is_free());
        assert_eq!(package.price(), None);
        assert_eq!(package.status, ScraperStatus::PendingReview);
        assert!(!package.is_available());
    }

    #[test]
    fn test_new_paid_scraper() {
        let package = ScraperPackage::new_paid(
            "premium-scraper".to_string(),
            "2.0.0".to_string(),
            "bob".to_string(),
            vec!["linkedin.com".to_string()],
            "A premium scraper".to_string(),
            SchemaDefinition::default(),
            5.00,
        );

        assert!(!package.is_free());
        assert_eq!(package.price(), Some(5.00));
    }

    #[test]
    fn test_revenue_split() {
        let package = ScraperPackage::new_paid(
            "paid-scraper".to_string(),
            "1.0.0".to_string(),
            "creator".to_string(),
            vec!["site.com".to_string()],
            "Paid scraper".to_string(),
            SchemaDefinition::default(),
            10.00,
        );

        // 70% to creator, 30% to platform
        assert_eq!(package.creator_payout(), Some(7.00));
        assert_eq!(package.platform_fee(), Some(3.00));
    }
}
