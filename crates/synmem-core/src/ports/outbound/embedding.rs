//! Embedding outbound port

use async_trait::async_trait;
use std::error::Error;

/// Port for embedding generation
#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    /// Error type for this port
    type Error: Error + Send + Sync + 'static;

    /// Generate embeddings for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>, Self::Error>;

    /// Generate embeddings for multiple texts in batch
    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, Self::Error>;
}
