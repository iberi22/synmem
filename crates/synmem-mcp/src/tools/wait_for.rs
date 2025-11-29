//! Wait for element tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Default timeout in milliseconds.
const DEFAULT_TIMEOUT_MS: u64 = 30000;

/// Input parameters for wait_for tool.
#[derive(Debug, Deserialize)]
pub struct WaitForInput {
    /// CSS selector of the element to wait for.
    pub selector: String,
    /// Timeout in milliseconds.
    #[serde(default)]
    pub timeout: Option<u64>,
}

/// Output for wait_for tool.
#[derive(Debug, Serialize)]
pub struct WaitForOutput {
    /// Whether the element was found.
    pub success: bool,
    /// Error message if wait timed out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Execute the wait_for tool.
pub async fn execute_wait_for(
    browser: Arc<dyn BrowserControlPort>,
    input: WaitForInput,
) -> WaitForOutput {
    let timeout = input.timeout.unwrap_or(DEFAULT_TIMEOUT_MS);

    match browser.wait_for(input.selector, timeout).await {
        Ok(()) => WaitForOutput {
            success: true,
            error: None,
        },
        Err(e) => WaitForOutput {
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
            Ok(())
        }

        async fn scroll(&self, _direction: ScrollDirection, _amount: Option<i32>) -> Result<(), String> {
            Ok(())
        }

        async fn screenshot(&self, _full_page: bool, _path: Option<String>) -> Result<Vec<u8>, String> {
            Ok(vec![])
        }

        async fn wait_for(&self, _selector: String, _timeout_ms: u64) -> Result<(), String> {
            if self.should_fail {
                Err("Timeout waiting for element".to_string())
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_execute_wait_for_success() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = WaitForInput {
            selector: "#element".to_string(),
            timeout: Some(5000),
        };

        let output = execute_wait_for(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_wait_for_default_timeout() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = WaitForInput {
            selector: "#element".to_string(),
            timeout: None,
        };

        let output = execute_wait_for(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_wait_for_timeout() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = WaitForInput {
            selector: "#nonexistent".to_string(),
            timeout: Some(1000),
        };

        let output = execute_wait_for(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
    }
}
