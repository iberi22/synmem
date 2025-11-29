//! Subscription service implementation.
//!
//! Orchestrates the subscription workflow using ports.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::domain::{
    Customer, License, LicenseKeyPair, Subscription, SubscriptionEvent, SubscriptionEventType,
    Tier,
};
use crate::error::{Result, SubscriptionError};
use crate::ports::{
    CheckoutRequest, CheckoutSession, CreateCheckoutRequest, CreatePortalRequest,
    GenerateLicenseRequest, PaymentPort, PortalSession, SubscriptionServicePort,
    SubscriptionStoragePort,
};

/// Subscription service configuration.
#[derive(Debug, Clone)]
pub struct SubscriptionServiceConfig {
    /// Base URL for success redirects.
    pub base_url: String,
}

impl Default for SubscriptionServiceConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
        }
    }
}

/// Subscription service implementation.
pub struct SubscriptionService<P, S>
where
    P: PaymentPort,
    S: SubscriptionStoragePort,
{
    payment: Arc<P>,
    storage: Arc<S>,
    license_keys: LicenseKeyPair,
    #[allow(dead_code)]
    config: SubscriptionServiceConfig,
}

impl<P, S> SubscriptionService<P, S>
where
    P: PaymentPort,
    S: SubscriptionStoragePort,
{
    /// Creates a new subscription service.
    pub fn new(payment: Arc<P>, storage: Arc<S>, config: SubscriptionServiceConfig) -> Self {
        Self {
            payment,
            storage,
            license_keys: LicenseKeyPair::generate(),
            config,
        }
    }

    /// Creates the service with an existing license key pair.
    pub fn with_license_keys(
        payment: Arc<P>,
        storage: Arc<S>,
        config: SubscriptionServiceConfig,
        license_keys: LicenseKeyPair,
    ) -> Self {
        Self {
            payment,
            storage,
            license_keys,
            config,
        }
    }

    /// Returns the public key for license verification.
    #[must_use]
    pub fn license_public_key(&self) -> String {
        self.license_keys.export_public()
    }
}

