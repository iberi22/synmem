//! Type text tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Input parameters for type_text tool.
#[derive(Debug, Deserialize)]
pub struct TypeTextInput {
    /// CSS selector of the input element.
    pub selector: String,
    /// Text to type.
    pub text: String,
}

/// Output for type_text tool.
#[derive(Debug, Serialize)]
pub struct TypeTextOutput {
    /// Whether typing was successful.
    pub success: bool,
    /// Error message if typing failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Execute the type_text tool.
pub async fn execute_type_text(
    browser: Arc<dyn BrowserControlPort>,
    input: TypeTextInput,
) -> TypeTextOutput {
    match browser.type_text(input.selector, input.text).await {
        Ok(()) => TypeTextOutput {
            success: true,
            error: None,
        },
        Err(e) => TypeTextOutput {
            success: false,
            error: Some(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use synmem_core::domain::entities::{ScrollDirection, WaitCondition};

    struct MockBrowser {
        should_fail: bool,
    }

    #[async_trait]
    impl BrowserControlPort for MockBrowser {
        async fn navigate_to(&self, _url: String, _wait_for: WaitCondition) -> Result<(), String> {
            Ok(())
        }

        async fn click(&self, _selector: Option<String>, _text: Option<String>) -> Result<(), String> {
            Ok(())
        }

        async fn type_text(&self, _selector: String, _text: String) -> Result<(), String> {
            if self.should_fail {
                Err("Element not found".to_string())
            } else {
                Ok(())
            }
        }

        async fn scroll(&self, _direction: ScrollDirection, _amount: Option<i32>) -> Result<(), String> {
            Ok(())
        }

        async fn screenshot(&self, _full_page: bool, _path: Option<String>) -> Result<Vec<u8>, String> {
            Ok(vec![])
        }

        async fn wait_for(&self, _selector: String, _timeout_ms: u64) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_execute_type_text_success() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = TypeTextInput {
            selector: "#input".to_string(),
            text: "Hello, World!".to_string(),
        };

        let output = execute_type_text(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_type_text_failure() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = TypeTextInput {
            selector: "#nonexistent".to_string(),
            text: "Hello".to_string(),
        };

        let output = execute_type_text(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
    }
}
