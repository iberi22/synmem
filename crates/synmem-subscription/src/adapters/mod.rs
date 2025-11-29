//! Adapters for external systems.
//!
//! Implements the outbound ports defined in the ports module.

pub mod memory;
pub mod stripe;

pub use memory::InMemoryStorage;
pub use stripe::{StripeAdapter, StripeConfig, StripePrice, StripeProduct};
