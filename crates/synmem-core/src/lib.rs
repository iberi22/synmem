//! SynMem Core - Domain and Application Layer
//!
//! This crate contains the core business logic, domain entities, and port definitions
//! for the SynMem browser automation system.

pub mod domain;
pub mod ports;

pub use domain::*;
pub use ports::*;
