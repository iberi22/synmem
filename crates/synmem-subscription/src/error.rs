//! Error types for the subscription system.

use chrono::{DateTime, Utc};
use thiserror::Error;

/// Result type for subscription operations.
pub type Result<T> = std::result::Result<T, SubscriptionError>;

/// Errors that can occur in the subscription system.
#[derive(Debug, Error)]
pub enum SubscriptionError {
    /// License validation failed.
    #[error("License validation failed: {0}")]
    LicenseValidation(String),

    /// License has expired.
    #[error("License expired at {0}")]
    LicenseExpired(DateTime<Utc>),

    /// Stripe API error.
    #[error("Stripe API error: {0}")]
    StripeApi(String),

    /// Webhook signature verification failed.
    #[error("Webhook signature verification failed: {0}")]
    WebhookSignature(String),

    /// Customer not found.
    #[error("Customer not found: {0}")]
    CustomerNotFound(String),

    /// Subscription not found.
    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(String),

    /// Invalid subscription operation.
    #[error("Invalid subscription operation: {0}")]
    InvalidOperation(String),

    /// Payment processing error.
    #[error("Payment processing error: {0}")]
    PaymentError(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Storage error.
    #[error("Storage error: {0}")]
    Storage(String),
}
