//! BrowserAction entity for macro recording
//!
//! Defines all possible browser actions that can be recorded and replayed.

use serde::{Deserialize, Serialize};

/// Direction for scroll actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a single browser action that can be recorded and replayed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BrowserAction {
    /// Navigate to a URL
    Navigate { url: String },
    /// Click on an element by selector
    Click { selector: String },
    /// Type text into an element
    Type { selector: String, text: String },
    /// Scroll in a direction
    Scroll {
        direction: ScrollDirection,
        amount: i32,
    },
    /// Wait for an element with timeout
    Wait { selector: String, timeout_ms: u64 },
}

impl BrowserAction {
    /// Create a navigate action
    pub fn navigate(url: impl Into<String>) -> Self {
        Self::Navigate { url: url.into() }
    }

    /// Create a click action
    pub fn click(selector: impl Into<String>) -> Self {
        Self::Click {
            selector: selector.into(),
        }
    }

    /// Create a type action
    pub fn type_text(selector: impl Into<String>, text: impl Into<String>) -> Self {
        Self::Type {
            selector: selector.into(),
            text: text.into(),
        }
    }

    /// Create a scroll action
    pub fn scroll(direction: ScrollDirection, amount: i32) -> Self {
        Self::Scroll { direction, amount }
    }

    /// Create a wait action
    pub fn wait(selector: impl Into<String>, timeout_ms: u64) -> Self {
        Self::Wait {
            selector: selector.into(),
            timeout_ms,
        }
    }

    /// Returns a human-readable description of this action
    pub fn description(&self) -> String {
        match self {
            Self::Navigate { url } => format!("Navigate to {}", url),
            Self::Click { selector } => format!("Click on {}", selector),
            Self::Type { selector, text } => format!("Type '{}' into {}", text, selector),
            Self::Scroll { direction, amount } => {
                format!("Scroll {:?} by {} pixels", direction, amount)
            }
            Self::Wait { selector, timeout_ms } => {
                format!("Wait for {} (timeout: {}ms)", selector, timeout_ms)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate_action() {
        let action = BrowserAction::navigate("https://example.com");
        assert!(matches!(action, BrowserAction::Navigate { url } if url == "https://example.com"));
    }

    #[test]
    fn test_click_action() {
        let action = BrowserAction::click("#submit-btn");
        assert!(matches!(action, BrowserAction::Click { selector } if selector == "#submit-btn"));
    }

    #[test]
    fn test_type_action() {
        let action = BrowserAction::type_text("#username", "john");
        assert!(
            matches!(action, BrowserAction::Type { selector, text } if selector == "#username" && text == "john")
        );
    }

    #[test]
    fn test_scroll_action() {
        let action = BrowserAction::scroll(ScrollDirection::Down, 100);
        assert!(
            matches!(action, BrowserAction::Scroll { direction, amount } if direction == ScrollDirection::Down && amount == 100)
        );
    }

    #[test]
    fn test_wait_action() {
        let action = BrowserAction::wait("#loading", 5000);
        assert!(
            matches!(action, BrowserAction::Wait { selector, timeout_ms } if selector == "#loading" && timeout_ms == 5000)
        );
    }

    #[test]
    fn test_action_serialization() {
        let action = BrowserAction::navigate("https://example.com");
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("navigate"));
        assert!(json.contains("https://example.com"));

        let deserialized: BrowserAction = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_action_description() {
        let action = BrowserAction::navigate("https://example.com");
        assert_eq!(action.description(), "Navigate to https://example.com");
    }
}
