//! Click element tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Input parameters for click tool.
#[derive(Debug, Deserialize)]
pub struct ClickInput {
    /// CSS selector of the element to click.
    #[serde(default)]
    pub selector: Option<String>,
    /// Text content of the element to click.
    #[serde(default)]
    pub text: Option<String>,
}

/// Output for click tool.
#[derive(Debug, Serialize)]
pub struct ClickOutput {
    /// Whether click was successful.
    pub success: bool,
    /// Error message if click failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Execute the click tool.
pub async fn execute_click(
    browser: Arc<dyn BrowserControlPort>,
    input: ClickInput,
) -> ClickOutput {
    // Validate that at least one of selector or text is provided
    if input.selector.is_none() && input.text.is_none() {
        return ClickOutput {
            success: false,
            error: Some("Either 'selector' or 'text' must be provided".to_string()),
        };
    }

    match browser.click(input.selector, input.text).await {
        Ok(()) => ClickOutput {
            success: true,
            error: None,
        },
        Err(e) => ClickOutput {
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
            if self.should_fail {
                Err("Element not found".to_string())
            } else {
                Ok(())
            }
        }

        async fn type_text(&self, _selector: String, _text: String) -> Result<(), String> {
            Ok(())
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
    async fn test_execute_click_by_selector() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ClickInput {
            selector: Some("#button".to_string()),
            text: None,
        };

        let output = execute_click(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_click_by_text() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ClickInput {
            selector: None,
            text: Some("Click me".to_string()),
        };

        let output = execute_click(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_click_no_target() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ClickInput {
            selector: None,
            text: None,
        };

        let output = execute_click(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
        assert!(output.error.unwrap().contains("Either 'selector' or 'text'"));
    }

    #[tokio::test]
    async fn test_execute_click_failure() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = ClickInput {
            selector: Some("#nonexistent".to_string()),
            text: None,
        };

        let output = execute_click(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
    }
}
