//! Batch processor for parallel page extraction using Rayon
//!
//! This module provides parallel processing capabilities for extracting
//! content from multiple pages simultaneously using Rayon's parallel iterators.

use rayon::prelude::*;
use synmem_core::{ExtractedContent, Page};

use crate::chromium::dom_extractor::{DomExtractor, DomExtractorError};

/// Result of a batch extraction operation
#[derive(Debug)]
pub struct BatchResult {
    /// Successfully extracted content
    pub successes: Vec<ExtractedContent>,
    /// Errors that occurred during extraction
    pub errors: Vec<(usize, DomExtractorError)>,
}

impl BatchResult {
    /// Creates a new empty BatchResult
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Returns the number of successful extractions
    pub fn success_count(&self) -> usize {
        self.successes.len()
    }

    /// Returns the number of failed extractions
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Returns true if all extractions were successful
    pub fn all_succeeded(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Default for BatchResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Batch processor for parallel DOM extraction
#[derive(Debug, Clone)]
pub struct BatchProcessor {
    extractor: DomExtractor,
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchProcessor {
    /// Creates a new BatchProcessor with default settings
    pub fn new() -> Self {
        Self {
            extractor: DomExtractor::new(),
        }
    }

    /// Creates a new BatchProcessor with a custom DomExtractor
    pub fn with_extractor(extractor: DomExtractor) -> Self {
        Self { extractor }
    }

    /// Extracts content from all pages in parallel using Rayon
    ///
    /// This is the primary method for high-performance parallel extraction.
    /// It uses Rayon's `par_iter` to process all pages concurrently.
    ///
    /// # Arguments
    /// * `pages` - A slice of pages to process
    ///
    /// # Returns
    /// A vector of extracted content for each page
    ///
    /// # Example
    /// ```
    /// use synmem_browser::BatchProcessor;
    /// use synmem_core::Page;
    /// use url::Url;
    ///
    /// let processor = BatchProcessor::new();
    /// let pages = vec![
    ///     Page::new(Url::parse("https://example.com").unwrap(), "<html><body>Hello</body></html>".to_string()),
    /// ];
    /// let results = processor.extract_all(&pages);
    /// ```
    pub fn extract_all(&self, pages: &[Page]) -> Vec<ExtractedContent> {
        pages
            .par_iter()
            .filter_map(|page| self.extractor.extract_content(page).ok())
            .collect()
    }

    /// Extracts content from all pages in parallel, collecting both successes and errors
    ///
    /// Unlike `extract_all`, this method returns a `BatchResult` that includes
    /// information about any errors that occurred during extraction.
    ///
    /// # Arguments
    /// * `pages` - A slice of pages to process
    ///
    /// # Returns
    /// A `BatchResult` containing both successful extractions and errors
    pub fn extract_all_with_errors(&self, pages: &[Page]) -> BatchResult {
        let results: Vec<(usize, Result<ExtractedContent, DomExtractorError>)> = pages
            .par_iter()
            .enumerate()
            .map(|(idx, page)| (idx, self.extractor.extract_content(page)))
            .collect();

        let mut batch_result = BatchResult::new();

        for (idx, result) in results {
            match result {
                Ok(content) => batch_result.successes.push(content),
                Err(err) => batch_result.errors.push((idx, err)),
            }
        }

        batch_result
    }

    /// Extracts only text content from all pages in parallel
    ///
    /// This is a more efficient method when you only need the text content
    /// and not links, images, or structured data.
    pub fn extract_text_all(&self, pages: &[Page]) -> Vec<String> {
        pages
            .par_iter()
            .map(|page| {
                let document = scraper::Html::parse_document(&page.html);
                self.extractor.extract_text(&document)
            })
            .collect()
    }

    /// Extracts only links from all pages in parallel
    pub fn extract_links_all(&self, pages: &[Page]) -> Vec<Vec<synmem_core::LinkInfo>> {
        pages
            .par_iter()
            .filter_map(|page| {
                let document = scraper::Html::parse_document(&page.html);
                self.extractor.extract_links(&document, &page.url).ok()
            })
            .collect()
    }

    /// Extracts only images from all pages in parallel
    pub fn extract_images_all(&self, pages: &[Page]) -> Vec<Vec<synmem_core::ImageInfo>> {
        pages
            .par_iter()
            .filter_map(|page| {
                let document = scraper::Html::parse_document(&page.html);
                self.extractor.extract_images(&document, &page.url).ok()
            })
            .collect()
    }

    /// Extracts only structured data from all pages in parallel
    pub fn extract_structured_data_all(
        &self,
        pages: &[Page],
    ) -> Vec<Vec<synmem_core::StructuredData>> {
        pages
            .par_iter()
            .filter_map(|page| {
                let document = scraper::Html::parse_document(&page.html);
                self.extractor.extract_structured_data(&document).ok()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    fn create_test_pages(count: usize) -> Vec<Page> {
        (0..count)
            .map(|i| {
                let html = format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <head><title>Page {}</title></head>
                    <body>
                        <article>
                            <h1>Article {}</h1>
                            <p>This is the content of page {}.</p>
                            <a href="/page{}">Link {}</a>
                            <img src="/image{}.jpg" alt="Image {}">
                        </article>
                    </body>
                    </html>"#,
                    i, i, i, i, i, i, i
                );
                Page::new(
                    Url::parse(&format!("https://example.com/page{}", i)).unwrap(),
                    html,
                )
            })
            .collect()
    }

    #[test]
    fn test_extract_all_single_page() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(1);
        let results = processor.extract_all(&pages);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, Some("Page 0".to_string()));
    }

    #[test]
    fn test_extract_all_multiple_pages() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(10);
        let results = processor.extract_all(&pages);

        assert_eq!(results.len(), 10);

        // Verify all pages were processed
        for (i, content) in results.iter().enumerate() {
            assert_eq!(content.title, Some(format!("Page {}", i)));
        }
    }

    #[test]
    fn test_extract_all_with_errors() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(5);
        let batch_result = processor.extract_all_with_errors(&pages);

        assert_eq!(batch_result.success_count(), 5);
        assert_eq!(batch_result.error_count(), 0);
        assert!(batch_result.all_succeeded());
    }

    #[test]
    fn test_extract_text_all() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(3);
        let texts = processor.extract_text_all(&pages);

        assert_eq!(texts.len(), 3);
        for (i, text) in texts.iter().enumerate() {
            assert!(text.contains(&format!("Article {}", i)));
            assert!(text.contains(&format!("content of page {}", i)));
        }
    }

