//! Embedding port trait definition.
//!
//! This module defines the `EmbeddingPort` trait that abstracts over
//! different embedding implementations.

use crate::domain::entities::{Embedding, SearchResult};
use thiserror::Error;

/// Errors that can occur during embedding operations.
#[derive(Debug, Error)]
pub enum EmbeddingError {
    /// Failed to initialize the embedding model.
    #[error("Failed to initialize embedding model: {0}")]
    InitializationError(String),

    /// Failed to generate embeddings.
    #[error("Failed to generate embeddings: {0}")]
    GenerationError(String),

    /// The input text was invalid.
    #[error("Invalid input text: {0}")]
    InvalidInput(String),

    /// The model is not available.
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
}

/// Port for embedding generation and semantic search.
///
/// This trait defines the interface for generating embeddings from text
/// and performing semantic similarity search.
pub trait EmbeddingPort: Send + Sync {
    /// Generates an embedding for a single text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to embed.
    ///
    /// # Returns
    ///
    /// The embedding vector for the text.
    fn embed(&self, text: &str) -> Result<Embedding, EmbeddingError>;

    /// Generates embeddings for multiple texts in batch.
    ///
    /// This method is optimized for processing multiple texts efficiently,
    /// potentially using parallelization.
    ///
    /// # Arguments
    ///
    /// * `texts` - The texts to embed.
    ///
    /// # Returns
    ///
    /// A vector of embeddings, one for each input text.
    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>, EmbeddingError>;

    /// Searches for the most similar embeddings to a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query embedding to search for.
    /// * `candidates` - The candidate embeddings to search through.
    /// * `top_k` - The maximum number of results to return.
    ///
    /// # Returns
    ///
    /// A vector of search results sorted by similarity (highest first).
    fn search(
        &self,
        query: &Embedding,
        candidates: &[(usize, Embedding)],
        top_k: usize,
    ) -> Vec<SearchResult<usize>>;

    /// Returns the dimensionality of embeddings produced by this model.
    fn embedding_dimension(&self) -> usize;

    /// Returns the name/identifier of the embedding model.
    fn model_name(&self) -> &str;
}
