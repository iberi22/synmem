//! SynMem Core - Domain and Application Layer
//!
//! This crate contains the core domain entities, services, and ports
//! for the SynMem synthetic memory browser agent.

pub mod domain;
pub mod ports;

pub use domain::*;
pub use ports::*;
