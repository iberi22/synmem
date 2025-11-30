//! Scraped page entity representing extracted content

use serde::{Deserialize, Serialize};

/// Represents a scraped web page with extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// URL of the scraped page
    pub url: String,
    /// Page title
    pub title: Option<String>,
    /// Raw HTML content
    pub html: Option<String>,
    /// Extracted text content
    pub text: Option<String>,
    /// Page screenshot as base64 PNG
    pub screenshot: Option<String>,
    /// Extracted links from the page
    pub links: Vec<Link>,
    /// Metadata about the scrape
    pub metadata: PageMetadata,
}

/// Represents a link extracted from a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Link URL (href)
    pub url: String,
    /// Link text content
    pub text: Option<String>,
    /// Link title attribute
    pub title: Option<String>,
}

/// Metadata about a scraped page
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageMetadata {
    /// Timestamp when the page was scraped (Unix epoch milliseconds)
    pub scraped_at: i64,
    /// Response status code
    pub status_code: Option<u16>,
    /// Content type header
    pub content_type: Option<String>,
    /// Page load time in milliseconds
    pub load_time_ms: Option<u64>,
}

impl ScrapedPage {
    /// Create a new scraped page with the given URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: None,
            html: None,
            text: None,
            screenshot: None,
            links: Vec::new(),
            metadata: PageMetadata::default(),
        }
    }

    /// Set the page title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the HTML content
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Set the text content
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the screenshot
    pub fn with_screenshot(mut self, screenshot: impl Into<String>) -> Self {
        self.screenshot = Some(screenshot.into());
        self
    }
}

impl Link {
    /// Create a new link with the given URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            text: None,
            title: None,
        }
    }

    /// Set the link text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_scraped_page() {
        let page = ScrapedPage::new("https://example.com")
            .with_title("Example Domain")
            .with_html("<html></html>");
        
        assert_eq!(page.url, "https://example.com");
        assert_eq!(page.title, Some("Example Domain".to_string()));
        assert!(page.html.is_some());
    }

    #[test]
    fn test_create_link() {
        let link = Link::new("https://example.com/about")
            .with_text("About Us");
        
        assert_eq!(link.url, "https://example.com/about");
        assert_eq!(link.text, Some("About Us".to_string()));
    }
}
