//! SavedContext entity for storing context with tags

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a saved context for later retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedContext {
    /// Unique identifier
    pub id: Uuid,

    /// Context name/title
    pub name: String,

    /// The context content
    pub content: String,

    /// Tags for categorization and retrieval
    pub tags: Vec<String>,

    /// When the context was saved
    pub created_at: DateTime<Utc>,

    /// Session ID this context belongs to
    pub session_id: Option<Uuid>,

    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

impl SavedContext {
    /// Creates a new SavedContext
    pub fn new(name: String, content: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            content,
            tags,
            created_at: Utc::now(),
            session_id: None,
            metadata: None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saved_context_creation() {
        let context = SavedContext::new(
            "Architecture Context".to_string(),
            "We use hexagonal architecture...".to_string(),
            vec!["architecture".to_string(), "design".to_string()],
        );

        assert_eq!(context.name, "Architecture Context");
        assert_eq!(context.tags.len(), 2);
    }
}
