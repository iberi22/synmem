//! ScrapedPage entity representing extracted web page content

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a scraped web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// Unique identifier
    pub id: Uuid,

    /// Page URL
    pub url: String,

    /// Page title
    pub title: String,

    /// Extracted text content
    pub content: String,

    /// HTML content (optional)
    pub html: Option<String>,

    /// When the page was scraped
    pub scraped_at: DateTime<Utc>,

    /// Session ID this page belongs to
    pub session_id: Option<Uuid>,

    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

impl ScrapedPage {
    /// Creates a new ScrapedPage
    pub fn new(url: String, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            title,
            content,
            html: None,
            scraped_at: Utc::now(),
            session_id: None,
            metadata: None,
        }
    }

    /// Sets the HTML content
    pub fn with_html(mut self, html: String) -> Self {
        self.html = Some(html);
        self
    }

    /// Sets the session ID
    pub fn with_session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraped_page_creation() {
        let page = ScrapedPage::new(
            "https://example.com".to_string(),
            "Example".to_string(),
            "Page content".to_string(),
        );

        assert_eq!(page.url, "https://example.com");
        assert_eq!(page.title, "Example");
        assert!(page.html.is_none());
    }
}
