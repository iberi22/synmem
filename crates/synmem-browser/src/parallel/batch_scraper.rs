//! Batch scraper for processing multiple pages

use rayon::prelude::*;
use synmem_core::domain::entities::ScrapedPage;

/// Configuration for batch scraping
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum concurrent pages
    pub max_concurrent: usize,
    /// Timeout per page in milliseconds
    pub timeout_ms: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 4,
            timeout_ms: 30000,
        }
    }
}

/// Results from batch scraping
#[derive(Debug)]
pub struct BatchResult {
    /// Successfully scraped pages
    pub pages: Vec<ScrapedPage>,
    /// Failed URLs with error messages
    pub errors: Vec<(String, String)>,
}

/// Batch scraper for processing multiple URLs
pub struct BatchScraper {
    config: BatchConfig,
}

impl BatchScraper {
    /// Create a new batch scraper with the given configuration
    pub fn new(config: BatchConfig) -> Self {
        Self { config }
    }

    /// Process scraped HTML in parallel (CPU-bound)
    ///
    /// This takes already-fetched HTML and processes it in parallel.
    /// The actual network requests should be done asynchronously before calling this.
    pub fn process_html_batch(&self, pages: Vec<(String, String)>) -> Vec<ScrapedPage> {
        pages
            .par_iter()
            .map(|(url, html)| {
                // Process each page in parallel
                ScrapedPage::new(url.clone())
                    .with_html(html.clone())
                    .with_text(Self::extract_text(html))
            })
            .collect()
    }

    /// Extract text from HTML (simplified)
    fn extract_text(html: &str) -> String {
        html.split(|c| c == '<' || c == '>')
            .filter(|s| !s.trim().is_empty() && !s.starts_with('/'))
            .filter(|s| {
                !s.chars().next().map(|c| c == '!' || c.is_alphabetic()).unwrap_or(false)
                    || !s.contains('=')
            })
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get the configuration
    pub fn config(&self) -> &BatchConfig {
        &self.config
    }
}

impl Default for BatchScraper {
    fn default() -> Self {
        Self::new(BatchConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_scraper_config() {
        let config = BatchConfig {
            max_concurrent: 8,
            timeout_ms: 60000,
        };
        let scraper = BatchScraper::new(config);
        
        assert_eq!(scraper.config().max_concurrent, 8);
        assert_eq!(scraper.config().timeout_ms, 60000);
    }

    #[test]
    fn test_process_html_batch() {
        let scraper = BatchScraper::default();
        let pages = vec![
            ("https://example1.com".to_string(), "<p>Page 1</p>".to_string()),
            ("https://example2.com".to_string(), "<p>Page 2</p>".to_string()),
        ];

        let results = scraper.process_html_batch(pages);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].url, "https://example1.com");
        assert_eq!(results[1].url, "https://example2.com");
    }
}
