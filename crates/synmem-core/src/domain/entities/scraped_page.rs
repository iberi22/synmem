//! ScrapedPage entity - represents a scraped web page

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a scraped web page with its content and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScrapedPage {
    /// Unique identifier for the page
    pub id: String,
    /// URL of the scraped page
    pub url: String,
    /// Title of the page
    pub title: Option<String>,
    /// Main text content of the page
    pub content: Option<String>,
    /// Additional metadata as key-value pairs
    pub metadata: HashMap<String, serde_json::Value>,
    /// Timestamp when the page was scraped
    pub scraped_at: DateTime<Utc>,
}

impl ScrapedPage {
    /// Creates a new ScrapedPage with the given URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url: url.into(),
            title: None,
            content: None,
            metadata: HashMap::new(),
            scraped_at: Utc::now(),
        }
    }

    /// Sets the title of the page
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the content of the page
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Adds a metadata entry
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}
