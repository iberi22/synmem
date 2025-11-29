//! # Outbound Ports (Driven)
//!
//! These ports define how the SynMem system interacts with external dependencies.

mod browser_driver;
mod embedding;
mod session_persistence;
mod storage;

pub use browser_driver::BrowserDriverPort;
pub use embedding::EmbeddingPort;
pub use session_persistence::SessionPersistencePort;
pub use storage::StoragePort;
