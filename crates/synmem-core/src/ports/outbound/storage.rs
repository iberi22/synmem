//! Storage port - outbound port for persistence operations

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::{ChatContext, Memory, SavedContext, SearchResult, Session};

/// Errors that can occur during storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Connection error: {0}")]
    Connection(String),
}

/// Outbound port for storage operations
#[async_trait]
pub trait StoragePort {
    /// Search memories using FTS
    async fn search(
        &self,
        query: &str,
        limit: Option<usize>,
        content_types: Option<Vec<String>>,
        sources: Option<Vec<String>>,
    ) -> Result<Vec<SearchResult>, StorageError>;

    /// Get recent memories
    async fn get_recent_memories(&self, limit: usize) -> Result<Vec<Memory>, StorageError>;

    /// Save a memory entry
    async fn save_memory(&self, memory: &Memory) -> Result<(), StorageError>;

    /// Save a context
    async fn save_context(&self, context: &SavedContext) -> Result<(), StorageError>;

    /// List all sessions
    async fn list_sessions(&self) -> Result<Vec<Session>, StorageError>;

    /// Get a session by ID
    async fn get_session(&self, id: &Uuid) -> Result<Option<Session>, StorageError>;

    /// Save a session
    async fn save_session(&self, session: &Session) -> Result<(), StorageError>;

    /// Get a chat context by ID
    async fn get_chat_context(&self, id: &Uuid) -> Result<Option<ChatContext>, StorageError>;

    /// Save a chat context
    async fn save_chat_context(&self, chat: &ChatContext) -> Result<(), StorageError>;
}
