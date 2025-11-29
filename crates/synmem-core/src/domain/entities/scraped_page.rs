//! Scraped page entity

use serde::{Deserialize, Serialize};

/// Represents a scraped web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// URL of the page
    pub url: String,
    /// Page title
    pub title: String,
    /// Main text content
    pub content: String,
    /// HTML content (optional)
    pub html: Option<String>,
    /// Extracted links
    pub links: Vec<Link>,
    /// Metadata
    pub metadata: PageMetadata,
}

/// A link extracted from a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Link URL
    pub href: String,
    /// Link text
    pub text: String,
}

/// Page metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageMetadata {
    /// Meta description
    pub description: Option<String>,
    /// Meta keywords
    pub keywords: Vec<String>,
    /// OpenGraph title
    pub og_title: Option<String>,
    /// OpenGraph description
    pub og_description: Option<String>,
    /// OpenGraph image
    pub og_image: Option<String>,
}

impl ScrapedPage {
    /// Creates a new scraped page
    pub fn new(url: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            content: content.into(),
            html: None,
            links: Vec::new(),
            metadata: PageMetadata::default(),
        }
    }
}
