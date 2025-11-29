//! Scraper inbound port

use crate::domain::entities::ScrapedPage;
use async_trait::async_trait;
use std::error::Error;

/// Port for scraping web pages
#[async_trait]
pub trait ScraperPort: Send + Sync {
    /// Error type for this port
    type Error: Error + Send + Sync + 'static;

    /// Scrape the current page
    async fn scrape_page(&self) -> Result<ScrapedPage, Self::Error>;

    /// Get HTML content
    async fn get_html(&self) -> Result<String, Self::Error>;

    /// Extract text content
    async fn extract_text(&self) -> Result<String, Self::Error>;

    /// Get all links from the page
    async fn get_links(&self) -> Result<Vec<String>, Self::Error>;
}
