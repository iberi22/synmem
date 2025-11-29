//! DOM extraction functionality for parsing HTML content
//!
//! This module provides the `DomExtractor` struct for extracting structured content
//! from HTML pages, including text, links, images, and structured data.

use scraper::{Html, Selector};
use synmem_core::{ExtractedContent, ImageInfo, LinkInfo, Page, StructuredData};
use thiserror::Error;
use url::Url;

/// Errors that can occur during DOM extraction
#[derive(Debug, Error)]
pub enum DomExtractorError {
    #[error("Failed to parse selector: {0}")]
    SelectorParseError(String),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Failed to parse JSON-LD: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

/// Result type for DOM extraction operations
pub type Result<T> = std::result::Result<T, DomExtractorError>;

/// DOM extractor for parsing HTML content and extracting structured data
#[derive(Debug, Clone)]
pub struct DomExtractor {
    /// Selectors for main content detection
    main_content_selectors: Vec<String>,
}

impl Default for DomExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl DomExtractor {
    /// Creates a new DomExtractor with default settings
    pub fn new() -> Self {
        Self {
            main_content_selectors: vec![
                "article".to_string(),
                "main".to_string(),
                "[role='main']".to_string(),
                ".article-content".to_string(),
                ".post-content".to_string(),
                ".entry-content".to_string(),
                "#content".to_string(),
            ],
        }
    }

    /// Extracts all content from a single page
    pub fn extract_content(&self, page: &Page) -> Result<ExtractedContent> {
        let document = Html::parse_document(&page.html);
        let base_url = &page.url;

        let mut content = ExtractedContent::new(page.url.clone());

        // Extract title
        content.title = self.extract_title(&document).or(page.title.clone());

        // Extract text content
        content.text_content = self.extract_text(&document);

        // Extract links
        content.links = self.extract_links(&document, base_url)?;

        // Extract images
        content.images = self.extract_images(&document, base_url)?;

        // Extract structured data (JSON-LD)
        content.structured_data = self.extract_structured_data(&document)?;

        // Extract main content
        content.main_content = self.extract_main_content(&document);

        // Extract meta description
        content.meta_description = self.extract_meta_description(&document);

        // Extract meta keywords
        content.meta_keywords = self.extract_meta_keywords(&document);

        Ok(content)
    }

    /// Extracts the page title
    pub fn extract_title(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        document
            .select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
    }

    /// Extracts all text content from the page body
    pub fn extract_text(&self, document: &Html) -> String {
        let body_selector = Selector::parse("body").ok();

        if let Some(selector) = body_selector {
            if let Some(body) = document.select(&selector).next() {
                // Get all text, filtering out script and style content
                let text: String = body
                    .text()
                    .filter(|t| {
                        // Check if this text node is inside a script or style tag
                        // This is a simple heuristic
                        !t.trim().is_empty()
                    })
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ");

                // Clean up the text - remove excessive whitespace
                return text.split_whitespace().collect::<Vec<_>>().join(" ");
            }
        }

        // Fallback to root text extraction
        document
            .root_element()
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Extracts all links from the page
    pub fn extract_links(&self, document: &Html, base_url: &Url) -> Result<Vec<LinkInfo>> {
        let selector = Selector::parse("a[href]")
            .map_err(|e| DomExtractorError::SelectorParseError(format!("{:?}", e)))?;

        let base_host = base_url.host_str().unwrap_or("");

        let links: Vec<LinkInfo> = document
            .select(&selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let url = resolve_url(base_url, href).ok()?;

                let text = el.text().collect::<String>().trim().to_string();
                let title = el.value().attr("title").map(|s| s.to_string());
                let is_internal = url.host_str() == Some(base_host);

                Some(LinkInfo {
                    url,
                    text,
                    title,
                    is_internal,
                })
            })
            .collect();

        Ok(links)
    }

    /// Extracts all images from the page
    pub fn extract_images(&self, document: &Html, base_url: &Url) -> Result<Vec<ImageInfo>> {
        let selector = Selector::parse("img[src]")
            .map_err(|e| DomExtractorError::SelectorParseError(format!("{:?}", e)))?;

        let images: Vec<ImageInfo> = document
            .select(&selector)
            .filter_map(|el| {
                let src = el.value().attr("src")?;
                let url = resolve_url(base_url, src).ok()?;

                let alt = el.value().attr("alt").unwrap_or("").to_string();
                let title = el.value().attr("title").map(|s| s.to_string());
                let width = el.value().attr("width").and_then(|w| w.parse().ok());
                let height = el.value().attr("height").and_then(|h| h.parse().ok());

                Some(ImageInfo {
                    src: url,
                    alt,
                    title,
                    width,
                    height,
                })
            })
            .collect();

        Ok(images)
    }

    /// Extracts structured data (JSON-LD) from the page
    pub fn extract_structured_data(&self, document: &Html) -> Result<Vec<StructuredData>> {
        let selector = Selector::parse("script[type='application/ld+json']")
            .map_err(|e| DomExtractorError::SelectorParseError(format!("{:?}", e)))?;

        let mut structured_data = Vec::new();

        for el in document.select(&selector) {
            let json_text = el.text().collect::<String>();
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json_text) {
                let schema_type = data
                    .get("@type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                structured_data.push(StructuredData {
                    data_type: "json-ld".to_string(),
                    schema_type,
                    data,
                });
            }
        }

