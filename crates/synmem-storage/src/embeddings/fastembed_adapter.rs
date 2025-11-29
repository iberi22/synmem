//! FastEmbed adapter implementing the EmbeddingPort trait.
//!
//! This adapter uses the fastembed library for local embedding generation
//! with the all-MiniLM-L6-v2 model by default.
//!
//! # Requirements
//!
//! This adapter requires the ONNX runtime, which will be downloaded automatically
//! during the build process. Ensure you have internet access during the first build.

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use rayon::prelude::*;
use synmem_core::{Embedding, EmbeddingError, EmbeddingPort, SearchResult};
use tracing::{debug, instrument};

/// Configuration for the FastEmbed adapter.
#[derive(Debug, Clone)]
pub struct FastEmbedConfig {
    /// The embedding model to use.
    pub model: EmbeddingModel,
    /// Whether to show download progress.
    pub show_download_progress: bool,
    /// Batch size for parallel processing.
    pub batch_size: usize,
}

impl Default for FastEmbedConfig {
    fn default() -> Self {
        Self {
            model: EmbeddingModel::AllMiniLML6V2,
            show_download_progress: true,
            batch_size: 32,
        }
    }
}

/// FastEmbed adapter for local embedding generation.
///
/// Uses the fastembed library with Rayon for parallel batch processing.
pub struct FastEmbedAdapter {
    model: TextEmbedding,
    config: FastEmbedConfig,
}

impl FastEmbedAdapter {
    /// Creates a new FastEmbedAdapter with the default model (all-MiniLM-L6-v2).
    pub fn new() -> Result<Self, EmbeddingError> {
        Self::with_config(FastEmbedConfig::default())
    }

    /// Creates a new FastEmbedAdapter with custom configuration.
    pub fn with_config(config: FastEmbedConfig) -> Result<Self, EmbeddingError> {
        let init_options = InitOptions::new(config.model.clone())
            .with_show_download_progress(config.show_download_progress);

        let model = TextEmbedding::try_new(init_options).map_err(|e| {
            EmbeddingError::InitializationError(format!("Failed to initialize FastEmbed: {}", e))
        })?;

        Ok(Self { model, config })
    }

    /// Creates a new FastEmbedAdapter with a specific model.
    pub fn with_model(model: EmbeddingModel) -> Result<Self, EmbeddingError> {
        Self::with_config(FastEmbedConfig {
            model,
            ..Default::default()
        })
    }
}

impl EmbeddingPort for FastEmbedAdapter {
    #[instrument(skip(self, text), fields(text_len = text.len()))]
    fn embed(&self, text: &str) -> Result<Embedding, EmbeddingError> {
        if text.is_empty() {
            return Err(EmbeddingError::InvalidInput(
                "Text cannot be empty".to_string(),
            ));
        }

        let embeddings = self
            .model
            .embed(vec![text], None)
            .map_err(|e| EmbeddingError::GenerationError(format!("Embedding failed: {}", e)))?;

        let values = embeddings
            .into_iter()
            .next()
            .ok_or_else(|| EmbeddingError::GenerationError("No embedding generated".to_string()))?;

        Ok(Embedding::new(values))
    }

