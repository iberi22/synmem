//! Memory entity for storing and retrieving scraped content

use serde::{Deserialize, Serialize};

/// Represents a memory entry for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier
    pub id: String,
    /// Source URL
    pub url: String,
    /// Title of the page/content
    pub title: String,
    /// Text content
    pub content: String,
    /// Optional embedding vector
    pub embedding: Option<Vec<f32>>,
    /// Timestamp when stored
    pub created_at: i64,
    /// Optional tags for categorization
    pub tags: Vec<String>,
}

/// Search result from memory query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResult {
    /// The memory entry
    pub memory: Memory,
    /// Relevance score (0.0 - 1.0)
    pub score: f32,
}

impl Memory {
    /// Creates a new memory entry
    pub fn new(
        id: impl Into<String>,
        url: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            title: title.into(),
            content: content.into(),
            embedding: None,
            created_at: chrono_timestamp(),
            tags: Vec::new(),
        }
    }
}

fn chrono_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
