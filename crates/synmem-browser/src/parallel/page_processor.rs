//! Page processor for parallel DOM operations

use rayon::prelude::*;

/// Processes page content in parallel
pub struct PageProcessor;

impl PageProcessor {
    /// Process multiple HTML documents in parallel
    pub fn process_documents<F, T>(documents: &[String], processor: F) -> Vec<T>
    where
        F: Fn(&str) -> T + Send + Sync,
        T: Send,
    {
        documents.par_iter().map(|doc| processor(doc)).collect()
    }

    /// Extract all text from multiple documents
    pub fn extract_all_text(documents: &[String]) -> Vec<String> {
        Self::process_documents(documents, |html| {
            // Simple text extraction (in production, use proper HTML parser)
            html.split(|c| c == '<' || c == '>')
                .filter(|s| !s.trim().is_empty() && !s.starts_with('/'))
                .filter(|s| !s.chars().all(|c| c.is_whitespace() || c == '\n'))
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    /// Count words in multiple documents
    pub fn count_words(documents: &[String]) -> Vec<usize> {
        Self::process_documents(documents, |html| {
            html.split_whitespace().count()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_documents() {
        let docs = vec![
            "<p>Hello</p>".to_string(),
            "<p>World</p>".to_string(),
        ];

        let results = PageProcessor::process_documents(&docs, |_| 1);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_count_words() {
        let docs = vec![
            "one two three".to_string(),
            "four five".to_string(),
        ];

        let counts = PageProcessor::count_words(&docs);
        assert_eq!(counts[0], 3);
        assert_eq!(counts[1], 2);
    }
}
