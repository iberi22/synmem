//! Embedding port for text embeddings

use async_trait::async_trait;

/// Outbound port for embedding generation
#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    /// Generate embedding for text
    async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>>;

    /// Generate embeddings for multiple texts (batch)
    async fn embed_batch(&self, texts: &[&str]) -> anyhow::Result<Vec<Vec<f32>>>;

    /// Get the embedding dimension
    fn dimension(&self) -> usize;
}
