//! Mock embedding adapter for testing purposes.
//!
//! This adapter provides deterministic embeddings for testing without
//! requiring any external ML models or runtime dependencies.

use rayon::prelude::*;
use synmem_core::{Embedding, EmbeddingError, EmbeddingPort, SearchResult};

/// A mock embedding adapter that generates deterministic embeddings.
///
/// This adapter is useful for testing the embedding port interface
/// without requiring actual ML models.
pub struct MockEmbedAdapter {
    /// The dimensionality of generated embeddings.
    dimension: usize,
}

impl MockEmbedAdapter {
    /// Creates a new mock adapter with the specified embedding dimension.
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// Generates a deterministic embedding based on the text content.
    ///
    /// The embedding is derived from a simple hash of the text characters,
    /// which ensures the same text always produces the same embedding.
    fn generate_embedding(&self, text: &str) -> Vec<f32> {
        let mut values = vec![0.0f32; self.dimension];

        if text.is_empty() {
            return values;
        }

        // Generate deterministic values based on text content
        for (i, ch) in text.chars().enumerate() {
            let idx = i % self.dimension;
            values[idx] += (ch as u32) as f32 / 1000.0;
        }

        // Normalize the vector to unit length
        let norm: f32 = values.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut values {
                *v /= norm;
            }
        }

        values
    }
}

impl Default for MockEmbedAdapter {
    fn default() -> Self {
        // Use 384 dimensions to match all-MiniLM-L6-v2
        Self::new(384)
    }
}

impl EmbeddingPort for MockEmbedAdapter {
    fn embed(&self, text: &str) -> Result<Embedding, EmbeddingError> {
        if text.is_empty() {
            return Err(EmbeddingError::InvalidInput(
                "Text cannot be empty".to_string(),
            ));
        }

        let values = self.generate_embedding(text);
        Ok(Embedding::new(values))
    }

    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>, EmbeddingError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Check if all texts are empty - this is an error condition
        // as there would be nothing meaningful to embed
        if texts.iter().all(|t| t.is_empty()) {
            return Err(EmbeddingError::InvalidInput(
                "All texts are empty".to_string(),
            ));
        }

        // Process in parallel using Rayon
        // Empty texts within a mixed batch get a zero vector of the correct dimension
        // This preserves index alignment with the input
        let embeddings: Vec<Embedding> = texts
            .par_iter()
            .map(|text| {
                if text.is_empty() {
                    Embedding::new(vec![0.0; self.dimension])
                } else {
                    Embedding::new(self.generate_embedding(text))
                }
            })
            .collect();

        Ok(embeddings)
    }

    fn search(
        &self,
        query: &Embedding,
        candidates: &[(usize, Embedding)],
        top_k: usize,
    ) -> Vec<SearchResult<usize>> {
        if candidates.is_empty() || top_k == 0 {
            return Vec::new();
        }

        // Calculate similarities in parallel
        let mut scores: Vec<SearchResult<usize>> = candidates
            .par_iter()
            .map(|(id, embedding)| {
                let score = query.cosine_similarity(embedding);
                SearchResult::new(*id, score)
            })
            .collect();

        // Sort by score descending
        scores.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Return top_k results
        scores.truncate(top_k);
        scores
    }

    fn embedding_dimension(&self) -> usize {
        self.dimension
    }

    fn model_name(&self) -> &str {
        "mock-embedding-model"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_adapter_creation() {
        let adapter = MockEmbedAdapter::new(128);
        assert_eq!(adapter.embedding_dimension(), 128);
    }

    #[test]
    fn test_mock_adapter_default() {
        let adapter = MockEmbedAdapter::default();
        assert_eq!(adapter.embedding_dimension(), 384);
    }

    #[test]
    fn test_mock_embed_single() {
        let adapter = MockEmbedAdapter::default();
        let embedding = adapter.embed("Hello, world!");

        assert!(embedding.is_ok());
        let embedding = embedding.unwrap();
        assert_eq!(embedding.dimension, 384);
        assert_eq!(embedding.values.len(), 384);
    }

    #[test]
    fn test_mock_embed_deterministic() {
        let adapter = MockEmbedAdapter::default();
        let text = "Hello, world!";

        let embedding1 = adapter.embed(text).unwrap();
        let embedding2 = adapter.embed(text).unwrap();

        assert_eq!(embedding1.values, embedding2.values);
    }

    #[test]
    fn test_mock_embed_empty_text() {
        let adapter = MockEmbedAdapter::default();
        let result = adapter.embed("");

        assert!(result.is_err());
    }

    #[test]
    fn test_mock_embed_batch() {
        let adapter = MockEmbedAdapter::default();
        let texts = vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()];

        let embeddings = adapter.embed_batch(&texts);
        assert!(embeddings.is_ok());

        let embeddings = embeddings.unwrap();
        assert_eq!(embeddings.len(), 3);

        for embedding in &embeddings {
            assert_eq!(embedding.dimension, 384);
        }
    }

    #[test]
    fn test_mock_embed_batch_empty() {
        let adapter = MockEmbedAdapter::default();
        let texts: Vec<String> = vec![];

        let result = adapter.embed_batch(&texts);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_mock_embed_batch_all_empty_texts() {
        let adapter = MockEmbedAdapter::default();
        let texts = vec!["".to_string(), "".to_string()];

        let result = adapter.embed_batch(&texts);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_similar_texts() {
        let adapter = MockEmbedAdapter::default();

        let embedding1 = adapter.embed("Hello world").unwrap();
        let embedding2 = adapter.embed("Hello world!").unwrap();
        let embedding3 = adapter.embed("Completely different text").unwrap();

        let sim_same = embedding1.cosine_similarity(&embedding2);
        let sim_diff = embedding1.cosine_similarity(&embedding3);

        // Similar texts should have higher similarity than different texts
        assert!(
            sim_same > sim_diff,
            "Expected similar texts to have higher similarity: {} > {}",
            sim_same,
            sim_diff
        );
    }

    #[test]
    fn test_mock_search() {
        let adapter = MockEmbedAdapter::default();

        let texts = vec![
            "Hello world".to_string(),
            "Goodbye world".to_string(),
            "Hello there".to_string(),
        ];

        let embeddings = adapter.embed_batch(&texts).unwrap();
        let candidates: Vec<(usize, Embedding)> = embeddings.into_iter().enumerate().collect();

        let query = adapter.embed("Hello world").unwrap();
        let results = adapter.search(&query, &candidates, 2);

        assert_eq!(results.len(), 2);
        // First result should be the exact match (index 0)
        assert_eq!(results[0].item, 0);
        assert!((results[0].score - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_mock_search_empty_candidates() {
        let adapter = MockEmbedAdapter::default();
        let query = adapter.embed("test").unwrap();
        let candidates: Vec<(usize, Embedding)> = vec![];

        let results = adapter.search(&query, &candidates, 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_mock_search_top_k_zero() {
        let adapter = MockEmbedAdapter::default();
        let query = adapter.embed("test").unwrap();
        let embedding = adapter.embed("candidate").unwrap();
        let candidates = vec![(0, embedding)];

        let results = adapter.search(&query, &candidates, 0);
        assert!(results.is_empty());
    }

    #[test]
    fn test_mock_model_name() {
        let adapter = MockEmbedAdapter::default();
        assert_eq!(adapter.model_name(), "mock-embedding-model");
    }
}
