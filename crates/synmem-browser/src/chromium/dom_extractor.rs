//! DOM extractor for parallel content extraction using Rayon

use rayon::prelude::*;

/// Result of DOM extraction
#[derive(Debug, Clone)]
pub struct ExtractedContent {
    /// Page title
    pub title: Option<String>,
    /// Extracted text content
    pub text: String,
    /// Extracted links
    pub links: Vec<ExtractedLink>,
}

/// Extracted link from the DOM
#[derive(Debug, Clone)]
pub struct ExtractedLink {
    /// Link URL
    pub href: String,
    /// Link text
    pub text: String,
}

/// Extracts content from HTML using Rayon for parallel processing
pub struct DomExtractor;

impl DomExtractor {
    /// Extract text content from HTML (CPU-bound, parallelized)
    ///
    /// This is a simplified implementation. In production, you'd use
    /// a proper HTML parser like `scraper` or `select.rs`.
    pub fn extract_text(html: &str) -> String {
        // Split HTML into chunks for parallel processing
        let chunks: Vec<&str> = html.split('>').collect();

        // Process chunks in parallel
        let texts: Vec<String> = chunks
            .par_iter()
            .filter_map(|chunk| {
                // Extract text before the next tag
                if let Some(text_end) = chunk.find('<') {
                    let text = &chunk[..text_end];
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
                None
            })
            .collect();

        texts.join(" ")
    }

    /// Extract links from HTML (CPU-bound, parallelized)
    pub fn extract_links(html: &str, base_url: &str) -> Vec<ExtractedLink> {
        // Find all href attributes
        let href_pattern = "href=\"";

        // Split by href to find links
        let parts: Vec<&str> = html.split(href_pattern).skip(1).collect();

        parts
            .par_iter()
            .filter_map(|part| {
                // Extract the URL
                if let Some(end) = part.find('"') {
                    let href = &part[..end];
                    let href = if href.starts_with('/') {
                        format!("{}{}", base_url.trim_end_matches('/'), href)
                    } else {
                        href.to_string()
                    };

                    // Try to extract link text (simplified)
                    let text = part
                        .find('>')
                        .and_then(|start| {
                            let after_tag = &part[start + 1..];
                            after_tag.find('<').map(|end| after_tag[..end].trim().to_string())
                        })
                        .unwrap_or_default();

                    return Some(ExtractedLink { href, text });
                }
                None
            })
            .collect()
    }

    /// Batch extract content from multiple HTML documents
    pub fn batch_extract(documents: &[(&str, &str)]) -> Vec<ExtractedContent> {
        documents
            .par_iter()
            .map(|(html, base_url)| {
                let text = Self::extract_text(html);
                let links = Self::extract_links(html, base_url);

                // Extract title (simplified)
                let title = html
                    .find("<title>")
                    .and_then(|start| {
                        let after_title = &html[start + 7..];
                        after_title
                            .find("</title>")
                            .map(|end| after_title[..end].to_string())
                    });

                ExtractedContent { title, text, links }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text() {
        let html = "<html><body><p>Hello World</p><p>Test</p></body></html>";
        let text = DomExtractor::extract_text(html);
        assert!(text.contains("Hello World"));
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_extract_links() {
        let html = r#"<a href="/about">About</a><a href="https://example.com">External</a>"#;
        let links = DomExtractor::extract_links(html, "https://test.com");

        assert_eq!(links.len(), 2);
        assert!(links.iter().any(|l| l.href == "https://test.com/about"));
        assert!(links.iter().any(|l| l.href == "https://example.com"));
    }

    #[test]
    fn test_batch_extract() {
        let docs = vec![
            ("<html><title>Page 1</title><body>Content 1</body></html>", "https://test1.com"),
            ("<html><title>Page 2</title><body>Content 2</body></html>", "https://test2.com"),
        ];

        let results = DomExtractor::batch_extract(&docs);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, Some("Page 1".to_string()));
        assert_eq!(results[1].title, Some("Page 2".to_string()));
    }
}
