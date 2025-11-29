//! Memory query port - inbound port for memory operations

use async_trait::async_trait;
use thiserror::Error;

use crate::domain::entities::{ChatContext, Memory, SavedContext, SearchResult, Session};

/// Errors that can occur during memory query operations
#[derive(Debug, Error)]
pub enum MemoryQueryError {
    #[error("Search failed: {0}")]
    SearchFailed(String),

    #[error("Failed to retrieve data: {0}")]
    RetrievalFailed(String),

    #[error("Failed to save data: {0}")]
    SaveFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Options for search queries
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Filter by content types
    pub content_types: Option<Vec<String>>,
    /// Filter by sources
    pub sources: Option<Vec<String>>,
}

/// Inbound port for memory query operations
#[async_trait]
pub trait MemoryQueryPort {
    /// Search memory using semantic/full-text search
    async fn search_memory(
        &self,
        query: &str,
        options: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>, MemoryQueryError>;

    /// Get the most recent N memory entries
    async fn get_recent(&self, limit: usize) -> Result<Vec<Memory>, MemoryQueryError>;

    /// Save the current context with tags for later retrieval
    async fn save_context(
        &self,
        name: &str,
        content: &str,
        tags: Vec<String>,
    ) -> Result<SavedContext, MemoryQueryError>;

    /// List all saved browser sessions
    async fn list_sessions(&self) -> Result<Vec<Session>, MemoryQueryError>;

    /// Get a specific AI chat conversation for context injection
    async fn get_chat_context(
        &self,
        chat_id: &str,
    ) -> Result<Option<ChatContext>, MemoryQueryError>;
}
