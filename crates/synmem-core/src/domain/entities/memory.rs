//! Memory entity for storing scraped content with metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a memory entry - any stored content from scraping or manual save
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Unique identifier
    pub id: Uuid,

    /// Type of content (page, chat, context, etc.)
    pub content_type: ContentType,

    /// Title or summary of the content
    pub title: String,

    /// The actual content
    pub content: String,

    /// Source URL if applicable
    pub source_url: Option<String>,

    /// Source type (gemini, chatgpt, claude, twitter, web, etc.)
    pub source: String,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Session ID this memory belongs to
    pub session_id: Option<Uuid>,

    /// When the content was created/scraped
    pub created_at: DateTime<Utc>,

    /// When the memory was last updated
    pub updated_at: DateTime<Utc>,

    /// Additional metadata as JSON
    pub metadata: Option<serde_json::Value>,
}

/// Type of content stored in memory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// Scraped web page
    Page,
    /// AI chat conversation
    Chat,
    /// Saved context
    Context,
    /// Tweet or thread
    Tweet,
    /// Other content
    Other,
}

impl Memory {
    /// Creates a new Memory entry
    pub fn new(
        content_type: ContentType,
        title: String,
        content: String,
        source: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content_type,
            title,
            content,
            source_url: None,
            source,
            tags: Vec::new(),
            session_id: None,
            created_at: now,
            updated_at: now,
            metadata: None,
        }
    }

    /// Sets the source URL
    pub fn with_source_url(mut self, url: impl Into<String>) -> Self {
        self.source_url = Some(url.into());
        self
    }

    /// Sets the tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Sets the session ID
    pub fn with_session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Sets the metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Page => write!(f, "page"),
            ContentType::Chat => write!(f, "chat"),
            ContentType::Context => write!(f, "context"),
            ContentType::Tweet => write!(f, "tweet"),
            ContentType::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "page" => Ok(ContentType::Page),
            "chat" => Ok(ContentType::Chat),
            "context" => Ok(ContentType::Context),
            "tweet" => Ok(ContentType::Tweet),
            "other" => Ok(ContentType::Other),
            _ => Err(format!("Unknown content type: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new(
            ContentType::Chat,
            "Test Chat".to_string(),
            "Hello, world!".to_string(),
            "gemini".to_string(),
        );

        assert_eq!(memory.title, "Test Chat");
        assert_eq!(memory.content, "Hello, world!");
        assert_eq!(memory.source, "gemini");
        assert_eq!(memory.content_type, ContentType::Chat);
    }

    #[test]
    fn test_memory_builder() {
        let session_id = Uuid::new_v4();
        let memory = Memory::new(
            ContentType::Page,
            "Web Page".to_string(),
            "Content".to_string(),
            "web".to_string(),
        )
        .with_source_url("https://example.com")
        .with_tags(vec!["test".to_string(), "example".to_string()])
        .with_session_id(session_id);

        assert_eq!(memory.source_url, Some("https://example.com".to_string()));
        assert_eq!(memory.tags.len(), 2);
        assert_eq!(memory.session_id, Some(session_id));
    }
}
