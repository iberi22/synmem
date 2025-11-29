//! Search memory MCP tool implementation
//!
//! Provides the `search_memory` tool for hybrid semantic search

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use synmem_core::{MemoryQuery, SearchResult, domain::entities::SearchSource};

/// Input parameters for the search_memory tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoryInput {
    /// The search query
    pub query: String,
    /// Maximum number of results to return (default: 10)
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Search mode: "hybrid", "fts", or "vector" (default: "hybrid")
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_limit() -> usize {
    10
}

fn default_mode() -> String {
    "hybrid".to_string()
}

/// Output result for the search_memory tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoryOutput {
    /// List of search results
    pub results: Vec<SearchMemoryResult>,
    /// Total number of results found
    pub total: usize,
    /// The query that was executed
    pub query: String,
    /// The search mode used
    pub mode: String,
}

/// A single search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoryResult {
    /// Memory ID
    pub id: String,
    /// Content text
    pub content: String,
    /// Optional title
    pub title: Option<String>,
    /// Optional source URL
    pub source_url: Option<String>,
    /// Relevance score (0.0 to 1.0)
    pub score: f32,
    /// Source of this result (fts, vector, or hybrid)
    pub source: String,
    /// Optional snippet with context
    pub snippet: Option<String>,
}

impl From<SearchResult> for SearchMemoryResult {
    fn from(result: SearchResult) -> Self {
        Self {
            id: result.memory.id,
            content: result.memory.content,
            title: result.memory.title,
            source_url: result.memory.source_url,
            score: result.score,
            source: match result.source {
                SearchSource::FullText => "fts".to_string(),
                SearchSource::Vector => "vector".to_string(),
                SearchSource::Hybrid => "hybrid".to_string(),
            },
            snippet: result.snippet,
        }
    }
}

/// The search_memory MCP tool
pub struct SearchMemoryTool<Q: MemoryQuery> {
    query_service: Arc<Q>,
}

impl<Q: MemoryQuery> SearchMemoryTool<Q> {
    /// Create a new SearchMemoryTool
    pub fn new(query_service: Arc<Q>) -> Self {
        Self { query_service }
    }

    /// Tool name
    pub fn name() -> &'static str {
        "search_memory"
    }

    /// Tool description
    pub fn description() -> &'static str {
        "Search stored memories using semantic search. Supports hybrid (FTS + vector), \
         full-text only, or vector-only search modes."
    }

    /// JSON schema for the tool input
    pub fn input_schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query text"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "default": 10,
                    "minimum": 1,
                    "maximum": 100
                },
                "mode": {
                    "type": "string",
                    "description": "Search mode: 'hybrid' (default), 'fts', or 'vector'",
                    "enum": ["hybrid", "fts", "vector"],
                    "default": "hybrid"
                }
            },
            "required": ["query"]
        })
    }

    /// Execute the search_memory tool
    pub async fn execute(&self, input: SearchMemoryInput) -> anyhow::Result<SearchMemoryOutput> {
        let results = match input.mode.as_str() {
            "fts" => {
                self.query_service
                    .search_fts(&input.query, input.limit)
                    .await?
            }
            "vector" => {
                self.query_service
                    .search_vector(&input.query, input.limit)
                    .await?
            }
            _ => self.query_service.search(&input.query, input.limit).await?,
        };

        let total = results.len();
        let results: Vec<SearchMemoryResult> = results.into_iter().map(Into::into).collect();

        Ok(SearchMemoryOutput {
            results,
            total,
            query: input.query,
            mode: input.mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use synmem_core::{Memory, SearchResult as CoreSearchResult};

    struct MockQueryService {
        results: Mutex<Vec<CoreSearchResult>>,
    }

    impl MockQueryService {
        fn new(results: Vec<CoreSearchResult>) -> Self {
            Self {
                results: Mutex::new(results),
            }
        }
    }

    #[async_trait]
    impl MemoryQuery for MockQueryService {
        async fn search(
            &self,
            _query: &str,
            _limit: usize,
        ) -> anyhow::Result<Vec<CoreSearchResult>> {
            Ok(self.results.lock().unwrap().clone())
        }

        async fn search_fts(
            &self,
            _query: &str,
            _limit: usize,
        ) -> anyhow::Result<Vec<CoreSearchResult>> {
            Ok(self.results.lock().unwrap().clone())
        }

        async fn search_vector(
            &self,
            _query: &str,
            _limit: usize,
        ) -> anyhow::Result<Vec<CoreSearchResult>> {
            Ok(self.results.lock().unwrap().clone())
        }

        async fn get_recent(&self, _limit: usize) -> anyhow::Result<Vec<CoreSearchResult>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_search_memory_tool() {
        let mem = Memory::new("1", "Rust programming language").with_title("Rust Guide");
        let result = CoreSearchResult::from_fts(mem, 0.9, 1);

        let service = Arc::new(MockQueryService::new(vec![result]));
        let tool = SearchMemoryTool::new(service);

        let input = SearchMemoryInput {
            query: "rust".to_string(),
            limit: 10,
            mode: "hybrid".to_string(),
        };

        let output = tool.execute(input).await.unwrap();

        assert_eq!(output.total, 1);
        assert_eq!(output.results[0].id, "1");
        assert_eq!(output.results[0].title, Some("Rust Guide".to_string()));
        assert_eq!(output.results[0].score, 0.9);
    }

    #[test]
    fn test_tool_metadata() {
        assert_eq!(
            SearchMemoryTool::<MockQueryService>::name(),
            "search_memory"
        );
        assert!(!SearchMemoryTool::<MockQueryService>::description().is_empty());

        let schema = SearchMemoryTool::<MockQueryService>::input_schema();
        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());
    }
}
