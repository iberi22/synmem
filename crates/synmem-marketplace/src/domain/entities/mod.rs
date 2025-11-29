//! Domain entities for the scraper marketplace.

mod install;
mod payout;
mod schema_def;
mod package;
mod review;

pub use install::InstallRecord;
pub use package::{PricingModel, ScraperPackage, ScraperPackageMetadata, ScraperStatus};
pub use payout::{PayoutRecord, PayoutStatus};
pub use review::{Rating, Review};
pub use schema_def::{SchemaDefinition, SchemaField, SchemaFieldType};
