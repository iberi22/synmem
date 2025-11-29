//! Outbound port for subscription storage.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Customer, License, Subscription, SubscriptionEvent};
use crate::error::Result;

/// Port for storing and retrieving subscription data.
#[async_trait]
pub trait SubscriptionStoragePort: Send + Sync {
    // Customer operations
    /// Creates a new customer.
    async fn create_customer(&self, customer: &Customer) -> Result<()>;

    /// Retrieves a customer by ID.
    async fn get_customer(&self, id: Uuid) -> Result<Option<Customer>>;

    /// Retrieves a customer by email.
    async fn get_customer_by_email(&self, email: &str) -> Result<Option<Customer>>;

    /// Retrieves a customer by Stripe customer ID.
    async fn get_customer_by_stripe_id(&self, stripe_id: &str) -> Result<Option<Customer>>;

    /// Updates a customer.
    async fn update_customer(&self, customer: &Customer) -> Result<()>;

    // Subscription operations
    /// Creates a new subscription.
    async fn create_subscription(&self, subscription: &Subscription) -> Result<()>;

    /// Retrieves a subscription by ID.
    async fn get_subscription(&self, id: Uuid) -> Result<Option<Subscription>>;

    /// Retrieves subscriptions for a customer.
    async fn get_customer_subscriptions(&self, customer_id: Uuid) -> Result<Vec<Subscription>>;

    /// Retrieves the active subscription for a customer.
    async fn get_active_subscription(&self, customer_id: Uuid) -> Result<Option<Subscription>>;

    /// Updates a subscription.
    async fn update_subscription(&self, subscription: &Subscription) -> Result<()>;

    // License operations
    /// Stores a license.
    async fn store_license(&self, license: &License) -> Result<()>;

    /// Retrieves a license by key.
    async fn get_license(&self, key: &str) -> Result<Option<License>>;

    /// Retrieves licenses for a customer.
    async fn get_customer_licenses(&self, customer_id: Uuid) -> Result<Vec<License>>;

    /// Revokes a license.
    async fn revoke_license(&self, key: &str) -> Result<()>;

    // Event operations
    /// Records a subscription event.
    async fn record_event(&self, event: &SubscriptionEvent) -> Result<()>;

    /// Retrieves events for a customer.
    async fn get_customer_events(
        &self,
        customer_id: Uuid,
        limit: Option<u32>,
    ) -> Result<Vec<SubscriptionEvent>>;
}
