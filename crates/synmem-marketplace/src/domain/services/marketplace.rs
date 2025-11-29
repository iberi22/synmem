//! Marketplace service - core business logic for scraper marketplace.

use crate::domain::entities::{InstallRecord, ScraperPackage, ScraperStatus};
use crate::ports::outbound::ScraperRepository;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in marketplace operations.
#[derive(Debug, Error)]
pub enum MarketplaceError {
    #[error("Package not found: {0}")]
    PackageNotFound(Uuid),
    #[error("Package already exists: {0}")]
    PackageAlreadyExists(String),
    #[error("Package is not available: {0}")]
    PackageNotAvailable(Uuid),
    #[error("Invalid package: {0}")]
    InvalidPackage(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

/// Result type for marketplace operations.
pub type MarketplaceResult<T> = Result<T, MarketplaceError>;

/// Marketplace service trait defining core marketplace operations.
#[async_trait]
pub trait MarketplaceService: Send + Sync {
    /// Submits a new scraper package for review.
    async fn submit_package(&self, package: ScraperPackage) -> MarketplaceResult<ScraperPackage>;

    /// Gets a package by ID.
    async fn get_package(&self, id: Uuid) -> MarketplaceResult<ScraperPackage>;

    /// Gets a package by name.
    async fn get_package_by_name(&self, name: &str) -> MarketplaceResult<ScraperPackage>;

    /// Lists all published packages.
    async fn list_packages(&self, limit: usize, offset: usize)
        -> MarketplaceResult<Vec<ScraperPackage>>;

    /// Searches packages by query.
    async fn search_packages(&self, query: &str) -> MarketplaceResult<Vec<ScraperPackage>>;

    /// Lists packages by author.
    async fn list_by_author(&self, author: &str) -> MarketplaceResult<Vec<ScraperPackage>>;

    /// Lists packages for a specific site.
    async fn list_by_site(&self, site: &str) -> MarketplaceResult<Vec<ScraperPackage>>;

    /// Updates a package (new version).
    async fn update_package(
        &self,
        id: Uuid,
        author: &str,
        package: ScraperPackage,
    ) -> MarketplaceResult<ScraperPackage>;

    /// Approves a package (admin only).
    async fn approve_package(&self, id: Uuid) -> MarketplaceResult<ScraperPackage>;

    /// Rejects a package (admin only).
    async fn reject_package(&self, id: Uuid, reason: &str) -> MarketplaceResult<ScraperPackage>;

    /// Deprecates a package.
    async fn deprecate_package(&self, id: Uuid, author: &str) -> MarketplaceResult<ScraperPackage>;

    /// Installs a package for a user.
    async fn install_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> MarketplaceResult<InstallRecord>;

    /// Uninstalls a package for a user.
    async fn uninstall_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> MarketplaceResult<InstallRecord>;

    /// Lists installed packages for a user.
    async fn list_user_installs(&self, user: &str) -> MarketplaceResult<Vec<InstallRecord>>;
}

/// Default implementation of the marketplace service.
pub struct DefaultMarketplaceService<R: ScraperRepository> {
    repository: R,
}

impl<R: ScraperRepository> DefaultMarketplaceService<R> {
    /// Creates a new marketplace service.
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Validates a package before submission.
    fn validate_package(package: &ScraperPackage) -> MarketplaceResult<()> {
        if package.name.is_empty() {
            return Err(MarketplaceError::InvalidPackage(
                "Package name cannot be empty".to_string(),
            ));
        }

        if !package
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        {
            return Err(MarketplaceError::InvalidPackage(
                "Package name must be lowercase with hyphens only".to_string(),
            ));
        }

        if package.version.is_empty() {
            return Err(MarketplaceError::InvalidPackage(
                "Version cannot be empty".to_string(),
            ));
        }

        if package.sites.is_empty() {
            return Err(MarketplaceError::InvalidPackage(
                "At least one site must be specified".to_string(),
            ));
        }

        if package.description.is_empty() {
            return Err(MarketplaceError::InvalidPackage(
                "Description cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl<R: ScraperRepository + Send + Sync> MarketplaceService for DefaultMarketplaceService<R> {
    async fn submit_package(&self, mut package: ScraperPackage) -> MarketplaceResult<ScraperPackage> {
        Self::validate_package(&package)?;

        // Check if package name already exists
        if self
            .repository
            .get_by_name(&package.name)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?
            .is_some()
        {
            return Err(MarketplaceError::PackageAlreadyExists(package.name));
        }

        // Set initial status
        package.status = ScraperStatus::PendingReview;

        self.repository
            .save(&package)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(package)
    }

    async fn get_package(&self, id: Uuid) -> MarketplaceResult<ScraperPackage> {
        self.repository
            .get_by_id(id)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?
            .ok_or(MarketplaceError::PackageNotFound(id))
    }

    async fn get_package_by_name(&self, name: &str) -> MarketplaceResult<ScraperPackage> {
        self.repository
            .get_by_name(name)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?
            .ok_or_else(|| {
                MarketplaceError::InvalidPackage(format!("Package not found: {}", name))
            })
    }

    async fn list_packages(
        &self,
        limit: usize,
        offset: usize,
    ) -> MarketplaceResult<Vec<ScraperPackage>> {
        self.repository
            .list_published(limit, offset)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))
    }

    async fn search_packages(&self, query: &str) -> MarketplaceResult<Vec<ScraperPackage>> {
        self.repository
            .search(query)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))
    }

    async fn list_by_author(&self, author: &str) -> MarketplaceResult<Vec<ScraperPackage>> {
        self.repository
            .list_by_author(author)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))
    }