#[async_trait]
impl<P, S> SubscriptionServicePort for SubscriptionService<P, S>
where
    P: PaymentPort + 'static,
    S: SubscriptionStoragePort + 'static,
{
    async fn get_or_create_customer(&self, email: &str) -> Result<Customer> {
        // Check if customer exists
        if let Some(customer) = self.storage.get_customer_by_email(email).await? {
            debug!("Found existing customer: {}", customer.id);
            return Ok(customer);
        }

        // Create customer in Stripe
        let stripe_customer_id = self.payment.create_customer(email).await?;

        // Create customer in storage
        let customer = Customer::with_stripe_id(email.to_string(), stripe_customer_id);
        self.storage.create_customer(&customer).await?;

        // Create free subscription
        let subscription = Subscription::free(customer.id);
        self.storage.create_subscription(&subscription).await?;

        // Record event
        let event = SubscriptionEvent::new(
            customer.id,
            SubscriptionEventType::Created,
            None,
            Some(Tier::Free),
        );
        self.storage.record_event(&event).await?;

        info!("Created new customer: {} ({})", customer.id, email);
        Ok(customer)
    }

    async fn get_customer(&self, id: Uuid) -> Result<Option<Customer>> {
        self.storage.get_customer(id).await
    }

    async fn create_checkout(&self, request: CheckoutRequest) -> Result<CheckoutSession> {
        // Get or create customer
        let customer = self.get_or_create_customer(&request.email).await?;

        // Create checkout session
        let checkout_request = CreateCheckoutRequest {
            customer_id: customer.id,
            tier: request.tier,
            success_url: request.success_url,
            cancel_url: request.cancel_url,
        };

        let session = self
            .payment
            .create_checkout_session(checkout_request)
            .await?;

        info!(
            "Created checkout session for customer {} tier {:?}",
            customer.id, request.tier
        );
        Ok(session)
    }

    async fn create_portal(&self, customer_id: Uuid, return_url: &str) -> Result<PortalSession> {
        let customer = self
            .storage
            .get_customer(customer_id)
            .await?
            .ok_or_else(|| SubscriptionError::CustomerNotFound(customer_id.to_string()))?;

        let _stripe_customer_id = customer.stripe_customer_id.ok_or_else(|| {
            SubscriptionError::InvalidOperation(
                "Customer has no Stripe account".to_string(),
            )
        })?;

        let request = CreatePortalRequest {
            customer_id,
            return_url: return_url.to_string(),
        };

        self.payment.create_portal_session(request).await
    }

    async fn get_subscription(&self, customer_id: Uuid) -> Result<Option<Subscription>> {
        self.storage.get_active_subscription(customer_id).await
    }

    async fn cancel_subscription(&self, customer_id: Uuid) -> Result<()> {
        let subscription = self
            .storage
            .get_active_subscription(customer_id)
            .await?
            .ok_or_else(|| {
                SubscriptionError::SubscriptionNotFound(format!(
                    "No active subscription for customer {}",
                    customer_id
                ))
            })?;

        // Cancel in Stripe if applicable
        if let Some(stripe_sub_id) = &subscription.stripe_subscription_id {
            self.payment.cancel_subscription(stripe_sub_id).await?;
        }

        // Update subscription status
        let mut updated = subscription.clone();
        updated.cancel_at_period_end = true;
        updated.updated_at = Utc::now();
        self.storage.update_subscription(&updated).await?;

        // Record event
        let event = SubscriptionEvent::new(
            customer_id,
            SubscriptionEventType::Canceled,
            Some(subscription.tier),
            None,
        );
        self.storage.record_event(&event).await?;

        info!("Canceled subscription for customer {}", customer_id);
        Ok(())
    }

    async fn handle_webhook(&self, payload: &[u8], signature: &str) -> Result<()> {
        let event = self.payment.verify_webhook(payload, signature).await?;

        info!("Processing webhook event: {} ({})", event.event_type, event.id);

        match event.event_type.as_str() {
            "checkout.session.completed" => {
                self.handle_checkout_completed(&event.payload).await?;
            }
            "customer.subscription.updated" => {
                self.handle_subscription_updated(&event.payload).await?;
            }
            "customer.subscription.deleted" => {
                self.handle_subscription_deleted(&event.payload).await?;
            }
            "invoice.payment_failed" => {
                self.handle_payment_failed(&event.payload).await?;
            }
            "invoice.payment_succeeded" => {
                self.handle_payment_succeeded(&event.payload).await?;
            }
            _ => {
                debug!("Ignoring webhook event type: {}", event.event_type);
            }
        }

        Ok(())
    }

    async fn generate_license(&self, request: GenerateLicenseRequest) -> Result<License> {
        // Verify customer exists
        let _customer = self
            .storage
            .get_customer(request.customer_id)
            .await?
            .ok_or_else(|| {
                SubscriptionError::CustomerNotFound(request.customer_id.to_string())
            })?;

        // Generate license key
        let key = format!(
            "SYNMEM-{}-{}",
            request.tier.to_string().to_uppercase(),
            uuid::Uuid::new_v4().to_string().replace('-', "")[..12].to_uppercase()
        );

        let expires_at = Utc::now() + Duration::days(i64::from(request.duration_days));

        let license = License::new(
            key,
            request.tier,
            expires_at,
            self.license_keys.signing_key(),
        );

        // Store license
        self.storage.store_license(&license).await?;

        info!(
            "Generated license {} for customer {}",
            license.key, request.customer_id
        );
        Ok(license)
    }

    async fn validate_license(&self, license_key: &str) -> Result<License> {
        let license = self
            .storage
            .get_license(license_key)
            .await?
            .ok_or_else(|| {
                SubscriptionError::LicenseValidation(format!(
                    "License not found: {}",
                    license_key
                ))
            })?;

        // Validate signature and expiration
        license.validate(self.license_keys.verifying_key())?;

        Ok(license)
    }

    async fn has_feature_access(&self, customer_id: Uuid, feature: &str) -> Result<bool> {
        let subscription = self.storage.get_active_subscription(customer_id).await?;

        let tier = subscription.map(|s| s.tier).unwrap_or(Tier::Free);

        let has_access = match feature {
            "cloud_sessions" => tier.cloud_sessions(),
            "api_access" => tier.api_access(),
            "dedicated_support" => tier.dedicated_support(),
            "sla" => tier.sla(),
            "unlimited_scrapes" => tier.scrapes_per_month().is_none(),
            _ => false,
        };

        Ok(has_access)
    }

    async fn get_remaining_scrapes(&self, customer_id: Uuid) -> Result<Option<u32>> {
        let subscription = self.storage.get_active_subscription(customer_id).await?;
        let tier = subscription.map(|s| s.tier).unwrap_or(Tier::Free);

        // For now, just return the limit (in production, track usage)
        Ok(tier.scrapes_per_month())
    }

    async fn record_event(&self, event: SubscriptionEvent) -> Result<()> {
        self.storage.record_event(&event).await
    }

    async fn get_events(
        &self,
        customer_id: Uuid,
        limit: Option<u32>,
    ) -> Result<Vec<SubscriptionEvent>> {
        self.storage.get_customer_events(customer_id, limit).await
    }
}

