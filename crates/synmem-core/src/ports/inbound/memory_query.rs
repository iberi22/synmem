//! # Memory Query Port
//!
//! Inbound port for memory and context operations.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

/// Errors that can occur during memory operations.
#[derive(Debug, Error)]
pub enum MemoryQueryError {
    #[error("search failed: {0}")]
    SearchFailed(String),

    #[error("save failed: {0}")]
    SaveFailed(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("storage unavailable")]
    StorageUnavailable,
}

/// Result type for memory query operations.
pub type MemoryQueryResult<T> = Result<T, MemoryQueryError>;

/// A memory entry representing stored context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier.
    pub id: Uuid,
    /// The URL associated with this memory.
    pub url: Option<Url>,
    /// Title or summary.
    pub title: String,
    /// The content stored.
    pub content: String,
    /// When this memory was created.
    pub created_at: DateTime<Utc>,
    /// Optional tags for categorization.
    pub tags: Vec<String>,
}

/// Options for searching memory.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum number of results to return.
    pub limit: Option<usize>,
    /// Filter by tags.
    pub tags: Vec<String>,
    /// Filter by date range (start).
    pub from_date: Option<DateTime<Utc>>,
    /// Filter by date range (end).
    pub to_date: Option<DateTime<Utc>>,
}

/// A search result with relevance score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The matching memory entry.
    pub entry: MemoryEntry,
    /// Relevance score (0.0 to 1.0).
    pub score: f32,
}

/// Context to be saved to memory.
#[derive(Debug, Clone)]
pub struct ContextToSave {
    /// Optional URL associated with the context.
    pub url: Option<Url>,
    /// Title or summary.
    pub title: String,
    /// The content to save.
    pub content: String,
    /// Optional tags for categorization.
    pub tags: Vec<String>,
}

/// Inbound port for memory and context operations.
///
/// This port defines the interface for searching memory, getting recent
/// entries, and saving context.
#[async_trait]
pub trait MemoryQueryPort: Send + Sync {
    /// Search memory using semantic or keyword search.
    async fn search(
        &self,
        query: &str,
        options: SearchOptions,
    ) -> MemoryQueryResult<Vec<SearchResult>>;

    /// Get the most recent memory entries.
    async fn get_recent(&self, limit: usize) -> MemoryQueryResult<Vec<MemoryEntry>>;

    /// Save context to memory.
    async fn save_context(&self, context: ContextToSave) -> MemoryQueryResult<Uuid>;
}
