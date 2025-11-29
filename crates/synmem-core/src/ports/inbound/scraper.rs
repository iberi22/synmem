//! Scraper inbound port
//!
//! This port defines the interface for scraping web content.

use crate::domain::entities::{Link, ScrapedPage};
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during scraping operations
#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Scrape failed: {0}")]
    ScrapeFailed(String),
    #[error("Page not loaded")]
    PageNotLoaded,
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for scraper operations
pub type ScraperResult<T> = Result<T, ScraperError>;

/// Scrape options
#[derive(Debug, Clone, Default)]
pub struct ScrapeOptions {
    /// Include HTML content
    pub include_html: bool,
    /// Extract links
    pub extract_links: bool,
    /// Extract metadata
    pub extract_metadata: bool,
    /// Specific selector to scrape
    pub selector: Option<String>,
}

/// Chat scrape options for AI chat interfaces
#[derive(Debug, Clone, Default)]
pub struct ChatScrapeOptions {
    /// Chat platform type
    pub platform: ChatPlatform,
    /// Include timestamps
    pub include_timestamps: bool,
    /// Max messages to extract
    pub max_messages: Option<usize>,
}

/// Supported chat platforms
#[derive(Debug, Clone, Default)]
pub enum ChatPlatform {
    #[default]
    Universal,
    ChatGPT,
    Claude,
    Gemini,
}

/// Chat message from AI conversation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    /// Message role (user/assistant)
    pub role: String,
    /// Message content
    pub content: String,
    /// Optional timestamp
    pub timestamp: Option<String>,
}

/// Scraper inbound port trait
#[async_trait]
pub trait ScraperPort: Send + Sync {
    /// Scrapes the current page
    async fn scrape_page(
        &self,
        session_id: &str,
        options: ScrapeOptions,
    ) -> ScraperResult<ScrapedPage>;

    /// Scrapes chat messages from an AI chat interface
    async fn scrape_chat(
        &self,
        session_id: &str,
        options: ChatScrapeOptions,
    ) -> ScraperResult<Vec<ChatMessage>>;

    /// Extracts all links from the current page
    async fn extract_links(&self, session_id: &str) -> ScraperResult<Vec<Link>>;

    /// Extracts clean text from the current page
    async fn extract_text(&self, session_id: &str) -> ScraperResult<String>;

    /// Gets a simplified DOM representation
    async fn get_dom(&self, session_id: &str, max_depth: Option<usize>) -> ScraperResult<String>;
}
