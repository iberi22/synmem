//! Search service implementing hybrid search with ranking and deduplication

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::{SearchResult, SearchSource};
use crate::ports::inbound::MemoryQuery;
use crate::ports::outbound::{EmbeddingPort, StoragePort};

/// Configuration for search behavior
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Weight for FTS results in hybrid search (0.0 to 1.0)
    pub fts_weight: f32,
    /// Weight for vector results in hybrid search (0.0 to 1.0)
    pub vector_weight: f32,
    /// Constant k for RRF (Reciprocal Rank Fusion)
    pub rrf_k: f32,
    /// Maximum context window size in characters
    pub max_context_chars: usize,
    /// Whether to enable result deduplication
    pub dedup_enabled: bool,
    /// Similarity threshold for deduplication (0.0 to 1.0)
    pub dedup_threshold: f32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            fts_weight: 0.4,
            vector_weight: 0.6,
            rrf_k: 60.0,
            max_context_chars: 8000,
            dedup_enabled: true,
            dedup_threshold: 0.85,
        }
    }
}

/// Main search service implementing hybrid search
pub struct SearchService<S, E>
where
    S: StoragePort,
    E: EmbeddingPort,
{
    storage: Arc<S>,
    embedder: Arc<E>,
    config: SearchConfig,
}