// Private helper methods for webhook handling
impl<P, S> SubscriptionService<P, S>
where
    P: PaymentPort,
    S: SubscriptionStoragePort,
{
    async fn handle_checkout_completed(&self, data: &serde_json::Value) -> Result<()> {
        // Extract customer info from webhook data
        // In production, parse Stripe webhook data structure
        debug!("Checkout completed: {:?}", data);
        Ok(())
    }

    async fn handle_subscription_updated(&self, data: &serde_json::Value) -> Result<()> {
        debug!("Subscription updated: {:?}", data);
        Ok(())
    }

    async fn handle_subscription_deleted(&self, data: &serde_json::Value) -> Result<()> {
        debug!("Subscription deleted: {:?}", data);
        Ok(())
    }

    async fn handle_payment_failed(&self, data: &serde_json::Value) -> Result<()> {
        warn!("Payment failed: {:?}", data);
        Ok(())
    }

    async fn handle_payment_succeeded(&self, data: &serde_json::Value) -> Result<()> {
        debug!("Payment succeeded: {:?}", data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{InMemoryStorage, StripeAdapter, StripeConfig};

    fn test_service() -> SubscriptionService<StripeAdapter, InMemoryStorage> {
        let config = StripeConfig::new(
            "sk_test_123".to_string(),
            "whsec_test".to_string(),
            "price_pro".to_string(),
        );
        let payment = Arc::new(StripeAdapter::new(config));
        let storage = Arc::new(InMemoryStorage::new());

        SubscriptionService::new(payment, storage, SubscriptionServiceConfig::default())
    }

    #[tokio::test]
    async fn test_get_or_create_customer() {
        let service = test_service();

        // First call creates customer
        let customer1 = service
            .get_or_create_customer("test@example.com")
            .await
            .unwrap();
        assert_eq!(customer1.email, "test@example.com");
        assert!(customer1.stripe_customer_id.is_some());

        // Second call returns same customer
        let customer2 = service
            .get_or_create_customer("test@example.com")
            .await
            .unwrap();
        assert_eq!(customer1.id, customer2.id);
    }

    #[tokio::test]
    async fn test_create_checkout() {
        let service = test_service();

        let request = CheckoutRequest {
            email: "test@example.com".to_string(),
            tier: Tier::Pro,
            success_url: "https://example.com/success".to_string(),
            cancel_url: "https://example.com/cancel".to_string(),
        };

        let session = service.create_checkout(request).await.unwrap();
        assert!(!session.id.is_empty());
        assert!(!session.url.is_empty());
    }

    #[tokio::test]
    async fn test_generate_and_validate_license() {
        let service = test_service();

        // Create customer first
        let customer = service
            .get_or_create_customer("test@example.com")
            .await
            .unwrap();

        // Generate license
        let request = GenerateLicenseRequest {
            customer_id: customer.id,
            tier: Tier::Pro,
            duration_days: 30,
        };

        let license = service.generate_license(request).await.unwrap();
        assert!(license.key.starts_with("SYNMEM-PRO-"));
        assert_eq!(license.tier, Tier::Pro);

        // Validate license
        let validated = service.validate_license(&license.key).await.unwrap();
        assert_eq!(validated.key, license.key);
    }

    #[tokio::test]
    async fn test_feature_access() {
        let service = test_service();

        // Create customer with free tier
        let customer = service
            .get_or_create_customer("test@example.com")
            .await
            .unwrap();

        // Free tier doesn't have cloud sessions
        let has_cloud = service
            .has_feature_access(customer.id, "cloud_sessions")
            .await
            .unwrap();
        assert!(!has_cloud);

        // Free tier has limited scrapes
        let scrapes = service.get_remaining_scrapes(customer.id).await.unwrap();
        assert_eq!(scrapes, Some(100));
    }

    #[tokio::test]
    async fn test_public_key_export() {
        let service = test_service();
        let public_key = service.license_public_key();

        // Should be base64 encoded
        assert!(!public_key.is_empty());
        assert!(public_key.len() > 20);
    }
}
