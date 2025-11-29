//! # SynMem Marketplace
//!
//! Community scraper marketplace for SynMem.
//!
//! ## Features
//!
//! - **Scraper Packages**: Define and distribute site-specific scrapers
//! - **Revenue Model**: Free scrapers (community) and paid scrapers (70% to creator, 30% to SynMem)
//! - **Review System**: Ratings and reviews for quality assurance
//! - **Install Tracking**: Track scraper installations and usage
//!
//! ## Package Format
//!
//! ```json
//! {
//!   "name": "linkedin-profile-scraper",
//!   "version": "1.0.0",
//!   "author": "username",
//!   "price": 5.00,
//!   "sites": ["linkedin.com"],
//!   "description": "Extract profile data from LinkedIn",
//!   "schema": {
//!     "output": {
//!       "name": "string",
//!       "headline": "string",
//!       "experience": "array"
//!     }
//!   }
//! }
//! ```

pub mod domain;
pub mod ports;
pub mod schema;

// Re-exports for convenience
pub use domain::entities::{
    InstallRecord, PayoutRecord, PayoutStatus, PricingModel, Rating, Review, ScraperPackage,
    ScraperPackageMetadata, ScraperStatus, SchemaDefinition, SchemaField, SchemaFieldType,
};
pub use domain::services::{MarketplaceService, PayoutService, ReviewService};
pub use ports::inbound::{MarketplaceCommands, MarketplaceQueries};
pub use ports::outbound::{PayoutGateway, ScraperRepository};
