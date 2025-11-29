//! SearchResult entity for memory search results

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a search result from memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// ID of the matched memory entry
    pub id: Uuid,

    /// Title of the matched content
    pub title: String,

    /// Snippet showing the matched text with context
    pub snippet: String,

    /// Source type (gemini, chatgpt, claude, etc.)
    pub source: String,

    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,

    /// URL if applicable
    pub url: Option<String>,

    /// Content type
    pub content_type: String,
}

impl SearchResult {
    /// Creates a new SearchResult
    pub fn new(
        id: Uuid,
        title: String,
        snippet: String,
        source: String,
        relevance: f64,
    ) -> Self {
        Self {
            id,
            title,
            snippet,
            source,
            relevance,
            url: None,
            content_type: "other".to_string(),
        }
    }

    /// Sets the URL
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the content type
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = content_type.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult::new(
            Uuid::new_v4(),
            "Test Result".to_string(),
            "...matched text...".to_string(),
            "gemini".to_string(),
            0.95,
        )
        .with_url("https://example.com")
        .with_content_type("chat");

        assert_eq!(result.title, "Test Result");
        assert_eq!(result.relevance, 0.95);
        assert_eq!(result.url, Some("https://example.com".to_string()));
        assert_eq!(result.content_type, "chat");
    }
}
