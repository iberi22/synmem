//! Memory service for managing memory operations

use std::sync::Arc;

use crate::domain::entities::{ChatContext, Memory, SavedContext, SearchResult, Session};
use crate::ports::inbound::memory_query::{MemoryQueryError, MemoryQueryPort, SearchOptions};
use crate::ports::outbound::storage::StoragePort;

/// Service for memory operations
pub struct MemoryService<S: StoragePort> {
    storage: Arc<S>,
}

impl<S: StoragePort> MemoryService<S> {
    /// Creates a new MemoryService
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }
}

#[async_trait::async_trait]
impl<S: StoragePort + Send + Sync> MemoryQueryPort for MemoryService<S> {
    async fn search_memory(
        &self,
        query: &str,
        options: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>, MemoryQueryError> {
        let opts = options.unwrap_or_default();

        self.storage
            .search(query, opts.limit, opts.content_types, opts.sources)
            .await
            .map_err(|e| MemoryQueryError::SearchFailed(e.to_string()))
    }

    async fn get_recent(&self, limit: usize) -> Result<Vec<Memory>, MemoryQueryError> {
        self.storage
            .get_recent_memories(limit)
            .await
            .map_err(|e| MemoryQueryError::RetrievalFailed(e.to_string()))
    }

    async fn save_context(
        &self,
        name: &str,
        content: &str,
        tags: Vec<String>,
    ) -> Result<SavedContext, MemoryQueryError> {
        let context = SavedContext::new(name.to_string(), content.to_string(), tags);

        self.storage
            .save_context(&context)
            .await
            .map_err(|e| MemoryQueryError::SaveFailed(e.to_string()))?;

        Ok(context)
    }

    async fn list_sessions(&self) -> Result<Vec<Session>, MemoryQueryError> {
        self.storage
            .list_sessions()
            .await
            .map_err(|e| MemoryQueryError::RetrievalFailed(e.to_string()))
    }

    async fn get_chat_context(
        &self,
        chat_id: &str,
    ) -> Result<Option<ChatContext>, MemoryQueryError> {
        let id = uuid::Uuid::parse_str(chat_id)
            .map_err(|_| MemoryQueryError::InvalidInput("Invalid chat ID format".to_string()))?;

        self.storage
            .get_chat_context(&id)
            .await
            .map_err(|e| MemoryQueryError::RetrievalFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    // Tests will be added with the storage implementation
}
