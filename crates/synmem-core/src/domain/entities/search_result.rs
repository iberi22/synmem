//! SearchResult entity representing search results

use serde::{Deserialize, Serialize};

use super::Memory;

/// Source of the search result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchSource {
    /// Result from full-text search
    FullText,
    /// Result from vector similarity search
    Vector,
    /// Result from both search methods
    Hybrid,
}

/// Represents a search result with ranking information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// The memory item
    pub memory: Memory,
    /// Combined relevance score (0.0 to 1.0)
    pub score: f32,
    /// Source of this result
    pub source: SearchSource,
    /// Full-text search score (if available)
    pub fts_score: Option<f32>,
    /// Vector similarity score (if available)
    pub vector_score: Option<f32>,
    /// Rank from FTS results (1-indexed, lower is better)
    pub fts_rank: Option<usize>,
    /// Rank from vector results (1-indexed, lower is better)
    pub vector_rank: Option<usize>,
    /// Snippet of matching text with context
    pub snippet: Option<String>,
}

impl SearchResult {
    /// Create a new search result from FTS
    pub fn from_fts(memory: Memory, score: f32, rank: usize) -> Self {
        Self {
            memory,
            score,
            source: SearchSource::FullText,
            fts_score: Some(score),
            vector_score: None,
            fts_rank: Some(rank),
            vector_rank: None,
            snippet: None,
        }
    }

    /// Create a new search result from vector search
    pub fn from_vector(memory: Memory, score: f32, rank: usize) -> Self {
        Self {
            memory,
            score,
            source: SearchSource::Vector,
            fts_score: None,
            vector_score: Some(score),
            fts_rank: None,
            vector_rank: Some(rank),
            snippet: None,
        }
    }

    /// Create a hybrid result by merging FTS and vector results
    pub fn merge(
        fts_result: &SearchResult,
        vector_result: &SearchResult,
        combined_score: f32,
    ) -> Self {
        Self {
            memory: fts_result.memory.clone(),
            score: combined_score,
            source: SearchSource::Hybrid,
            fts_score: fts_result.fts_score,
            vector_score: vector_result.vector_score,
            fts_rank: fts_result.fts_rank,
            vector_rank: vector_result.vector_rank,
            snippet: fts_result
                .snippet
                .clone()
                .or_else(|| vector_result.snippet.clone()),
        }
    }

    /// Set the snippet
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.memory.id == other.memory.id
    }
}

impl Eq for SearchResult {}

impl std::hash::Hash for SearchResult {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.memory.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_from_fts() {
        let memory = Memory::new("1", "test content");
        let result = SearchResult::from_fts(memory, 0.9, 1);

        assert_eq!(result.source, SearchSource::FullText);
        assert_eq!(result.fts_score, Some(0.9));
        assert_eq!(result.fts_rank, Some(1));
        assert!(result.vector_score.is_none());
    }

    #[test]
    fn test_search_result_from_vector() {
        let memory = Memory::new("1", "test content");
        let result = SearchResult::from_vector(memory, 0.85, 2);

        assert_eq!(result.source, SearchSource::Vector);
        assert_eq!(result.vector_score, Some(0.85));
        assert_eq!(result.vector_rank, Some(2));
        assert!(result.fts_score.is_none());
    }

    #[test]
    fn test_search_result_merge() {
        let memory = Memory::new("1", "test content");
        let fts = SearchResult::from_fts(memory.clone(), 0.9, 1);
        let vector = SearchResult::from_vector(memory, 0.85, 2);
        let merged = SearchResult::merge(&fts, &vector, 0.88);

        assert_eq!(merged.source, SearchSource::Hybrid);
        assert_eq!(merged.fts_score, Some(0.9));
        assert_eq!(merged.vector_score, Some(0.85));
        assert_eq!(merged.score, 0.88);
    }
}