    async fn list_by_site(&self, site: &str) -> MarketplaceResult<Vec<ScraperPackage>> {
        self.repository
            .list_by_site(site)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))
    }

    async fn update_package(
        &self,
        id: Uuid,
        author: &str,
        mut package: ScraperPackage,
    ) -> MarketplaceResult<ScraperPackage> {
        let existing = self.get_package(id).await?;

        if existing.author != author {
            return Err(MarketplaceError::PermissionDenied(
                "Only the author can update a package".to_string(),
            ));
        }

        Self::validate_package(&package)?;

        package.id = id;
        package.status = ScraperStatus::PendingReview;

        self.repository
            .save(&package)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(package)
    }

    async fn approve_package(&self, id: Uuid) -> MarketplaceResult<ScraperPackage> {
        let mut package = self.get_package(id).await?;
        package.status = ScraperStatus::Published;
        package.metadata.published_at = Some(chrono::Utc::now());

        self.repository
            .save(&package)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(package)
    }

    async fn reject_package(&self, id: Uuid, _reason: &str) -> MarketplaceResult<ScraperPackage> {
        let mut package = self.get_package(id).await?;
        package.status = ScraperStatus::Rejected;

        self.repository
            .save(&package)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(package)
    }

    async fn deprecate_package(&self, id: Uuid, author: &str) -> MarketplaceResult<ScraperPackage> {
        let mut package = self.get_package(id).await?;

        if package.author != author {
            return Err(MarketplaceError::PermissionDenied(
                "Only the author can deprecate a package".to_string(),
            ));
        }

        package.status = ScraperStatus::Deprecated;

        self.repository
            .save(&package)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(package)
    }

    async fn install_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> MarketplaceResult<InstallRecord> {
        let package = self.get_package(package_id).await?;

        if !package.is_available() {
            return Err(MarketplaceError::PackageNotAvailable(package_id));
        }

        let record = InstallRecord::new(package_id, user.to_string(), package.version.clone());

        self.repository
            .record_install(&record)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(record)
    }

    async fn uninstall_package(
        &self,
        package_id: Uuid,
        user: &str,
    ) -> MarketplaceResult<InstallRecord> {
        let mut record = self
            .repository
            .get_install(package_id, user)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?
            .ok_or(MarketplaceError::PackageNotFound(package_id))?;

        record.uninstall();

        self.repository
            .record_install(&record)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))?;

        Ok(record)
    }

    async fn list_user_installs(&self, user: &str) -> MarketplaceResult<Vec<InstallRecord>> {
        self.repository
            .list_user_installs(user)
            .await
            .map_err(|e| MarketplaceError::RepositoryError(e.to_string()))
    }
}
