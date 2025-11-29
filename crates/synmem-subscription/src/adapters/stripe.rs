//! Stripe payment adapter.
//!
//! Implements the PaymentPort interface for Stripe integration.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::domain::Tier;
use crate::error::{Result, SubscriptionError};
use crate::ports::payment::{
    CheckoutSession, CreateCheckoutRequest, CreatePortalRequest, PaymentPort, PortalSession,
    StripeSubscription, WebhookEvent,
};

/// Configuration for the Stripe adapter.
#[derive(Debug, Clone)]
pub struct StripeConfig {
    /// Stripe API key (secret key).
    pub api_key: String,
    /// Stripe webhook signing secret.
    pub webhook_secret: String,
    /// Price ID for Pro tier.
    pub pro_price_id: String,
    /// Price ID for Enterprise tier (if applicable).
    pub enterprise_price_id: Option<String>,
}

impl StripeConfig {
    /// Creates a new Stripe configuration.
    #[must_use]
    pub fn new(api_key: String, webhook_secret: String, pro_price_id: String) -> Self {
        Self {
            api_key,
            webhook_secret,
            pro_price_id,
            enterprise_price_id: None,
        }
    }

    /// Sets the Enterprise price ID.
    #[must_use]
    pub fn with_enterprise_price(mut self, price_id: String) -> Self {
        self.enterprise_price_id = Some(price_id);
        self
    }

    /// Gets the price ID for a tier.
    pub fn price_id_for_tier(&self, tier: Tier) -> Result<&str> {
        match tier {
            Tier::Free => Err(SubscriptionError::InvalidOperation(
                "Free tier does not have a Stripe price".to_string(),
            )),
            Tier::Pro => Ok(&self.pro_price_id),
            Tier::Enterprise => self.enterprise_price_id.as_deref().ok_or_else(|| {
                SubscriptionError::Configuration("Enterprise price ID not configured".to_string())
            }),
        }
    }
}

/// Stripe payment adapter.
///
/// This is a mock implementation that demonstrates the interface.
/// In production, this would use the actual Stripe API.
pub struct StripeAdapter {
    config: StripeConfig,
}

impl StripeAdapter {
    /// Creates a new Stripe adapter.
    #[must_use]
    pub fn new(config: StripeConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl PaymentPort for StripeAdapter {
    async fn create_customer(&self, email: &str) -> Result<String> {
        info!("Creating Stripe customer for email: {}", email);

        // In production, this would call:
        // POST https://api.stripe.com/v1/customers
        // with email parameter

        // Mock implementation - generate a fake customer ID
        let customer_id = format!("cus_{}", &uuid::Uuid::new_v4().to_string().replace('-', "")[..14]);

        debug!("Created Stripe customer: {}", customer_id);
        Ok(customer_id)
    }

    async fn create_checkout_session(
        &self,
        request: CreateCheckoutRequest,
    ) -> Result<CheckoutSession> {
        info!(
            "Creating checkout session for tier {:?}",
            request.tier
        );

        let _price_id = self.config.price_id_for_tier(request.tier)?;

        // In production, this would call:
        // POST https://api.stripe.com/v1/checkout/sessions
        // with mode=subscription, line_items, success_url, cancel_url

        // Mock implementation
        let session_id = format!(
            "cs_{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..24]
        );
        let url = format!(
            "https://checkout.stripe.com/pay/{}?success_url={}&cancel_url={}",
            session_id, request.success_url, request.cancel_url
        );

        debug!("Created checkout session: {}", session_id);

        Ok(CheckoutSession {
            id: session_id,
            url,
        })
    }

    async fn create_portal_session(&self, request: CreatePortalRequest) -> Result<PortalSession> {
        info!("Creating portal session for customer {:?}", request.customer_id);

        // In production, this would call:
        // POST https://api.stripe.com/v1/billing_portal/sessions
        // with customer and return_url

        // Mock implementation
        let session_id = format!(
            "bps_{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..24]
        );
        let url = format!(
            "https://billing.stripe.com/session/{}?return_url={}",
            session_id, request.return_url
        );

        debug!("Created portal session: {}", session_id);

        Ok(PortalSession {
            id: session_id,
            url,
        })
    }

    async fn verify_webhook(&self, payload: &[u8], _signature: &str) -> Result<WebhookEvent> {
        debug!("Verifying webhook signature");

        // In production, this would:
        // 1. Parse the Stripe-Signature header
        // 2. Compute HMAC-SHA256 of payload with webhook secret
        // 3. Compare signatures
        // 4. Check timestamp to prevent replay attacks

        // For now, we just parse the payload
        let payload_str =
            std::str::from_utf8(payload).map_err(|e| {
                SubscriptionError::WebhookSignature(format!("Invalid UTF-8: {e}"))
            })?;

        let event: WebhookPayload = serde_json::from_str(payload_str).map_err(|e| {
            SubscriptionError::WebhookSignature(format!("Invalid JSON: {e}"))
        })?;

        Ok(WebhookEvent {
            id: event.id,
            event_type: event.event_type,
            payload: event.data,
        })
    }

    async fn cancel_subscription(&self, stripe_subscription_id: &str) -> Result<()> {
        info!("Canceling subscription: {}", stripe_subscription_id);

        // In production, this would call:
        // DELETE https://api.stripe.com/v1/subscriptions/{id}
        // or POST with cancel_at_period_end=true

        debug!("Subscription canceled: {}", stripe_subscription_id);
        Ok(())
    }

    async fn get_subscription(&self, stripe_subscription_id: &str) -> Result<StripeSubscription> {
        info!("Getting subscription: {}", stripe_subscription_id);

        // In production, this would call:
        // GET https://api.stripe.com/v1/subscriptions/{id}

        // Mock implementation
        let now = chrono::Utc::now().timestamp();
        Ok(StripeSubscription {
            id: stripe_subscription_id.to_string(),
            status: "active".to_string(),
            current_period_start: now,
            current_period_end: now + 30 * 24 * 60 * 60, // 30 days
            cancel_at_period_end: false,
        })
    }
}

/// Internal webhook payload structure.
#[derive(Debug, Deserialize)]
struct WebhookPayload {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    data: serde_json::Value,
}

/// Stripe product configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeProduct {
    /// Product ID.
    pub id: String,
    /// Product name.
    pub name: String,
    /// Product description.
    pub description: Option<String>,
}

/// Stripe price configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePrice {
    /// Price ID.
    pub id: String,
    /// Product ID this price belongs to.
    pub product_id: String,
    /// Price in cents.
    pub unit_amount: u32,
    /// Currency (e.g., "usd").
    pub currency: String,
    /// Billing interval (e.g., "month").
    pub interval: String,
}

