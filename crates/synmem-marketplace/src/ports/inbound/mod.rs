//! Inbound ports - interfaces for driving adapters (API, CLI, etc.).

use crate::domain::entities::{InstallRecord, PayoutRecord, Review, ScraperPackage};
use async_trait::async_trait;
use uuid::Uuid;

/// Commands for marketplace operations (write operations).
#[async_trait]
pub trait MarketplaceCommands: Send + Sync {
    /// Error type for command operations.
    type Error: std::error::Error + Send + Sync + 'static;

    // Package commands
    /// Submits a new scraper package for review.
    async fn submit_package(&self, package: ScraperPackage) -> Result<ScraperPackage, Self::Error>;

    /// Updates an existing package (creates new version).
    async fn update_package(
        &self,
        id: Uuid,
        author: &str,
        package: ScraperPackage,
    ) -> Result<ScraperPackage, Self::Error>;

    /// Approves a package for publishing.
    async fn approve_package(&self, id: Uuid) -> Result<ScraperPackage, Self::Error>;

    /// Rejects a package submission.
    async fn reject_package(&self, id: Uuid, reason: &str) -> Result<ScraperPackage, Self::Error>;

    /// Deprecates a package.
    async fn deprecate_package(
        &self,
        id: Uuid,
        author: &str,
    ) -> Result<ScraperPackage, Self::Error>;

    // Install commands
    /// Installs a package for a user.
    async fn install_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> Result<InstallRecord, Self::Error>;

    /// Uninstalls a package for a user.
    async fn uninstall_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> Result<InstallRecord, Self::Error>;

    // Review commands
    /// Creates a new review for a package.
    async fn create_review(
        &self,
        package_id: Uuid,
        reviewer: &str,
        rating: u8,
        content: &str,
        title: Option<&str>,
    ) -> Result<Review, Self::Error>;

    /// Updates an existing review.
    async fn update_review(
        &self,
        id: Uuid,
        reviewer: &str,
        rating: Option<u8>,
        content: Option<&str>,
        title: Option<&str>,
    ) -> Result<Review, Self::Error>;

    /// Deletes a review.
    async fn delete_review(&self, id: Uuid, reviewer: &str) -> Result<(), Self::Error>;

    /// Marks a review as helpful.
    async fn mark_review_helpful(&self, id: Uuid) -> Result<Review, Self::Error>;

    // Payout commands
    /// Records a sale and creates a pending payout.
    async fn record_sale(
        &self,
        package_id: Uuid,
        buyer: &str,
        amount: f64,
    ) -> Result<PayoutRecord, Self::Error>;

    /// Processes a pending payout.
    async fn process_payout(&self, id: Uuid) -> Result<PayoutRecord, Self::Error>;

    /// Cancels a pending payout.
    async fn cancel_payout(&self, id: Uuid, reason: &str) -> Result<PayoutRecord, Self::Error>;
}

/// Queries for marketplace operations (read operations).
#[async_trait]
pub trait MarketplaceQueries: Send + Sync {
    /// Error type for query operations.
    type Error: std::error::Error + Send + Sync + 'static;

    // Package queries
    /// Gets a package by ID.
    async fn get_package(&self, id: Uuid) -> Result<Option<ScraperPackage>, Self::Error>;

    /// Gets a package by name.
    async fn get_package_by_name(&self, name: &str) -> Result<Option<ScraperPackage>, Self::Error>;

    /// Lists published packages with pagination.
    async fn list_packages(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ScraperPackage>, Self::Error>;

    /// Searches packages by query string.
    async fn search_packages(&self, query: &str) -> Result<Vec<ScraperPackage>, Self::Error>;

    /// Lists packages by author.
    async fn list_packages_by_author(
        &self,
        author: &str,
    ) -> Result<Vec<ScraperPackage>, Self::Error>;

    /// Lists packages by supported site.
    async fn list_packages_by_site(&self, site: &str) -> Result<Vec<ScraperPackage>, Self::Error>;

    /// Lists packages pending review.
    async fn list_pending_packages(&self) -> Result<Vec<ScraperPackage>, Self::Error>;

    // Install queries
    /// Gets user's installed packages.
    async fn list_user_installs(&self, user: &str) -> Result<Vec<InstallRecord>, Self::Error>;

    /// Gets install count for a package.
    async fn get_install_count(&self, package_id: Uuid) -> Result<u64, Self::Error>;

    // Review queries
    /// Gets a review by ID.
    async fn get_review(&self, id: Uuid) -> Result<Option<Review>, Self::Error>;

    /// Lists reviews for a package.
    async fn list_package_reviews(&self, package_id: Uuid) -> Result<Vec<Review>, Self::Error>;

    /// Lists reviews by a user.
    async fn list_user_reviews(&self, user: &str) -> Result<Vec<Review>, Self::Error>;

    /// Gets average rating for a package.
    async fn get_average_rating(&self, package_id: Uuid) -> Result<Option<f32>, Self::Error>;

    // Payout queries
    /// Gets a payout by ID.
    async fn get_payout(&self, id: Uuid) -> Result<Option<PayoutRecord>, Self::Error>;

    /// Lists payouts for a creator.
    async fn list_creator_payouts(&self, creator: &str) -> Result<Vec<PayoutRecord>, Self::Error>;

    /// Gets total earnings for a creator.
    async fn get_total_earnings(&self, creator: &str) -> Result<f64, Self::Error>;

    /// Gets pending earnings for a creator.
    async fn get_pending_earnings(&self, creator: &str) -> Result<f64, Self::Error>;
}
