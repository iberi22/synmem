//! Inbound port for subscription service.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Customer, License, Subscription, SubscriptionEvent, Tier};
use crate::error::Result;
use crate::ports::payment::{CheckoutSession, PortalSession};

/// Request to create a subscription checkout.
#[derive(Debug, Clone)]
pub struct CheckoutRequest {
    /// Customer email.
    pub email: String,
    /// Target subscription tier.
    pub tier: Tier,
    /// Success redirect URL.
    pub success_url: String,
    /// Cancel redirect URL.
    pub cancel_url: String,
}

/// Request to generate a license key.
#[derive(Debug, Clone)]
pub struct GenerateLicenseRequest {
    /// Customer ID.
    pub customer_id: Uuid,
    /// Tier to grant.
    pub tier: Tier,
    /// License duration in days.
    pub duration_days: u32,
}

/// Inbound port for subscription operations.
#[async_trait]
pub trait SubscriptionServicePort: Send + Sync {
    // Customer operations
    /// Creates or retrieves a customer by email.
    async fn get_or_create_customer(&self, email: &str) -> Result<Customer>;

    /// Gets a customer by ID.
    async fn get_customer(&self, id: Uuid) -> Result<Option<Customer>>;

    // Checkout operations
    /// Creates a checkout session for subscription purchase.
    async fn create_checkout(&self, request: CheckoutRequest) -> Result<CheckoutSession>;

    /// Creates a customer portal session.
    async fn create_portal(&self, customer_id: Uuid, return_url: &str) -> Result<PortalSession>;

    // Subscription operations
    /// Gets the current subscription for a customer.
    async fn get_subscription(&self, customer_id: Uuid) -> Result<Option<Subscription>>;

    /// Cancels a subscription.
    async fn cancel_subscription(&self, customer_id: Uuid) -> Result<()>;

    /// Handles a webhook event from the payment provider.
    async fn handle_webhook(&self, payload: &[u8], signature: &str) -> Result<()>;

    // License operations
    /// Generates a license key for offline validation.
    async fn generate_license(&self, request: GenerateLicenseRequest) -> Result<License>;

    /// Validates a license key.
    async fn validate_license(&self, license_key: &str) -> Result<License>;

    // Feature access
    /// Checks if a customer has access to a feature.
    async fn has_feature_access(&self, customer_id: Uuid, feature: &str) -> Result<bool>;

    /// Gets the remaining scrapes for a customer this period.
    async fn get_remaining_scrapes(&self, customer_id: Uuid) -> Result<Option<u32>>;

    // Events
    /// Records a subscription event.
    async fn record_event(&self, event: SubscriptionEvent) -> Result<()>;

    /// Gets recent events for a customer.
    async fn get_events(&self, customer_id: Uuid, limit: Option<u32>)
        -> Result<Vec<SubscriptionEvent>>;
}
