//! # Ports Module
//!
//! Defines the hexagonal architecture boundaries (ports) for SynMem.
//!
//! ## Inbound Ports (Driving)
//! These ports define how external actors can interact with the system.
//!
//! ## Outbound Ports (Driven)
//! These ports define how the system interacts with external dependencies.

pub mod inbound;
pub mod outbound;
