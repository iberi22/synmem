//! SynMem Core - Domain and application layer
//!
//! This crate contains the core business logic for SynMem, including:
//! - Domain entities (BrowserAction, Macro)
//! - Domain services (MacroService)
//! - Port definitions for adapters

pub mod domain;
pub mod error;
pub mod ports;

pub use domain::entities::{BrowserAction, Macro, MacroInfo, ScrollDirection};
pub use domain::services::MacroService;
pub use error::CoreError;
pub use ports::inbound::MacroController;
pub use ports::outbound::MacroStorage;
