//! Memory query inbound port
//!
//! This port defines the interface for querying stored memories.

use crate::domain::entities::{Memory, MemorySearchResult};
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during memory operations
#[derive(Debug, Error)]
pub enum MemoryQueryError {
    #[error("Memory not found: {0}")]
    NotFound(String),
    #[error("Search failed: {0}")]
    SearchFailed(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for memory operations
pub type MemoryQueryResult<T> = Result<T, MemoryQueryError>;

/// Search options
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Minimum relevance score (0.0 - 1.0)
    pub min_score: Option<f32>,
    /// Filter by tags
    pub tags: Option<Vec<String>>,
    /// Filter by URL pattern
    pub url_pattern: Option<String>,
}

/// Memory query inbound port trait
#[async_trait]
pub trait MemoryQueryPort: Send + Sync {
    /// Performs semantic search on stored memories
    async fn search(
        &self,
        query: &str,
        options: SearchOptions,
    ) -> MemoryQueryResult<Vec<MemorySearchResult>>;

    /// Gets the most recent memories
    async fn get_recent(&self, limit: usize) -> MemoryQueryResult<Vec<Memory>>;

    /// Saves the current context as a memory
    async fn save_context(
        &self,
        url: &str,
        title: &str,
        content: &str,
        tags: Vec<String>,
    ) -> MemoryQueryResult<Memory>;

    /// Lists all stored sessions
    async fn list_sessions(&self) -> MemoryQueryResult<Vec<String>>;

    /// Gets a specific memory by ID
    async fn get_memory(&self, id: &str) -> MemoryQueryResult<Memory>;

    /// Deletes a memory by ID
    async fn delete_memory(&self, id: &str) -> MemoryQueryResult<()>;
}
