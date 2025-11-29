//! Review and rating entities for the marketplace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Rating value (1-5 stars).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rating(u8);

impl Rating {
    /// Creates a new rating from a value.
    ///
    /// # Panics
    ///
    /// Panics if the value is not between 1 and 5.
    pub fn new(value: u8) -> Self {
        assert!((1..=5).contains(&value), "Rating must be between 1 and 5");
        Self(value)
    }

    /// Creates a rating, clamping the value between 1 and 5.
    pub fn clamped(value: u8) -> Self {
        Self(value.clamp(1, 5))
    }

    /// Returns the rating value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self(5)
    }
}

impl From<Rating> for u8 {
    fn from(rating: Rating) -> Self {
        rating.0
    }
}

impl TryFrom<u8> for Rating {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (1..=5).contains(&value) {
            Ok(Self(value))
        } else {
            Err("Rating must be between 1 and 5")
        }
    }
}

/// A review for a scraper package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Unique identifier
    pub id: Uuid,
    /// ID of the package being reviewed
    pub package_id: Uuid,
    /// Username of the reviewer
    pub reviewer: String,
    /// Rating (1-5 stars)
    pub rating: Rating,
    /// Review title
    pub title: Option<String>,
    /// Review content
    pub content: String,
    /// Whether the reviewer verified they installed the package
    pub verified_install: bool,
    /// Date the review was created
    pub created_at: DateTime<Utc>,
    /// Date the review was last updated
    pub updated_at: DateTime<Utc>,
    /// Number of users who found this review helpful
    pub helpful_count: u32,
}

impl Review {
    /// Creates a new review.
    pub fn new(package_id: Uuid, reviewer: String, rating: Rating, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            package_id,
            reviewer,
            rating,
            title: None,
            content,
            verified_install: false,
            created_at: now,
            updated_at: now,
            helpful_count: 0,
        }
    }

    /// Creates a review with a title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Marks the review as a verified install.
    pub fn as_verified(mut self) -> Self {
        self.verified_install = true;
        self
    }

    /// Increments the helpful count.
    pub fn mark_helpful(&mut self) {
        self.helpful_count += 1;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_creation() {
        let rating = Rating::new(5);
        assert_eq!(rating.value(), 5);

        let rating = Rating::clamped(10);
        assert_eq!(rating.value(), 5);

        let rating = Rating::clamped(0);
        assert_eq!(rating.value(), 1);
    }

    #[test]
    #[should_panic(expected = "Rating must be between 1 and 5")]
    fn test_rating_invalid() {
        Rating::new(6);
    }

    #[test]
    fn test_rating_try_from() {
        assert!(Rating::try_from(3).is_ok());
        assert!(Rating::try_from(0).is_err());
        assert!(Rating::try_from(6).is_err());
    }

    #[test]
    fn test_review_creation() {
        let package_id = Uuid::new_v4();
        let review = Review::new(
            package_id,
            "alice".to_string(),
            Rating::new(4),
            "Great scraper!".to_string(),
        )
        .with_title("Excellent tool")
        .as_verified();

        assert_eq!(review.rating.value(), 4);
        assert_eq!(review.title, Some("Excellent tool".to_string()));
        assert!(review.verified_install);
    }

    #[test]
    fn test_review_helpful() {
        let mut review = Review::new(
            Uuid::new_v4(),
            "bob".to_string(),
            Rating::new(5),
            "Amazing!".to_string(),
        );

        assert_eq!(review.helpful_count, 0);
        review.mark_helpful();
        assert_eq!(review.helpful_count, 1);
    }
}