    #[instrument(skip(self, texts), fields(batch_size = texts.len()))]
    fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>, EmbeddingError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Filter out empty texts and track their indices
        let non_empty: Vec<(usize, &String)> = texts
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.is_empty())
            .collect();

        if non_empty.is_empty() {
            return Err(EmbeddingError::InvalidInput(
                "All texts are empty".to_string(),
            ));
        }

        debug!(
            "Processing {} non-empty texts in batches of {}",
            non_empty.len(),
            self.config.batch_size
        );

        // Process in parallel chunks using Rayon
        let text_refs: Vec<&str> = non_empty.iter().map(|(_, t)| t.as_str()).collect();

        // Use par_chunks for parallel processing
        let batch_results: Result<Vec<Vec<Vec<f32>>>, EmbeddingError> = text_refs
            .par_chunks(self.config.batch_size)
            .map(|chunk| {
                self.model
                    .embed(chunk.to_vec(), None)
                    .map_err(|e| EmbeddingError::GenerationError(format!("Batch failed: {}", e)))
            })
            .collect();

        let all_embeddings: Vec<Vec<f32>> = batch_results?.into_iter().flatten().collect();

        // Reconstruct results preserving original order
        let mut results = vec![Embedding::new(vec![]); texts.len()];
        for ((original_idx, _), embedding_values) in non_empty.into_iter().zip(all_embeddings) {
            results[original_idx] = Embedding::new(embedding_values);
        }

        // For empty texts, we'll leave them as empty embeddings
        Ok(results)
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
        // all-MiniLM-L6-v2 produces 384-dimensional embeddings
        384
    }

    fn model_name(&self) -> &str {
        match self.config.model {
            EmbeddingModel::AllMiniLML6V2 => "all-MiniLM-L6-v2",
            EmbeddingModel::AllMiniLML6V2Q => "all-MiniLM-L6-v2-quantized",
            _ => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require the model to be downloaded, so they may take time on first run.

    #[test]
    fn test_adapter_creation() {
        let adapter = FastEmbedAdapter::new();
        assert!(adapter.is_ok());
    }

    #[test]
    fn test_embedding_dimension() {
        let adapter = FastEmbedAdapter::new().unwrap();
        assert_eq!(adapter.embedding_dimension(), 384);
    }

    #[test]
    fn test_model_name() {
        let adapter = FastEmbedAdapter::new().unwrap();
        assert_eq!(adapter.model_name(), "all-MiniLM-L6-v2");
    }

    #[test]
    fn test_embed_single() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let embedding = adapter.embed("Hello, world!");
        assert!(embedding.is_ok());

        let embedding = embedding.unwrap();
        assert_eq!(embedding.dimension, 384);
        assert_eq!(embedding.values.len(), 384);
    }

    #[test]
    fn test_embed_empty_text() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let result = adapter.embed("");
        assert!(result.is_err());
    }

    #[test]
    fn test_embed_batch() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let texts = vec![
            "Hello, world!".to_string(),
            "How are you?".to_string(),
            "Rust is awesome!".to_string(),
        ];

        let embeddings = adapter.embed_batch(&texts);
        assert!(embeddings.is_ok());

        let embeddings = embeddings.unwrap();
        assert_eq!(embeddings.len(), 3);
        for embedding in &embeddings {
            assert_eq!(embedding.dimension, 384);
        }
    }

    #[test]
    fn test_embed_batch_empty() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let texts: Vec<String> = vec![];

        let embeddings = adapter.embed_batch(&texts);
        assert!(embeddings.is_ok());
        assert!(embeddings.unwrap().is_empty());
    }

    #[test]
    fn test_semantic_similarity() {
        let adapter = FastEmbedAdapter::new().unwrap();

        let dog_embedding = adapter.embed("The dog runs in the park").unwrap();
        let cat_embedding = adapter.embed("A cat plays in the garden").unwrap();
        let math_embedding = adapter
            .embed("Mathematical equations and calculus")
            .unwrap();

        // Dog and cat should be more similar (both about animals/outdoor activities)
        let dog_cat_similarity = dog_embedding.cosine_similarity(&cat_embedding);
        let dog_math_similarity = dog_embedding.cosine_similarity(&math_embedding);

        assert!(
            dog_cat_similarity > dog_math_similarity,
            "Expected dog-cat similarity ({}) > dog-math similarity ({})",
            dog_cat_similarity,
            dog_math_similarity
        );
    }

    #[test]
    fn test_search() {
        let adapter = FastEmbedAdapter::new().unwrap();

        // Create some candidate embeddings
        let texts = vec![
            "Programming in Rust".to_string(),
            "Cooking Italian pasta".to_string(),
            "Machine learning algorithms".to_string(),
            "Baking chocolate cake".to_string(),
            "Software development best practices".to_string(),
        ];

        let embeddings = adapter.embed_batch(&texts).unwrap();
        let candidates: Vec<(usize, Embedding)> = embeddings.into_iter().enumerate().collect();

        // Search for programming-related content
        let query = adapter.embed("Computer programming and coding").unwrap();
        let results = adapter.search(&query, &candidates, 3);

        assert_eq!(results.len(), 3);

        // The top results should be programming-related (indices 0, 2, or 4)
        let programming_indices: Vec<usize> = vec![0, 2, 4];
        assert!(
            programming_indices.contains(&results[0].item),
            "Expected top result to be programming-related, got index {}",
            results[0].item
        );
    }

    #[test]
    fn test_search_empty_candidates() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let query = adapter.embed("test query").unwrap();
        let candidates: Vec<(usize, Embedding)> = vec![];

        let results = adapter.search(&query, &candidates, 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_top_k_zero() {
        let adapter = FastEmbedAdapter::new().unwrap();
        let query = adapter.embed("test query").unwrap();
        let embedding = adapter.embed("some text").unwrap();
        let candidates = vec![(0, embedding)];

        let results = adapter.search(&query, &candidates, 0);
        assert!(results.is_empty());
    }
}
