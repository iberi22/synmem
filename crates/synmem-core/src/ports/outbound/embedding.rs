//! # Embedding Port
//!
//! Outbound port for vector embedding generation.

use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during embedding operations.
#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("generation failed: {0}")]
    GenerationFailed(String),

    #[error("model not loaded: {0}")]
    ModelNotLoaded(String),

    #[error("input too long: max {max} characters, got {actual}")]
    InputTooLong { max: usize, actual: usize },

    #[error("batch too large: max {max} items, got {actual}")]
    BatchTooLarge { max: usize, actual: usize },
}

/// Result type for embedding operations.
pub type EmbeddingResult<T> = Result<T, EmbeddingError>;

/// A vector embedding.
#[derive(Debug, Clone)]
pub struct Embedding {
    /// The embedding vector.
    pub vector: Vec<f32>,
    /// Dimensionality of the embedding.
    pub dimensions: usize,
}

/// Information about the embedding model.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Model name or identifier.
    pub name: String,
    /// Embedding dimensions.
    pub dimensions: usize,
    /// Maximum input length in characters.
    pub max_input_length: usize,
}

/// Outbound port for vector embedding generation.
///
/// This port defines the interface for generating vector embeddings
/// from text. Implementations might use local models (fastembed) or
/// remote APIs (OpenAI, Cohere, etc.).
#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    /// Get information about the embedding model.
    async fn model_info(&self) -> EmbeddingResult<ModelInfo>;

    /// Generate an embedding for a single text.
    async fn embed(&self, text: &str) -> EmbeddingResult<Embedding>;

    /// Generate embeddings for multiple texts (batch).
    async fn embed_batch(&self, texts: &[&str]) -> EmbeddingResult<Vec<Embedding>>;
}
