//! Embedding outbound port
//!
//! This port defines the interface for generating embeddings.

use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during embedding operations
#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("Embedding failed: {0}")]
    EmbeddingFailed(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for embedding operations
pub type EmbeddingResult<T> = Result<T, EmbeddingError>;

/// Embedding outbound port trait
#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    /// Generates an embedding for the given text
    async fn embed(&self, text: &str) -> EmbeddingResult<Vec<f32>>;

    /// Generates embeddings for multiple texts
    async fn embed_batch(&self, texts: &[String]) -> EmbeddingResult<Vec<Vec<f32>>>;

    /// Returns the embedding dimension
    fn dimension(&self) -> usize;
}
