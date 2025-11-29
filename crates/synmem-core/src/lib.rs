//! SynMem Core - Domain and Application Layer
//!
//! This crate contains the core domain logic and port definitions for SynMem.

pub mod domain;
pub mod ports;

pub use domain::entities::*;
pub use ports::outbound::embedding::*;
