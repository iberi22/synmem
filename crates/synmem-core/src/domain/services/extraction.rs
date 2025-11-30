//! Extraction service for content scraping

use crate::domain::entities::ScrapedPage;
use crate::ports::outbound::BrowserDriverPort;
use std::sync::Arc;

/// Service for extracting content from web pages
pub struct ExtractionService<D: BrowserDriverPort> {
    driver: Arc<D>,
}

impl<D: BrowserDriverPort> ExtractionService<D> {
    /// Create a new extraction service with the given driver
    pub fn new(driver: Arc<D>) -> Self {
        Self { driver }
    }

    /// Extract page content and return a ScrapedPage
    pub async fn extract_page(&self) -> Result<ScrapedPage, D::Error> {
        let url = self.driver.current_url().await?;
        let html = self.driver.get_html().await?;
        let title = self.driver.evaluate_js("document.title").await.ok();
        
        Ok(ScrapedPage::new(url)
            .with_html(html)
            .with_title(title.unwrap_or_default()))
    }

    /// Take a screenshot of the current page
    pub async fn screenshot(&self) -> Result<Vec<u8>, D::Error> {
        self.driver.screenshot().await
    }

    /// Get the HTML content of the page
    pub async fn get_html(&self) -> Result<String, D::Error> {
        self.driver.get_html().await
    }

    /// Evaluate JavaScript and return the result
    pub async fn evaluate_js(&self, script: &str) -> Result<String, D::Error> {
        self.driver.evaluate_js(script).await
    }
}
