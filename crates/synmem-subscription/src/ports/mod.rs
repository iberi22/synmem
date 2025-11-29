//! Ports (interfaces) for the subscription system.
//!
//! Following hexagonal architecture:
//! - Inbound ports: interfaces that the application provides to the outside world
//! - Outbound ports: interfaces that the application requires from the outside world

pub mod payment;
pub mod service;
pub mod storage;

pub use payment::{
    CheckoutSession, CreateCheckoutRequest, CreatePortalRequest, PaymentPort, PortalSession,
    StripeSubscription, WebhookEvent,
};
pub use service::{CheckoutRequest, GenerateLicenseRequest, SubscriptionServicePort};
pub use storage::SubscriptionStoragePort;
