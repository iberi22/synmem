//! Outbound port for payment processing (Stripe integration).

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::Tier;
use crate::error::Result;

/// Request to create a checkout session.
#[derive(Debug, Clone)]
pub struct CreateCheckoutRequest {
    /// Customer ID.
    pub customer_id: Uuid,
    /// Target subscription tier.
    pub tier: Tier,
    /// Success redirect URL.
    pub success_url: String,
    /// Cancel redirect URL.
    pub cancel_url: String,
}

/// Response from creating a checkout session.
#[derive(Debug, Clone)]
pub struct CheckoutSession {
    /// Checkout session ID.
    pub id: String,
    /// URL to redirect the user to.
    pub url: String,
}

/// Request to create a customer portal session.
#[derive(Debug, Clone)]
pub struct CreatePortalRequest {
    /// Customer ID.
    pub customer_id: Uuid,
    /// Return URL after portal visit.
    pub return_url: String,
}

/// Response from creating a portal session.
#[derive(Debug, Clone)]
pub struct PortalSession {
    /// Portal session ID.
    pub id: String,
    /// URL to redirect the user to.
    pub url: String,
}

/// Webhook event from the payment provider.
#[derive(Debug, Clone)]
pub struct WebhookEvent {
    /// Event ID.
    pub id: String,
    /// Event type (e.g., "checkout.session.completed").
    pub event_type: String,
    /// Raw event payload.
    pub payload: serde_json::Value,
}

/// Port for payment provider integration.
#[async_trait]
pub trait PaymentPort: Send + Sync {
    /// Creates a customer in the payment provider.
    async fn create_customer(&self, email: &str) -> Result<String>;

    /// Creates a checkout session for subscription purchase.
    async fn create_checkout_session(&self, request: CreateCheckoutRequest)
        -> Result<CheckoutSession>;

    /// Creates a customer portal session for managing subscription.
    async fn create_portal_session(&self, request: CreatePortalRequest) -> Result<PortalSession>;

    /// Verifies a webhook signature and parses the event.
    async fn verify_webhook(&self, payload: &[u8], signature: &str) -> Result<WebhookEvent>;

    /// Cancels a subscription in the payment provider.
    async fn cancel_subscription(&self, stripe_subscription_id: &str) -> Result<()>;

    /// Retrieves subscription details from payment provider.
    async fn get_subscription(&self, stripe_subscription_id: &str) -> Result<StripeSubscription>;
}

/// Subscription data from Stripe.
#[derive(Debug, Clone)]
pub struct StripeSubscription {
    /// Stripe subscription ID.
    pub id: String,
    /// Subscription status.
    pub status: String,
    /// Current period start timestamp.
    pub current_period_start: i64,
    /// Current period end timestamp.
    pub current_period_end: i64,
    /// Whether subscription cancels at period end.
    pub cancel_at_period_end: bool,
}
