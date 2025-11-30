//! # Scraper Port
//!
//! Inbound port for web scraping operations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

/// Errors that can occur during scraping operations.
#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("scrape failed: {0}")]
    ScrapeFailed(String),

    #[error("extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("invalid selector: {0}")]
    InvalidSelector(String),

    #[error("page not loaded")]
    PageNotLoaded,
}

/// Result type for scraper operations.
pub type ScraperResult<T> = Result<T, ScraperError>;

/// Structured content extracted from a web page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// The URL of the scraped page.
    pub url: Url,
    /// The page title.
    pub title: Option<String>,
    /// Extracted text content.
    pub text_content: String,
    /// Extracted links.
    pub links: Vec<ExtractedLink>,
    /// Metadata from the page.
    pub metadata: PageMetadata,
}

/// A link extracted from a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLink {
    /// The link URL.
    pub href: Url,
    /// The link text.
    pub text: String,
}

/// Metadata extracted from a page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageMetadata {
    /// Meta description.
    pub description: Option<String>,
    /// Meta keywords.
    pub keywords: Vec<String>,
    /// Open Graph title.
    pub og_title: Option<String>,
    /// Open Graph description.
    pub og_description: Option<String>,
    /// Open Graph image URL.
    pub og_image: Option<Url>,
}

/// Options for scraping operations.
#[derive(Debug, Clone, Default)]
pub struct ScrapeOptions {
    /// Include links in the scraped content.
    pub include_links: bool,
    /// Include metadata in the scraped content.
    pub include_metadata: bool,
    /// CSS selector to scope the scrape.
    pub selector: Option<String>,
}

/// A simplified DOM representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedDom {
    /// Root element tag name.
    pub tag: String,
    /// Element attributes.
    pub attributes: Vec<(String, String)>,
    /// Child elements.
    pub children: Vec<SimplifiedDomNode>,
}

/// A node in the simplified DOM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimplifiedDomNode {
    /// An element node.
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<SimplifiedDomNode>,
    },
    /// A text node.
    Text(String),
}

/// Inbound port for web scraping operations.
///
/// This port defines the interface for scraping pages, extracting text,
/// and getting DOM structure.
#[async_trait]
pub trait ScraperPort: Send + Sync {
    /// Scrape the current page and return structured content.
    async fn scrape_page(&self, options: ScrapeOptions) -> ScraperResult<ScrapedPage>;

    /// Extract text content from the current page or a specific selector.
    async fn extract_text(&self, selector: Option<&str>) -> ScraperResult<String>;

    /// Get a simplified DOM representation of the current page.
    async fn get_dom(&self, selector: Option<&str>) -> ScraperResult<SimplifiedDom>;
}
