//! Memory Entity
//!
//! Represents a stored memory/knowledge item with embedding support.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A memory/knowledge item stored in the system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier for the memory.
    pub id: Uuid,
    /// Source of the memory (URL, file path, user input, etc.).
    pub source: String,
    /// The content of the memory.
    pub content: String,
    /// Reference to the embedding vector in the vector store.
    pub embedding_id: Option<String>,
    /// Tags for categorization and filtering.
    #[serde(default)]
    pub tags: Vec<String>,
    /// When the memory was created.
    pub created_at: DateTime<Utc>,
    /// When the memory was last accessed.
    pub last_accessed_at: Option<DateTime<Utc>>,
    /// Relevance score (for search results).
    pub relevance_score: Option<f64>,
    /// Optional title or summary.
    pub title: Option<String>,
}

impl Memory {
    /// Creates a new memory with the given source and content.
    pub fn new(source: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            source: source.into(),
            content: content.into(),
            embedding_id: None,
            tags: Vec::new(),
            created_at: Utc::now(),
            last_accessed_at: None,
            relevance_score: None,
            title: None,
        }
    }

    /// Sets the embedding ID.
    pub fn with_embedding_id(mut self, embedding_id: impl Into<String>) -> Self {
        self.embedding_id = Some(embedding_id.into());
        self
    }

    /// Adds tags to the memory.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Adds a single tag.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }

    /// Sets the title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the relevance score (typically from search results).
    pub fn with_relevance_score(mut self, score: f64) -> Self {
        self.relevance_score = Some(score);
        self
    }

    /// Updates the last accessed timestamp to now.
    pub fn touch(&mut self) {
        self.last_accessed_at = Some(Utc::now());
    }

    /// Returns true if the memory has an embedding.
    pub fn has_embedding(&self) -> bool {
        self.embedding_id.is_some()
    }

    /// Returns true if the memory has the given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Returns the content length in characters.
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

impl Eq for Memory {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new("https://example.com", "This is a memory");
        assert_eq!(memory.source, "https://example.com");
        assert_eq!(memory.content, "This is a memory");
        assert!(memory.embedding_id.is_none());
        assert!(memory.tags.is_empty());
    }

    #[test]
    fn test_memory_with_tags() {
        let memory = Memory::new("source", "content")
            .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert!(memory.has_tag("tag1"));
        assert!(memory.has_tag("tag2"));
        assert!(!memory.has_tag("tag3"));
    }

    #[test]
    fn test_memory_add_tag() {
        let mut memory = Memory::new("source", "content");
        memory.add_tag("new-tag");
        assert!(memory.has_tag("new-tag"));
    }

    #[test]
    fn test_memory_with_embedding() {
        let memory = Memory::new("source", "content")
            .with_embedding_id("emb-12345");
        assert!(memory.has_embedding());
        assert_eq!(memory.embedding_id, Some("emb-12345".to_string()));
    }

    #[test]
    fn test_memory_touch() {
        let mut memory = Memory::new("source", "content");
        assert!(memory.last_accessed_at.is_none());

        memory.touch();
        assert!(memory.last_accessed_at.is_some());
    }

    #[test]
    fn test_memory_serialization() {
        let memory = Memory::new("https://example.com", "Test content")
            .with_title("Test Memory")
            .with_tags(vec!["test".to_string()]);

        let json = serde_json::to_string(&memory).unwrap();
        let deserialized: Memory = serde_json::from_str(&json).unwrap();

        assert_eq!(memory.id, deserialized.id);
        assert_eq!(memory.source, deserialized.source);
        assert_eq!(memory.content, deserialized.content);
        assert_eq!(memory.tags, deserialized.tags);
    }
}
