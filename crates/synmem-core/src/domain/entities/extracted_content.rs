//! Extracted content entities representing parsed page data

use serde::{Deserialize, Serialize};
use url::Url;

/// Information about an extracted link
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkInfo {
    /// The URL the link points to
    pub url: Url,
    /// The text content of the link
    pub text: String,
    /// The title attribute of the link, if present
    pub title: Option<String>,
    /// Whether the link is internal (same domain) or external
    pub is_internal: bool,
}

/// Information about an extracted image
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageInfo {
    /// The source URL of the image
    pub src: Url,
    /// The alt text of the image
    pub alt: String,
    /// The title attribute of the image, if present
    pub title: Option<String>,
    /// Width in pixels, if specified
    pub width: Option<u32>,
    /// Height in pixels, if specified
    pub height: Option<u32>,
}

/// Structured data extracted from the page (JSON-LD, microdata, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuredData {
    /// The type of structured data (e.g., "json-ld", "microdata")
    pub data_type: String,
    /// The schema type (e.g., "Article", "Product")
    pub schema_type: Option<String>,
    /// The raw data as a JSON value
    pub data: serde_json::Value,
}

/// Content extracted from a web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// The source URL of the page
    pub url: Url,
    /// The page title
    pub title: Option<String>,
    /// The main text content of the page
    pub text_content: String,
    /// All links found on the page
    pub links: Vec<LinkInfo>,
    /// All images found on the page
    pub images: Vec<ImageInfo>,
    /// Structured data (JSON-LD, microdata)
    pub structured_data: Vec<StructuredData>,
    /// Main content HTML (from article, main, etc.)
    pub main_content: Option<String>,
    /// Meta description
    pub meta_description: Option<String>,
    /// Meta keywords
    pub meta_keywords: Vec<String>,
}

impl ExtractedContent {
    /// Creates a new empty ExtractedContent for a given URL
    pub fn new(url: Url) -> Self {
        Self {
            url,
            title: None,
            text_content: String::new(),
            links: Vec::new(),
            images: Vec::new(),
            structured_data: Vec::new(),
            main_content: None,
            meta_description: None,
            meta_keywords: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracted_content_new() {
        let url = Url::parse("https://example.com").unwrap();
        let content = ExtractedContent::new(url.clone());

        assert_eq!(content.url, url);
        assert!(content.title.is_none());
        assert!(content.text_content.is_empty());
        assert!(content.links.is_empty());
        assert!(content.images.is_empty());
        assert!(content.structured_data.is_empty());
        assert!(content.main_content.is_none());
    }

    #[test]
    fn test_link_info() {
        let link = LinkInfo {
            url: Url::parse("https://example.com/page").unwrap(),
            text: "Example Link".to_string(),
            title: Some("Link Title".to_string()),
            is_internal: true,
        };

        assert_eq!(link.text, "Example Link");
        assert!(link.is_internal);
    }

    #[test]
    fn test_image_info() {
        let image = ImageInfo {
            src: Url::parse("https://example.com/image.jpg").unwrap(),
            alt: "Example Image".to_string(),
            title: None,
            width: Some(800),
            height: Some(600),
        };

        assert_eq!(image.alt, "Example Image");
        assert_eq!(image.width, Some(800));
    }

    #[test]
    fn test_structured_data() {
        let data = StructuredData {
            data_type: "json-ld".to_string(),
            schema_type: Some("Article".to_string()),
            data: serde_json::json!({"headline": "Test Article"}),
        };

        assert_eq!(data.data_type, "json-ld");
        assert_eq!(data.schema_type, Some("Article".to_string()));
    }
}