impl<S, E> SearchService<S, E>
where
    S: StoragePort,
    E: EmbeddingPort,
{
    /// Create a new search service
    pub fn new(storage: Arc<S>, embedder: Arc<E>) -> Self {
        Self {
            storage,
            embedder,
            config: SearchConfig::default(),
        }
    }

    /// Create a search service with custom configuration
    pub fn with_config(storage: Arc<S>, embedder: Arc<E>, config: SearchConfig) -> Self {
        Self {
            storage,
            embedder,
            config,
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &SearchConfig {
        &self.config
    }

    /// Merge FTS and vector results using Reciprocal Rank Fusion (RRF)
    ///
    /// RRF formula: score(d) = Σ 1 / (k + rank(d))
    /// where k is a constant (default 60) and rank is the position in each result set
    fn merge_results_rrf(
        &self,
        fts_results: Vec<SearchResult>,
        vector_results: Vec<SearchResult>,
    ) -> Vec<SearchResult> {
        let mut scores: HashMap<String, f32> = HashMap::new();
        let mut fts_map: HashMap<String, SearchResult> = HashMap::new();
        let mut vector_map: HashMap<String, SearchResult> = HashMap::new();

        // Calculate RRF scores for FTS results
        for (rank, result) in fts_results.into_iter().enumerate() {
            let rrf_score = self.config.fts_weight / (self.config.rrf_k + (rank + 1) as f32);
            let id = result.memory.id.clone();
            *scores.entry(id.clone()).or_insert(0.0) += rrf_score;
            fts_map.insert(id, result);
        }

        // Calculate RRF scores for vector results
        for (rank, result) in vector_results.into_iter().enumerate() {
            let rrf_score = self.config.vector_weight / (self.config.rrf_k + (rank + 1) as f32);
            let id = result.memory.id.clone();
            *scores.entry(id.clone()).or_insert(0.0) += rrf_score;
            vector_map.insert(id, result);
        }

        // Build merged results
        let mut merged: Vec<SearchResult> = scores
            .into_iter()
            .map(|(id, score)| {
                let fts_result = fts_map.get(&id);
                let vector_result = vector_map.get(&id);

                match (fts_result, vector_result) {
                    (Some(fts), Some(vec)) => SearchResult::merge(fts, vec, score),
                    (Some(fts), None) => {
                        let mut result = fts.clone();
                        result.score = score;
                        result
                    }
                    (None, Some(vec)) => {
                        let mut result = vec.clone();
                        result.score = score;
                        result
                    }
                    (None, None) => unreachable!("ID must exist in at least one map"),
                }
            })
            .collect();

        // Sort by combined score (descending)
        merged.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        merged
    }

    /// Deduplicate results based on content similarity
    ///
    /// Uses simple character-level Jaccard similarity for efficiency
    fn deduplicate(&self, results: Vec<SearchResult>) -> Vec<SearchResult> {
        if !self.config.dedup_enabled || results.is_empty() {
            return results;
        }

        let mut unique_results: Vec<SearchResult> = Vec::with_capacity(results.len());
        let mut seen_content_hashes: Vec<HashSet<char>> = Vec::new();

        for result in results {
            let content_chars: HashSet<char> = result.memory.content.chars().collect();

            // Check if this content is too similar to any existing result
            let is_duplicate = seen_content_hashes.iter().any(|existing| {
                let intersection = content_chars.intersection(existing).count();
                let union = content_chars.union(existing).count();
                if union == 0 {
                    return false;
                }
                let similarity = intersection as f32 / union as f32;
                similarity >= self.config.dedup_threshold
            });

            if !is_duplicate {
                seen_content_hashes.push(content_chars);
                unique_results.push(result);
            }
        }

        unique_results
    }

    /// Optimize results for context window
    ///
    /// Ensures total content size fits within the configured context window
    fn optimize_for_context(&self, results: Vec<SearchResult>) -> Vec<SearchResult> {
        let mut optimized = Vec::new();
        let mut total_chars = 0;

        for result in results {
            let content_len = result.memory.content.len();

            if total_chars + content_len <= self.config.max_context_chars {
                total_chars += content_len;
                optimized.push(result);
            } else if total_chars < self.config.max_context_chars {
                // Truncate the last result to fit
                // Reserve 3 chars for "..." suffix
                let remaining = self
                    .config
                    .max_context_chars
                    .saturating_sub(total_chars)
                    .saturating_sub(3);
                if remaining > 100 {
                    // Only include if we can fit meaningful content
                    let mut truncated_result = result;
                    truncated_result.memory.content = truncated_result
                        .memory
                        .content
                        .chars()
                        .take(remaining)
                        .collect();
                    truncated_result.memory.content.push_str("...");
                    optimized.push(truncated_result);
                }
                break;
            } else {
                break;
            }
        }

        optimized
    }
}

#[async_trait]
impl<S, E> MemoryQuery for SearchService<S, E>
where
    S: StoragePort + 'static,
    E: EmbeddingPort + 'static,
{
    /// Perform hybrid search combining FTS and vector similarity
    async fn search(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        // 1. Get FTS results (fetch more than limit for better merging)
        let fts_limit = limit * 2;
        let fts_results = self.storage.full_text_search(query, fts_limit).await?;

        // 2. Get vector results
        let query_embedding = self.embedder.embed(query).await?;
        let vector_results = self
            .storage
            .vector_search(&query_embedding, fts_limit)
            .await?;

        // 3. Merge using RRF
        let merged = self.merge_results_rrf(fts_results, vector_results);

        // 4. Deduplicate
        let deduped = self.deduplicate(merged);

        // 5. Optimize for context window
        let optimized = self.optimize_for_context(deduped);

        // 6. Return top N
        Ok(optimized.into_iter().take(limit).collect())
    }

    /// Perform full-text search only
    async fn search_fts(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let results = self.storage.full_text_search(query, limit).await?;
        let deduped = self.deduplicate(results);
        let optimized = self.optimize_for_context(deduped);
        Ok(optimized)
    }

    /// Perform vector similarity search only
    async fn search_vector(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let query_embedding = self.embedder.embed(query).await?;
        let results = self.storage.vector_search(&query_embedding, limit).await?;
        let deduped = self.deduplicate(results);
        let optimized = self.optimize_for_context(deduped);
        Ok(optimized)
    }

    /// Get recent memories
    async fn get_recent(&self, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let memories = self.storage.get_recent_memories(limit).await?;
        let results: Vec<SearchResult> = memories
            .into_iter()
            .enumerate()
            .map(|(rank, memory)| SearchResult {
                memory,
                score: 1.0 - (rank as f32 / limit as f32),
                source: SearchSource::FullText,
                fts_score: None,
                vector_score: None,
                fts_rank: Some(rank + 1),
                vector_rank: None,
                snippet: None,
            })
            .collect();
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Memory;
    use std::sync::Mutex;

    // Mock storage implementation for testing
    struct MockStorage {
        fts_results: Mutex<Vec<SearchResult>>,
        vector_results: Mutex<Vec<SearchResult>>,
    }

    impl MockStorage {
        fn new(fts: Vec<SearchResult>, vector: Vec<SearchResult>) -> Self {
            Self {
                fts_results: Mutex::new(fts),
                vector_results: Mutex::new(vector),
            }
        }
    }

    #[async_trait]
    impl StoragePort for MockStorage {
        async fn full_text_search(
            &self,
            _query: &str,
            _limit: usize,
        ) -> anyhow::Result<Vec<SearchResult>> {
            Ok(self.fts_results.lock().unwrap().clone())
        }

        async fn vector_search(
            &self,
            _embedding: &[f32],
            _limit: usize,
        ) -> anyhow::Result<Vec<SearchResult>> {
            Ok(self.vector_results.lock().unwrap().clone())
        }

        async fn store_memory(&self, _memory: &Memory, _embedding: &[f32]) -> anyhow::Result<()> {
            Ok(())
        }

        async fn get_memory(&self, _id: &str) -> anyhow::Result<Option<Memory>> {
            Ok(None)
        }

        async fn get_recent_memories(&self, _limit: usize) -> anyhow::Result<Vec<Memory>> {
            Ok(vec![])
        }

        async fn delete_memory(&self, _id: &str) -> anyhow::Result<bool> {
            Ok(false)
        }
    }

    // Mock embedding implementation for testing
    struct MockEmbedder;

    #[async_trait]
    impl EmbeddingPort for MockEmbedder {
        async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
            Ok(vec![0.1, 0.2, 0.3])
        }

        async fn embed_batch(&self, texts: &[&str]) -> anyhow::Result<Vec<Vec<f32>>> {
            Ok(texts.iter().map(|_| vec![0.1, 0.2, 0.3]).collect())
        }

        fn dimension(&self) -> usize {
            3
        }
    }

    #[tokio::test]
    async fn test_hybrid_search_merges_results() {
        let mem1 = Memory::new("1", "Rust programming language");
        let mem2 = Memory::new("2", "Python programming");
        let mem3 = Memory::new("3", "JavaScript framework");

        let fts_results = vec![
            SearchResult::from_fts(mem1.clone(), 0.9, 1),
            SearchResult::from_fts(mem2.clone(), 0.7, 2),
        ];

        let vector_results = vec![
            SearchResult::from_vector(mem2.clone(), 0.95, 1),
            SearchResult::from_vector(mem3.clone(), 0.8, 2),
        ];

        let storage = Arc::new(MockStorage::new(fts_results, vector_results));
        let embedder = Arc::new(MockEmbedder);
        let service = SearchService::new(storage, embedder);

        let results = service.search("programming", 10).await.unwrap();

        // Should have all 3 unique memories
        assert_eq!(results.len(), 3);

        // mem2 should be first (appears in both with high scores)
        assert_eq!(results[0].memory.id, "2");
        assert_eq!(results[0].source, SearchSource::Hybrid);
    }

    #[tokio::test]
    async fn test_deduplication() {
        let mem1 = Memory::new("1", "Hello world");
        let mem2 = Memory::new("2", "Hello world!"); // Very similar to mem1
        let mem3 = Memory::new("3", "Completely different content here");

        let fts_results = vec![
            SearchResult::from_fts(mem1.clone(), 0.9, 1),
            SearchResult::from_fts(mem2.clone(), 0.85, 2),
            SearchResult::from_fts(mem3.clone(), 0.7, 3),
        ];

        let storage = Arc::new(MockStorage::new(fts_results, vec![]));
        let embedder = Arc::new(MockEmbedder);
        let service = SearchService::new(storage, embedder);

        let results = service.search_fts("hello", 10).await.unwrap();

        // Should deduplicate mem2 as it's too similar to mem1
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].memory.id, "1");
        assert_eq!(results[1].memory.id, "3");
    }

    #[tokio::test]
    async fn test_context_window_optimization() {
        // Use different characters to avoid deduplication
        let long_content1: String = (0..5000).map(|i| ((i % 26) as u8 + b'a') as char).collect();
        let long_content2: String = (0..5000).map(|i| ((i % 26) as u8 + b'A') as char).collect();
        let mem1 = Memory::new("1", &long_content1);
        let mem2 = Memory::new("2", &long_content2);
        let mem3 = Memory::new("3", "short content");

        let fts_results = vec![
            SearchResult::from_fts(mem1.clone(), 0.9, 1),
            SearchResult::from_fts(mem2.clone(), 0.8, 2),
            SearchResult::from_fts(mem3.clone(), 0.7, 3),
        ];

        let storage = Arc::new(MockStorage::new(fts_results, vec![]));
        let embedder = Arc::new(MockEmbedder);
        let config = SearchConfig {
            max_context_chars: 8000,
            dedup_enabled: false,
            ..Default::default()
        };
        let service = SearchService::with_config(storage, embedder, config);

        let results = service.search_fts("test", 10).await.unwrap();

        // Should only include results that fit in context window
        let total_chars: usize = results.iter().map(|r| r.memory.content.len()).sum();
        assert!(total_chars <= 8000);

        // With 8000 char limit, we should get:
        // - First result (5000 chars)
        // - Second result truncated (up to 3000 chars) OR just first + third (5000 + 13 < 8000)
        // Since second (5000) doesn't fit, we try to truncate and get ~2997 chars + "..."
        assert!(results.len() >= 1);
        assert!(results.len() <= 2); // Can't fit all 3
    }

    #[test]
    fn test_rrf_scoring() {
        let storage = Arc::new(MockStorage::new(vec![], vec![]));
        let embedder = Arc::new(MockEmbedder);
        let service = SearchService::new(storage, embedder);

        let mem1 = Memory::new("1", "test");
        let mem2 = Memory::new("2", "test2");

        // mem1 ranks #1 in FTS, #2 in vector
        // mem2 ranks #2 in FTS, #1 in vector
        let fts_results = vec![
            SearchResult::from_fts(mem1.clone(), 0.9, 1),
            SearchResult::from_fts(mem2.clone(), 0.8, 2),
        ];

        let vector_results = vec![
            SearchResult::from_vector(mem2.clone(), 0.95, 1),
            SearchResult::from_vector(mem1.clone(), 0.85, 2),
        ];

        let merged = service.merge_results_rrf(fts_results, vector_results);

        // Both should have similar scores since they both rank well
        assert_eq!(merged.len(), 2);

        // Check that hybrid source is set for both
        assert!(merged.iter().all(|r| r.source == SearchSource::Hybrid));
    }
}