        Ok(structured_data)
    }

    /// Extracts main content from the page using common selectors
    pub fn extract_main_content(&self, document: &Html) -> Option<String> {
        for selector_str in &self.main_content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(el) = document.select(&selector).next() {
                    let text: String = el
                        .text()
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");

                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }
        None
    }

    /// Extracts meta description from the page
    pub fn extract_meta_description(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("meta[name='description']").ok()?;
        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.to_string())
    }

    /// Extracts meta keywords from the page
    pub fn extract_meta_keywords(&self, document: &Html) -> Vec<String> {
        let selector = match Selector::parse("meta[name='keywords']") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        document
            .select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| {
                s.split(',')
                    .map(|k| k.trim().to_string())
                    .filter(|k| !k.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Resolves a potentially relative URL against a base URL
fn resolve_url(base: &Url, href: &str) -> std::result::Result<Url, url::ParseError> {
    // Try parsing as absolute URL first
    if let Ok(url) = Url::parse(href) {
        return Ok(url);
    }

    // Otherwise, resolve against base URL
    base.join(href)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_page(html: &str) -> Page {
        Page::new(Url::parse("https://example.com").unwrap(), html.to_string())
    }

    #[test]
    fn test_extract_title() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Page Title</title></head>
            <body><p>Content</p></body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(content.title, Some("Test Page Title".to_string()));
    }

    #[test]
    fn test_extract_text() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body>
                <p>Hello World</p>
                <p>This is a test</p>
            </body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert!(content.text_content.contains("Hello World"));
        assert!(content.text_content.contains("This is a test"));
    }

    #[test]
    fn test_extract_links() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body>
                <a href="/page1">Internal Link</a>
                <a href="https://external.com/page">External Link</a>
                <a href="/page2" title="Page 2 Title">Link with Title</a>
            </body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(content.links.len(), 3);

        // Check internal link
        let internal_link = content.links.iter().find(|l| l.text == "Internal Link");
        assert!(internal_link.is_some());
        assert!(internal_link.unwrap().is_internal);

        // Check external link
        let external_link = content.links.iter().find(|l| l.text == "External Link");
        assert!(external_link.is_some());
        assert!(!external_link.unwrap().is_internal);

        // Check link with title
        let titled_link = content.links.iter().find(|l| l.text == "Link with Title");
        assert!(titled_link.is_some());
        assert_eq!(titled_link.unwrap().title, Some("Page 2 Title".to_string()));
    }

    #[test]
    fn test_extract_images() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body>
                <img src="/image1.jpg" alt="Image 1">
                <img src="https://cdn.example.com/image2.png" alt="Image 2" width="800" height="600">
                <img src="/image3.gif" alt="Image 3" title="Third Image">
            </body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(content.images.len(), 3);

        // Check image with dimensions
        let img_with_dims = content.images.iter().find(|i| i.alt == "Image 2");
        assert!(img_with_dims.is_some());
        let img = img_with_dims.unwrap();
        assert_eq!(img.width, Some(800));
        assert_eq!(img.height, Some(600));

        // Check image with title
        let img_with_title = content.images.iter().find(|i| i.alt == "Image 3");
        assert!(img_with_title.is_some());
        assert_eq!(
            img_with_title.unwrap().title,
            Some("Third Image".to_string())
        );
    }

    #[test]
    fn test_extract_structured_data() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Test Article</title>
                <script type="application/ld+json">
                {
                    "@context": "https://schema.org",
                    "@type": "Article",
                    "headline": "Test Article Headline",
                    "author": "Test Author"
                }
                </script>
            </head>
            <body><p>Content</p></body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(content.structured_data.len(), 1);
        assert_eq!(content.structured_data[0].data_type, "json-ld");
        assert_eq!(
            content.structured_data[0].schema_type,
            Some("Article".to_string())
        );
    }

    #[test]
    fn test_extract_main_content() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body>
                <header>Header Content</header>
                <article>
                    <h1>Article Title</h1>
                    <p>This is the main article content.</p>
                </article>
                <footer>Footer Content</footer>
            </body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert!(content.main_content.is_some());
        let main = content.main_content.unwrap();
        assert!(main.contains("Article Title"));
        assert!(main.contains("main article content"));
        // Should not contain header/footer
        assert!(!main.contains("Header Content"));
        assert!(!main.contains("Footer Content"));
    }

    #[test]
    fn test_extract_meta_description() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Test</title>
                <meta name="description" content="This is a test description.">
            </head>
            <body><p>Content</p></body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(
            content.meta_description,
            Some("This is a test description.".to_string())
        );
    }

    #[test]
    fn test_extract_meta_keywords() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Test</title>
                <meta name="keywords" content="rust, web scraping, dom extraction">
            </head>
            <body><p>Content</p></body>
            </html>
        "#;

        let extractor = DomExtractor::new();
        let page = create_test_page(html);
        let content = extractor.extract_content(&page).unwrap();

        assert_eq!(content.meta_keywords.len(), 3);
        assert!(content.meta_keywords.contains(&"rust".to_string()));
        assert!(content.meta_keywords.contains(&"web scraping".to_string()));
        assert!(content
            .meta_keywords
            .contains(&"dom extraction".to_string()));
    }
}