    #[test]
    fn test_extract_links_all() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(3);
        let links = processor.extract_links_all(&pages);

        assert_eq!(links.len(), 3);
        for (i, page_links) in links.iter().enumerate() {
            assert!(!page_links.is_empty());
            assert!(page_links.iter().any(|l| l.text == format!("Link {}", i)));
        }
    }

    #[test]
    fn test_extract_images_all() {
        let processor = BatchProcessor::new();
        let pages = create_test_pages(3);
        let images = processor.extract_images_all(&pages);

        assert_eq!(images.len(), 3);
        for (i, page_images) in images.iter().enumerate() {
            assert!(!page_images.is_empty());
            assert!(page_images
                .iter()
                .any(|img| img.alt == format!("Image {}", i)));
        }
    }

    #[test]
    fn test_extract_structured_data_all() {
        let pages: Vec<Page> = (0..3)
            .map(|i| {
                let html = format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <head>
                        <title>Page {}</title>
                        <script type="application/ld+json">
                        {{
                            "@context": "https://schema.org",
                            "@type": "Article",
                            "headline": "Article {}"
                        }}
                        </script>
                    </head>
                    <body><p>Content</p></body>
                    </html>"#,
                    i, i
                );
                Page::new(
                    Url::parse(&format!("https://example.com/page{}", i)).unwrap(),
                    html,
                )
            })
            .collect();

        let processor = BatchProcessor::new();
        let structured_data = processor.extract_structured_data_all(&pages);

        assert_eq!(structured_data.len(), 3);
        for page_data in &structured_data {
            assert!(!page_data.is_empty());
            assert_eq!(page_data[0].data_type, "json-ld");
            assert_eq!(page_data[0].schema_type, Some("Article".to_string()));
        }
    }

    #[test]
    fn test_parallel_performance() {
        // Create a larger batch to test parallel performance
        let processor = BatchProcessor::new();
        let pages = create_test_pages(100);

        let start = std::time::Instant::now();
        let results = processor.extract_all(&pages);
        let duration = start.elapsed();

        assert_eq!(results.len(), 100);
        // With parallelization, 100 simple pages should complete quickly
        assert!(
            duration.as_millis() < 5000,
            "Parallel extraction took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::new();
        assert_eq!(result.success_count(), 0);
        assert_eq!(result.error_count(), 0);
        assert!(result.all_succeeded());

        result.successes.push(ExtractedContent::new(
            Url::parse("https://example.com").unwrap(),
        ));
        assert_eq!(result.success_count(), 1);
        assert!(result.all_succeeded());
    }
}
