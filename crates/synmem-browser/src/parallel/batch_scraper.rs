//! Batch scraper for processing multiple pages

use rayon::prelude::*;
use synmem_core::domain::entities::ScrapedPage;

/// Configuration for batch scraping
///
/// Note: Rayon manages its own thread pool internally. The `timeout_ms` is for
/// reference by calling code when performing async fetches before batch processing.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Timeout per page in milliseconds (for async fetch operations)
    pub timeout_ms: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
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
    /// This takes already-fetched HTML and processes it in parallel using Rayon.
    /// The actual network requests should be done asynchronously before calling this.
    /// Rayon automatically manages the thread pool for optimal parallelism.
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

    /// Extract visible text content from HTML
    ///
    /// This is a simple text extraction that:
    /// 1. Splits on HTML tag boundaries
    /// 2. Filters out tag content (anything that looks like a tag)
    /// 3. Collects the remaining text
    ///
    /// For production use, consider using a proper HTML parser like `scraper`.
    fn extract_text(html: &str) -> String {
        // Split by < and > to separate tags from content
        let parts: Vec<&str> = html.split(|c| c == '<' || c == '>').collect();
        
        // Known HTML tag names to filter out
        const HTML_TAGS: &[&str] = &[
            "html", "head", "body", "div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6",
            "a", "img", "ul", "ol", "li", "table", "tr", "td", "th", "thead", "tbody",
            "form", "input", "button", "select", "option", "textarea", "label",
            "script", "style", "link", "meta", "title", "header", "footer", "nav",
            "section", "article", "aside", "main", "figure", "figcaption", "br", "hr",
            "strong", "em", "b", "i", "u", "small", "code", "pre", "blockquote",
            "iframe", "video", "audio", "source", "canvas", "svg", "path",
        ];
        
        parts
            .iter()
            .filter(|part| {
                let trimmed = part.trim();
                // Skip empty parts
                if trimmed.is_empty() {
                    return false;
                }
                // Skip parts that look like HTML tags (start with / or contain =)
                if trimmed.starts_with('/') || trimmed.starts_with('!') || trimmed.contains('=') {
                    return false;
                }
                // Skip known HTML tag names
                let lower = trimmed.to_lowercase();
                if HTML_TAGS.contains(&lower.as_str()) {
                    return false;
                }
                true
            })
            .map(|s| s.trim())
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
            timeout_ms: 60000,
        };
        let scraper = BatchScraper::new(config);
        
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

    #[test]
    fn test_extract_text() {
        let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
        let text = BatchScraper::extract_text(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("World"));
    }
}
