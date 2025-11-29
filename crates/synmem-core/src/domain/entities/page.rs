//! Page entity representing a web page to be processed

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents a web page with its URL and HTML content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// The URL of the page
    pub url: Url,
    /// The raw HTML content of the page
    pub html: String,
    /// Optional title extracted from the page
    pub title: Option<String>,
}

impl Page {
    /// Creates a new Page instance
    pub fn new(url: Url, html: String) -> Self {
        Self {
            url,
            html,
            title: None,
        }
    }

    /// Creates a new Page instance with a title
    pub fn with_title(url: Url, html: String, title: String) -> Self {
        Self {
            url,
            html,
            title: Some(title),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let url = Url::parse("https://example.com").unwrap();
        let html = "<html><body>Hello</body></html>".to_string();
        let page = Page::new(url.clone(), html.clone());

        assert_eq!(page.url, url);
        assert_eq!(page.html, html);
        assert!(page.title.is_none());
    }

    #[test]
    fn test_page_with_title() {
        let url = Url::parse("https://example.com").unwrap();
        let html = "<html><body>Hello</body></html>".to_string();
        let title = "Example Title".to_string();
        let page = Page::with_title(url.clone(), html.clone(), title.clone());

        assert_eq!(page.url, url);
        assert_eq!(page.html, html);
        assert_eq!(page.title, Some(title));
    }
}
