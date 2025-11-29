//! Scroll tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::domain::entities::ScrollDirection;
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Input parameters for scroll tool.
#[derive(Debug, Deserialize)]
pub struct ScrollInput {
    /// Direction to scroll.
    #[serde(default)]
    pub direction: Option<String>,
    /// Amount to scroll in pixels.
    #[serde(default)]
    pub amount: Option<i32>,
}

/// Output for scroll tool.
#[derive(Debug, Serialize)]
pub struct ScrollOutput {
    /// Whether scroll was successful.
    pub success: bool,
    /// Error message if scroll failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parse direction string to ScrollDirection enum.
pub fn parse_scroll_direction(direction: Option<&str>) -> ScrollDirection {
    match direction {
        Some("up") => ScrollDirection::Up,
        Some("left") => ScrollDirection::Left,
        Some("right") => ScrollDirection::Right,
        _ => ScrollDirection::Down,
    }
}

/// Execute the scroll tool.
pub async fn execute_scroll(
    browser: Arc<dyn BrowserControlPort>,
    input: ScrollInput,
) -> ScrollOutput {
    let direction = parse_scroll_direction(input.direction.as_deref());

    match browser.scroll(direction, input.amount).await {
        Ok(()) => ScrollOutput {
            success: true,
            error: None,
        },
        Err(e) => ScrollOutput {
            success: false,
            error: Some(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use synmem_core::domain::entities::WaitCondition;

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
            if self.should_fail {
                Err("Scroll failed".to_string())
            } else {
                Ok(())
            }
        }

        async fn screenshot(&self, _full_page: bool, _path: Option<String>) -> Result<Vec<u8>, String> {
            Ok(vec![])
        }

        async fn wait_for(&self, _selector: String, _timeout_ms: u64) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_parse_scroll_direction() {
        assert_eq!(parse_scroll_direction(None), ScrollDirection::Down);
        assert_eq!(parse_scroll_direction(Some("down")), ScrollDirection::Down);
        assert_eq!(parse_scroll_direction(Some("up")), ScrollDirection::Up);
        assert_eq!(parse_scroll_direction(Some("left")), ScrollDirection::Left);
        assert_eq!(parse_scroll_direction(Some("right")), ScrollDirection::Right);
        assert_eq!(parse_scroll_direction(Some("invalid")), ScrollDirection::Down);
    }

    #[tokio::test]
    async fn test_execute_scroll_success() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ScrollInput {
            direction: Some("down".to_string()),
            amount: Some(500),
        };

        let output = execute_scroll(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_scroll_default_direction() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ScrollInput {
            direction: None,
            amount: None,
        };

        let output = execute_scroll(browser, input).await;

        assert!(output.success);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_scroll_failure() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = ScrollInput {
            direction: Some("up".to_string()),
            amount: Some(100),
        };

        let output = execute_scroll(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
    }
}
