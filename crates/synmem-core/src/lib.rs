//! # SynMem Core
//!
//! Core domain logic and port definitions for SynMem browser automation.
//! This crate follows hexagonal architecture with ports and adapters pattern.

pub mod domain;
pub mod ports;

pub use domain::entities;
pub use ports::inbound;
pub use ports::outbound;
