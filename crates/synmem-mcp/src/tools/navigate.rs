//! Navigate to URL tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::domain::entities::WaitCondition;
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Input parameters for navigate_to tool.
#[derive(Debug, Deserialize)]
pub struct NavigateToInput {
    /// URL to navigate to.
    pub url: String,
    /// Wait condition before returning.
    #[serde(default)]
    pub wait_for: Option<String>,
}

/// Output for navigate_to tool.
#[derive(Debug, Serialize)]
pub struct NavigateToOutput {
    /// Whether navigation was successful.
    pub success: bool,
    /// URL that was navigated to.
    pub url: String,
    /// Error message if navigation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parse wait_for string to WaitCondition enum.
pub fn parse_wait_condition(wait_for: Option<&str>) -> WaitCondition {
    match wait_for {
        Some("domcontentloaded") => WaitCondition::DomContentLoaded,
        Some("networkidle") => WaitCondition::NetworkIdle,
        _ => WaitCondition::Load,
    }
}

/// Execute the navigate_to tool.
pub async fn execute_navigate_to(
    browser: Arc<dyn BrowserControlPort>,
    input: NavigateToInput,
) -> NavigateToOutput {
    let wait_condition = parse_wait_condition(input.wait_for.as_deref());

    match browser.navigate_to(input.url.clone(), wait_condition).await {
        Ok(()) => NavigateToOutput {
            success: true,
            url: input.url,
            error: None,
        },
        Err(e) => NavigateToOutput {
            success: false,
            url: input.url,
            error: Some(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use synmem_core::domain::entities::ScrollDirection;

    struct MockBrowser {
        should_fail: bool,
    }

    #[async_trait]
    impl BrowserControlPort for MockBrowser {
        async fn navigate_to(&self, _url: String, _wait_for: WaitCondition) -> Result<(), String> {
            if self.should_fail {
                Err("Navigation failed".to_string())
            } else {
                Ok(())
            }
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
            Ok(())
        }
    }

    #[test]
    fn test_parse_wait_condition() {
        assert_eq!(parse_wait_condition(None), WaitCondition::Load);
        assert_eq!(parse_wait_condition(Some("load")), WaitCondition::Load);
        assert_eq!(parse_wait_condition(Some("domcontentloaded")), WaitCondition::DomContentLoaded);
        assert_eq!(parse_wait_condition(Some("networkidle")), WaitCondition::NetworkIdle);
        assert_eq!(parse_wait_condition(Some("invalid")), WaitCondition::Load);
    }

    #[tokio::test]
    async fn test_execute_navigate_to_success() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = NavigateToInput {
            url: "https://example.com".to_string(),
            wait_for: Some("load".to_string()),
        };

        let output = execute_navigate_to(browser, input).await;

        assert!(output.success);
        assert_eq!(output.url, "https://example.com");
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_navigate_to_failure() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = NavigateToInput {
            url: "https://example.com".to_string(),
            wait_for: None,
        };

        let output = execute_navigate_to(browser, input).await;

        assert!(!output.success);
        assert_eq!(output.url, "https://example.com");
        assert!(output.error.is_some());
    }
}