/// Creates the default product and price configuration for SynMem.
///
/// This would be used during initial Stripe account setup.
#[must_use]
pub fn default_products() -> Vec<(StripeProduct, StripePrice)> {
    vec![
        (
            StripeProduct {
                id: "prod_synmem_pro".to_string(),
                name: "SynMem Pro".to_string(),
                description: Some(
                    "Unlimited scrapes, cloud sessions, and full API access".to_string(),
                ),
            },
            StripePrice {
                id: "price_synmem_pro_monthly".to_string(),
                product_id: "prod_synmem_pro".to_string(),
                unit_amount: 1900, // $19.00
                currency: "usd".to_string(),
                interval: "month".to_string(),
            },
        ),
        (
            StripeProduct {
                id: "prod_synmem_enterprise".to_string(),
                name: "SynMem Enterprise".to_string(),
                description: Some("Custom pricing with dedicated support and SLA".to_string()),
            },
            StripePrice {
                id: "price_synmem_enterprise_monthly".to_string(),
                product_id: "prod_synmem_enterprise".to_string(),
                unit_amount: 0, // Custom pricing
                currency: "usd".to_string(),
                interval: "month".to_string(),
            },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> StripeConfig {
        StripeConfig::new(
            "sk_test_123".to_string(),
            "whsec_test_456".to_string(),
            "price_pro_123".to_string(),
        )
        .with_enterprise_price("price_ent_456".to_string())
    }

    #[tokio::test]
    async fn test_create_customer() {
        let adapter = StripeAdapter::new(test_config());
        let customer_id = adapter.create_customer("test@example.com").await.unwrap();

        assert!(customer_id.starts_with("cus_"));
    }

    #[tokio::test]
    async fn test_create_checkout_session() {
        let adapter = StripeAdapter::new(test_config());
        let request = CreateCheckoutRequest {
            customer_id: uuid::Uuid::new_v4(),
            tier: Tier::Pro,
            success_url: "https://example.com/success".to_string(),
            cancel_url: "https://example.com/cancel".to_string(),
        };

        let session = adapter.create_checkout_session(request).await.unwrap();
        assert!(session.id.starts_with("cs_"));
        assert!(session.url.contains("checkout.stripe.com"));
    }

    #[tokio::test]
    async fn test_create_portal_session() {
        let adapter = StripeAdapter::new(test_config());
        let request = CreatePortalRequest {
            customer_id: uuid::Uuid::new_v4(),
            return_url: "https://example.com/account".to_string(),
        };

        let session = adapter.create_portal_session(request).await.unwrap();
        assert!(session.id.starts_with("bps_"));
        assert!(session.url.contains("billing.stripe.com"));
    }

    #[tokio::test]
    async fn test_verify_webhook() {
        let adapter = StripeAdapter::new(test_config());
        let payload = r#"{"id":"evt_123","type":"checkout.session.completed","data":{"object":{}}}"#;

        let event = adapter
            .verify_webhook(payload.as_bytes(), "signature")
            .await
            .unwrap();

        assert_eq!(event.id, "evt_123");
        assert_eq!(event.event_type, "checkout.session.completed");
    }

    #[tokio::test]
    async fn test_get_subscription() {
        let adapter = StripeAdapter::new(test_config());
        let subscription = adapter.get_subscription("sub_123").await.unwrap();

        assert_eq!(subscription.id, "sub_123");
        assert_eq!(subscription.status, "active");
    }

    #[test]
    fn test_price_id_for_tier() {
        let config = test_config();

        assert!(config.price_id_for_tier(Tier::Free).is_err());
        assert_eq!(config.price_id_for_tier(Tier::Pro).unwrap(), "price_pro_123");
        assert_eq!(
            config.price_id_for_tier(Tier::Enterprise).unwrap(),
            "price_ent_456"
        );
    }

    #[test]
    fn test_default_products() {
        let products = default_products();
        assert_eq!(products.len(), 2);

        let (pro_product, pro_price) = &products[0];
        assert_eq!(pro_product.name, "SynMem Pro");
        assert_eq!(pro_price.unit_amount, 1900);

        let (ent_product, _) = &products[1];
        assert_eq!(ent_product.name, "SynMem Enterprise");
    }
}
