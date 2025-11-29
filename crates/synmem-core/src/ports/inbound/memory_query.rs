//! Memory query port for search operations

use async_trait::async_trait;

use crate::domain::entities::SearchResult;

/// Inbound port for memory search operations
#[async_trait]
pub trait MemoryQuery: Send + Sync {
    /// Search memories using hybrid search (FTS + vector)
    async fn search(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>>;

    /// Search memories using only full-text search
    async fn search_fts(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>>;

    /// Search memories using only vector similarity
    async fn search_vector(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>>;

    /// Get recent memories
    async fn get_recent(&self, limit: usize) -> anyhow::Result<Vec<SearchResult>>;
}
