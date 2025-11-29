//! Storage outbound port
//!
//! This port defines the interface for persistent storage.

use crate::domain::entities::Memory;
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Write failed: {0}")]
    WriteFailed(String),
    #[error("Read failed: {0}")]
    ReadFailed(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

/// Storage outbound port trait
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Stores a memory entry
    async fn store(&self, memory: &Memory) -> StorageResult<()>;

    /// Retrieves a memory by ID
    async fn get(&self, id: &str) -> StorageResult<Memory>;

    /// Lists all memories
    async fn list(&self, limit: usize, offset: usize) -> StorageResult<Vec<Memory>>;

    /// Deletes a memory by ID
    async fn delete(&self, id: &str) -> StorageResult<()>;

    /// Searches memories by text
    async fn search(&self, query: &str, limit: usize) -> StorageResult<Vec<Memory>>;
}
