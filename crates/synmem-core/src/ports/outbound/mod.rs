//! # Outbound Ports (Driven)
//!
//! These ports define how the SynMem system interacts with external dependencies.

mod browser_driver;
mod embedding;
mod session_persistence;
mod storage;

pub use browser_driver::{
    BrowserDriverError, BrowserDriverPort, BrowserDriverResult, LaunchOptions, PageInfo, Viewport,
    WaitOptions,
};
pub use embedding::{Embedding, EmbeddingError, EmbeddingPort, EmbeddingResult, ModelInfo};
pub use session_persistence::{
    Cookie, LocalStorageEntry, SameSite, SessionPersistenceError, SessionPersistencePort,
    SessionPersistenceResult, SessionState,
};
pub use storage::{QueryFilter, StorageError, StoragePort, StorageResult, StoredRecord};
