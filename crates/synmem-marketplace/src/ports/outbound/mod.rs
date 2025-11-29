//! Outbound ports - interfaces for driven adapters (database, payment gateway, etc.).

use crate::domain::entities::{InstallRecord, PayoutRecord, Review, ScraperPackage};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

/// Repository errors.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Repository for scraper packages and related entities.
#[async_trait]
pub trait ScraperRepository: Send + Sync {
    // Package operations
    /// Saves a scraper package.
    async fn save(&self, package: &ScraperPackage) -> Result<(), RepositoryError>;

    /// Gets a package by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ScraperPackage>, RepositoryError>;

    /// Gets a package by name.
    async fn get_by_name(&self, name: &str) -> Result<Option<ScraperPackage>, RepositoryError>;

    /// Lists published packages with pagination.
    async fn list_published(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ScraperPackage>, RepositoryError>;

    /// Searches packages by query.
    async fn search(&self, query: &str) -> Result<Vec<ScraperPackage>, RepositoryError>;

    /// Lists packages by author.
    async fn list_by_author(&self, author: &str) -> Result<Vec<ScraperPackage>, RepositoryError>;

    /// Lists packages by site.
    async fn list_by_site(&self, site: &str) -> Result<Vec<ScraperPackage>, RepositoryError>;

    /// Lists packages pending review.
    async fn list_pending(&self) -> Result<Vec<ScraperPackage>, RepositoryError>;

    /// Deletes a package.
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;

    // Install operations
    /// Records an installation.
    async fn record_install(&self, record: &InstallRecord) -> Result<(), RepositoryError>;

    /// Gets an install record for a user and package.
    async fn get_install(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> Result<Option<InstallRecord>, RepositoryError>;

    /// Lists installs for a user.
    async fn list_user_installs(&self, user: &str) -> Result<Vec<InstallRecord>, RepositoryError>;

    /// Gets install count for a package.
    async fn get_install_count(&self, package_id: Uuid) -> Result<u64, RepositoryError>;

    // Review operations
    /// Saves a review.
    async fn save_review(&self, review: &Review) -> Result<(), RepositoryError>;

    /// Gets a review by ID.
    async fn get_review(&self, id: Uuid) -> Result<Option<Review>, RepositoryError>;

    /// Gets a user's review for a package.
    async fn get_user_review(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> Result<Option<Review>, RepositoryError>;

    /// Lists reviews for a package.
    async fn list_package_reviews(
        &self,
        package_id: Uuid,
    ) -> Result<Vec<Review>, RepositoryError>;

    /// Lists reviews by a user.
    async fn list_user_reviews(&self, user: &str) -> Result<Vec<Review>, RepositoryError>;

    /// Deletes a review.
    async fn delete_review(&self, id: Uuid) -> Result<(), RepositoryError>;

    // Payout operations
    /// Saves a payout record.
    async fn save_payout(&self, payout: &PayoutRecord) -> Result<(), RepositoryError>;

    /// Gets a payout by ID.
    async fn get_payout(&self, id: Uuid) -> Result<Option<PayoutRecord>, RepositoryError>;

    /// Lists payouts for a creator.
    async fn list_creator_payouts(
        &self,
        creator: &str,
    ) -> Result<Vec<PayoutRecord>, RepositoryError>;
}

/// Payment gateway errors.
#[derive(Debug, Error)]
pub enum PayoutGatewayError {
    #[error("Invalid recipient: {0}")]
    InvalidRecipient(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Payment failed: {0}")]
    PaymentFailed(String),
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Payment gateway for processing payouts.
#[async_trait]
pub trait PayoutGateway: Send + Sync {
    /// Processes a payout to a creator.
    ///
    /// Returns the transaction ID on success.
    async fn process_payout(
        &self,
        recipient: &str,
        amount: f64,
    ) -> Result<String, PayoutGatewayError>;

    /// Gets the payout status for a transaction.
    async fn get_payout_status(&self, transaction_id: &str) -> Result<String, PayoutGatewayError>;

    /// Validates a recipient can receive payouts.
    async fn validate_recipient(&self, recipient: &str) -> Result<bool, PayoutGatewayError>;
}
