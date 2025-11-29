//! Scraped Page Entity
//!
//! Represents a scraped web page with its content and metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata about a scraped page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageMetadata {
    /// Meta description of the page.
    pub description: Option<String>,
    /// Meta keywords.
    pub keywords: Option<Vec<String>>,
    /// Author of the content.
    pub author: Option<String>,
    /// Publication date if available.
    pub published_at: Option<DateTime<Utc>>,
    /// Language of the page (e.g., "en", "es").
    pub language: Option<String>,
    /// Open Graph metadata.
    #[serde(default)]
    pub og_metadata: HashMap<String, String>,
    /// Custom metadata fields.
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

impl PageMetadata {
    /// Creates new empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the author.
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sets the language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Adds a custom metadata field.
    pub fn with_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
}

/// A scraped web page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// The URL of the scraped page.
    pub url: String,
    /// The title of the page.
    pub title: Option<String>,
    /// The main content of the page (cleaned text).
    pub content: String,
    /// HTML content (if preserved).
    pub html: Option<String>,
    /// Page metadata.
    pub metadata: PageMetadata,
    /// When the page was scraped.
    pub timestamp: DateTime<Utc>,
    /// Links found on the page.
    #[serde(default)]
    pub links: Vec<String>,
    /// Byte size of the content.
    pub content_length: usize,
}

impl ScrapedPage {
    /// Creates a new scraped page with the given URL and content.
    pub fn new(url: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        let content_length = content.len();
        Self {
            url: url.into(),
            title: None,
            content,
            html: None,
            metadata: PageMetadata::default(),
            timestamp: Utc::now(),
            links: Vec::new(),
            content_length,
        }
    }

    /// Sets the title of the page.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the HTML content.
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Sets the metadata.
    pub fn with_metadata(mut self, metadata: PageMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds links to the page.
    pub fn with_links(mut self, links: Vec<String>) -> Self {
        self.links = links;
        self
    }

    /// Returns true if the page has a title.
    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    /// Returns the word count of the content.
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    /// Returns a summary of the content (first N characters).
    pub fn summary(&self, max_chars: usize) -> &str {
        if self.content.len() <= max_chars {
            &self.content
        } else {
            // Find a word boundary near max_chars
            match self.content[..max_chars].rfind(' ') {
                Some(pos) => &self.content[..pos],
                None => &self.content[..max_chars],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraped_page_creation() {
        let page = ScrapedPage::new("https://example.com", "Hello, World!");
        assert_eq!(page.url, "https://example.com");
        assert_eq!(page.content, "Hello, World!");
        assert_eq!(page.content_length, 13);
    }

    #[test]
    fn test_scraped_page_with_title() {
        let page = ScrapedPage::new("https://example.com", "Content")
            .with_title("Example Domain");
        assert_eq!(page.title, Some("Example Domain".to_string()));
    }

    #[test]
    fn test_scraped_page_word_count() {
        let page = ScrapedPage::new("https://example.com", "This is a test page with several words");
        assert_eq!(page.word_count(), 8);
    }

    #[test]
    fn test_scraped_page_summary() {
        let content = "This is a long piece of content that should be truncated";
        let page = ScrapedPage::new("https://example.com", content);
        let summary = page.summary(20);
        assert!(summary.len() <= 20);
    }

    #[test]
    fn test_page_metadata_builder() {
        let metadata = PageMetadata::new()
            .with_description("A test page")
            .with_author("Test Author")
            .with_language("en")
            .with_custom("source", "manual");

        assert_eq!(metadata.description, Some("A test page".to_string()));
        assert_eq!(metadata.author, Some("Test Author".to_string()));
        assert_eq!(metadata.custom.get("source"), Some(&"manual".to_string()));
    }

    #[test]
    fn test_scraped_page_serialization() {
        let page = ScrapedPage::new("https://example.com", "Test content")
            .with_title("Test Page");
        let json = serde_json::to_string(&page).unwrap();
        let deserialized: ScrapedPage = serde_json::from_str(&json).unwrap();
        assert_eq!(page.url, deserialized.url);
        assert_eq!(page.title, deserialized.title);
    }
}
