//! Memory entity representing stored content

use serde::{Deserialize, Serialize};

/// Represents a stored memory item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    /// Unique identifier
    pub id: String,
    /// The text content
    pub content: String,
    /// Source URL if applicable
    pub source_url: Option<String>,
    /// Title or headline
    pub title: Option<String>,
    /// Timestamp when the memory was created
    pub created_at: i64,
    /// Optional metadata
    pub metadata: Option<serde_json::Value>,
}

impl Memory {
    /// Create a new memory instance
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            source_url: None,
            title: None,
            created_at: 0,
            metadata: None,
        }
    }

    /// Set the source URL
    pub fn with_source_url(mut self, url: impl Into<String>) -> Self {
        self.source_url = Some(url.into());
        self
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the creation timestamp
    pub fn with_created_at(mut self, timestamp: i64) -> Self {
        self.created_at = timestamp;
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new("test-1", "Test content")
            .with_title("Test Title")
            .with_source_url("https://example.com")
            .with_created_at(1234567890);

        assert_eq!(memory.id, "test-1");
        assert_eq!(memory.content, "Test content");
        assert_eq!(memory.title, Some("Test Title".to_string()));
        assert_eq!(memory.source_url, Some("https://example.com".to_string()));
        assert_eq!(memory.created_at, 1234567890);
    }
}
