//! Port definitions for hexagonal architecture
//!
//! Inbound ports are interfaces that the application provides to external actors.
//! Outbound ports are interfaces that the application requires from external actors.

pub mod inbound;
pub mod outbound;

pub use inbound::*;
pub use outbound::*;
