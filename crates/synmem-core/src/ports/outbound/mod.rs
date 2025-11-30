//! Outbound ports - interfaces required by the application

pub mod session_persistence;

mod browser_driver;
mod storage;
mod embedding;

pub use session_persistence::SessionPersistencePort;
pub use browser_driver::*;
pub use storage::*;
pub use embedding::*;
