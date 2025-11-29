//! Review service - manages reviews and ratings.

use crate::domain::entities::{Rating, Review};
use crate::ports::outbound::ScraperRepository;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in review operations.
#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Review not found: {0}")]
    ReviewNotFound(Uuid),
    #[error("Package not found: {0}")]
    PackageNotFound(Uuid),
    #[error("User already reviewed this package")]
    AlreadyReviewed,
    #[error("Invalid rating: must be between 1 and 5")]
    InvalidRating,
    #[error("Content too short: minimum {0} characters")]
    ContentTooShort(usize),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

/// Result type for review operations.
pub type ReviewResult<T> = Result<T, ReviewError>;

/// Review service trait.
#[async_trait]
pub trait ReviewService: Send + Sync {
    /// Creates a new review.
    async fn create_review(
        &self,
        package_id: Uuid,
        reviewer: &str,
        rating: u8,
        content: &str,
        title: Option<&str>,
    ) -> ReviewResult<Review>;

    /// Gets a review by ID.
    async fn get_review(&self, id: Uuid) -> ReviewResult<Review>;

    /// Lists reviews for a package.
    async fn list_package_reviews(&self, package_id: Uuid) -> ReviewResult<Vec<Review>>;

    /// Gets reviews by a user.
    async fn list_user_reviews(&self, user: &str) -> ReviewResult<Vec<Review>>;

    /// Updates a review.
    async fn update_review(
        &self,
        id: Uuid,
        reviewer: &str,
        rating: Option<u8>,
        content: Option<&str>,
        title: Option<&str>,
    ) -> ReviewResult<Review>;

    /// Deletes a review.
    async fn delete_review(&self, id: Uuid, reviewer: &str) -> ReviewResult<()>;

    /// Marks a review as helpful.
    async fn mark_helpful(&self, id: Uuid) -> ReviewResult<Review>;

    /// Calculates average rating for a package.
    async fn get_average_rating(&self, package_id: Uuid) -> ReviewResult<Option<f32>>;
}

/// Default implementation of the review service.
pub struct DefaultReviewService<R: ScraperRepository> {
    repository: R,
    min_content_length: usize,
}

impl<R: ScraperRepository> DefaultReviewService<R> {
    /// Creates a new review service.
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            min_content_length: 10,
        }
    }

    /// Creates a review service with a custom minimum content length.
    pub fn with_min_content_length(repository: R, min_length: usize) -> Self {
        Self {
            repository,
            min_content_length: min_length,
        }
    }
}

#[async_trait]
impl<R: ScraperRepository + Send + Sync> ReviewService for DefaultReviewService<R> {
    async fn create_review(
        &self,
        package_id: Uuid,
        reviewer: &str,
        rating: u8,
        content: &str,
        title: Option<&str>,
    ) -> ReviewResult<Review> {
        // Validate rating
        let rating = Rating::try_from(rating).map_err(|_| ReviewError::InvalidRating)?;

        // Validate content length
        if content.len() < self.min_content_length {
            return Err(ReviewError::ContentTooShort(self.min_content_length));
        }

        // Check if user already reviewed
        let existing = self
            .repository
            .get_user_review(package_id, reviewer)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        if existing.is_some() {
            return Err(ReviewError::AlreadyReviewed);
        }

        // Check if user has installed the package
        let install = self
            .repository
            .get_install(package_id, reviewer)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        let mut review = Review::new(package_id, reviewer.to_string(), rating, content.to_string());

        if let Some(t) = title {
            review = review.with_title(t);
        }

        if install.is_some() {
            review = review.as_verified();
        }

        self.repository
            .save_review(&review)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        // Update package average rating
        self.update_package_rating(package_id).await?;

        Ok(review)
    }

    async fn get_review(&self, id: Uuid) -> ReviewResult<Review> {
        self.repository
            .get_review(id)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?
            .ok_or(ReviewError::ReviewNotFound(id))
    }

    async fn list_package_reviews(&self, package_id: Uuid) -> ReviewResult<Vec<Review>> {
        self.repository
            .list_package_reviews(package_id)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))
    }

    async fn list_user_reviews(&self, user: &str) -> ReviewResult<Vec<Review>> {
        self.repository
            .list_user_reviews(user)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))
    }

    async fn update_review(
        &self,
        id: Uuid,
        reviewer: &str,
        rating: Option<u8>,
        content: Option<&str>,
        title: Option<&str>,
    ) -> ReviewResult<Review> {
        let mut review = self.get_review(id).await?;

        if review.reviewer != reviewer {
            return Err(ReviewError::RepositoryError(
                "Only the reviewer can update their review".to_string(),
            ));
        }

        if let Some(r) = rating {
            review.rating = Rating::try_from(r).map_err(|_| ReviewError::InvalidRating)?;
        }

        if let Some(c) = content {
            if c.len() < self.min_content_length {
                return Err(ReviewError::ContentTooShort(self.min_content_length));
            }
            review.content = c.to_string();
        }

        if let Some(t) = title {
            review.title = Some(t.to_string());
        }

        review.updated_at = chrono::Utc::now();

        self.repository
            .save_review(&review)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        // Update package average rating
        self.update_package_rating(review.package_id).await?;

        Ok(review)
    }

    async fn delete_review(&self, id: Uuid, reviewer: &str) -> ReviewResult<()> {
        let review = self.get_review(id).await?;

        if review.reviewer != reviewer {
            return Err(ReviewError::RepositoryError(
                "Only the reviewer can delete their review".to_string(),
            ));
        }

        self.repository
            .delete_review(id)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        // Update package average rating
        self.update_package_rating(review.package_id).await?;

        Ok(())
    }

    async fn mark_helpful(&self, id: Uuid) -> ReviewResult<Review> {
        let mut review = self.get_review(id).await?;
        review.mark_helpful();

        self.repository
            .save_review(&review)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        Ok(review)
    }

    async fn get_average_rating(&self, package_id: Uuid) -> ReviewResult<Option<f32>> {
        let reviews = self.list_package_reviews(package_id).await?;

        if reviews.is_empty() {
            return Ok(None);
        }

        let sum: u32 = reviews.iter().map(|r| r.rating.value() as u32).sum();
        let avg = sum as f32 / reviews.len() as f32;

        Ok(Some(avg))
    }
}

impl<R: ScraperRepository + Send + Sync> DefaultReviewService<R> {
    /// Updates the average rating on the package metadata.
    async fn update_package_rating(&self, package_id: Uuid) -> ReviewResult<()> {
        let avg = self.get_average_rating(package_id).await?;
        let reviews = self.list_package_reviews(package_id).await?;

        // Get and update the package metadata
        let package = self
            .repository
            .get_by_id(package_id)
            .await
            .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;

        if let Some(mut pkg) = package {
            pkg.metadata.average_rating = avg;
            pkg.metadata.review_count = reviews.len() as u64;
            self.repository
                .save(&pkg)
                .await
                .map_err(|e| ReviewError::RepositoryError(e.to_string()))?;
        }

        Ok(())
    }
}
