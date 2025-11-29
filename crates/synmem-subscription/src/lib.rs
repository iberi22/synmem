//! SynMem Subscription System
//!
//! A subscription and payment system for SynMem, implementing:
//! - Stripe integration for payment processing
//! - Subscription tiers (Free, Pro, Enterprise)
//! - License key system with Ed25519 signatures for offline validation
//! - Subscription metrics tracking (MRR, churn, conversion funnel)
//!
//! # Architecture
//!
//! This crate follows hexagonal architecture (ports and adapters):
//!
//! - **Domain**: Core business entities and logic
//! - **Ports**: Interfaces for external systems
//! - **Adapters**: Implementations of ports
//! - **Service**: Application layer orchestrating the workflow
//!
//! # Example
//!
//! ```rust,no_run
//! use synmem_subscription::{
//!     adapters::{InMemoryStorage, StripeAdapter, StripeConfig},
//!     domain::Tier,
//!     ports::{CheckoutRequest, SubscriptionServicePort},
//!     service::{SubscriptionService, SubscriptionServiceConfig},
//! };
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure Stripe
//! let stripe_config = StripeConfig::new(
//!     "sk_test_...".to_string(),
//!     "whsec_...".to_string(),
//!     "price_...".to_string(),
//! );
//!
//! // Create adapters
//! let payment = Arc::new(StripeAdapter::new(stripe_config));
//! let storage = Arc::new(InMemoryStorage::new());
//!
//! // Create service
//! let service = SubscriptionService::new(
//!     payment,
//!     storage,
//!     SubscriptionServiceConfig::default(),
//! );
//!
//! // Create a checkout session
//! let checkout = service.create_checkout(CheckoutRequest {
//!     email: "user@example.com".to_string(),
//!     tier: Tier::Pro,
//!     success_url: "https://example.com/success".to_string(),
//!     cancel_url: "https://example.com/cancel".to_string(),
//! }).await?;
//!
//! println!("Checkout URL: {}", checkout.url);
//! # Ok(())
//! # }
//! ```
//!
//! # Subscription Tiers
//!
//! | Tier | Price | Scrapes/Month | Cloud Sessions | API Access |
//! |------|-------|---------------|----------------|------------|
//! | Free | $0 | 100 | ❌ | ❌ |
//! | Pro | $19 | Unlimited | ✅ | ✅ |
//! | Enterprise | Custom | Unlimited | ✅ | ✅ |
//!
//! # License Keys
//!
//! License keys are Ed25519-signed tokens that can be validated offline:
//!
//! ```rust
//! use synmem_subscription::domain::{License, LicenseKeyPair, Tier};
//! use chrono::{Duration, Utc};
//!
//! // Generate a key pair (do this once, store securely)
//! let key_pair = LicenseKeyPair::generate();
//!
//! // Create a license
//! let license = License::new(
//!     "LICENSE-KEY-123".to_string(),
//!     Tier::Pro,
//!     Utc::now() + Duration::days(365),
//!     key_pair.signing_key(),
//! );
//!
//! // Validate offline using only the public key
//! let public_key = key_pair.export_public();
//! let verifying_key = LicenseKeyPair::verifying_key_from_base64(&public_key).unwrap();
//! license.validate(&verifying_key).unwrap();
//! ```

pub mod adapters;
pub mod domain;
pub mod error;
pub mod ports;
pub mod service;

pub use error::{Result, SubscriptionError};

/// Re-export commonly used types.
pub mod prelude {
    pub use crate::adapters::{InMemoryStorage, StripeAdapter, StripeConfig};
    pub use crate::domain::{
        ChurnMetrics, ConversionFunnel, Customer, License, LicenseKeyPair, MrrSnapshot,
        Subscription, SubscriptionEvent, SubscriptionStatus, Tier,
    };
    pub use crate::error::{Result, SubscriptionError};
    pub use crate::ports::{
        CheckoutRequest, CheckoutSession, GenerateLicenseRequest, PaymentPort, PortalSession,
        SubscriptionServicePort, SubscriptionStoragePort,
    };
    pub use crate::service::{SubscriptionService, SubscriptionServiceConfig};
}
