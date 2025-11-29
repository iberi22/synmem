//! Domain entities for the scraper marketplace.

mod install;
mod package;
mod payout;
mod review;
mod schema_def;

pub use install::InstallRecord;
pub use package::{PricingModel, ScraperPackage, ScraperPackageMetadata, ScraperStatus};
pub use payout::{PayoutRecord, PayoutStatus};
pub use review::{Rating, Review};
pub use schema_def::{SchemaDefinition, SchemaField, SchemaFieldType};
