//! Storage port for data persistence

use async_trait::async_trait;

use crate::domain::entities::{Memory, SearchResult};

/// Outbound port for storage operations
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Perform full-text search
    async fn full_text_search(
        &self,
        query: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<SearchResult>>;

    /// Perform vector similarity search
    async fn vector_search(
        &self,
        embedding: &[f32],
        limit: usize,
    ) -> anyhow::Result<Vec<SearchResult>>;

    /// Store a memory with its embedding
    async fn store_memory(&self, memory: &Memory, embedding: &[f32]) -> anyhow::Result<()>;

    /// Get a memory by ID
    async fn get_memory(&self, id: &str) -> anyhow::Result<Option<Memory>>;

    /// Get recent memories
    async fn get_recent_memories(&self, limit: usize) -> anyhow::Result<Vec<Memory>>;

    /// Delete a memory by ID
    async fn delete_memory(&self, id: &str) -> anyhow::Result<bool>;
}
