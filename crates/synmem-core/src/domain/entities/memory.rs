//! Memory entity - represents a stored memory/knowledge item

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a memory/knowledge item stored in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    /// Unique identifier for the memory
    pub id: String,
    /// Source of the memory (e.g., URL, file path, user input)
    pub source: Option<String>,
    /// The actual content/text of the memory
    pub content: String,
    /// Reference to an embedding vector (if generated)
    pub embedding_id: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Timestamp when the memory was created
    pub created_at: DateTime<Utc>,
}

impl Memory {
    /// Creates a new Memory with the given content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source: None,
            content: content.into(),
            embedding_id: None,
            tags: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Sets the source of the memory
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Sets the embedding ID
    pub fn with_embedding_id(mut self, embedding_id: impl Into<String>) -> Self {
        self.embedding_id = Some(embedding_id.into());
        self
    }

    /// Adds a tag to the memory
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Sets all tags at once
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}
