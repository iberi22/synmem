//! Macro entity for storing recorded action sequences
//!
//! A Macro represents a named, reusable sequence of browser actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::browser_action::BrowserAction;

/// A macro containing a sequence of browser actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Sequence of browser actions
    pub actions: Vec<BrowserAction>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Macro {
    /// Create a new macro with the given name and actions
    pub fn new(name: impl Into<String>, actions: Vec<BrowserAction>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            actions,
            created_at: Utc::now(),
        }
    }

    /// Create a macro with a specific ID (useful for testing or restoring from storage)
    pub fn with_id(
        id: impl Into<String>,
        name: impl Into<String>,
        actions: Vec<BrowserAction>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            actions,
            created_at,
        }
    }

    /// Returns the number of actions in this macro
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Returns true if this macro has no actions
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Optimize the action sequence by removing redundant actions
    ///
    /// This removes:
    /// - Consecutive duplicate navigate actions (keeps the last one)
    /// - Empty type actions
    /// - Consecutive scroll actions in the same direction (combines them)
    pub fn optimize(&mut self) {
        if self.actions.is_empty() {
            return;
        }

        let mut optimized = Vec::with_capacity(self.actions.len());

        for action in self.actions.drain(..) {
            match (&action, optimized.last_mut()) {
                // Remove consecutive navigations, keep only the last
                (
                    BrowserAction::Navigate { .. },
                    Some(BrowserAction::Navigate { url: last_url }),
                ) => {
                    if let BrowserAction::Navigate { url } = &action {
                        *last_url = url.clone();
                    }
                }
                // Skip empty type actions
                (BrowserAction::Type { text, .. }, _) if text.is_empty() => {}
                // Combine consecutive scrolls in the same direction
                (
                    BrowserAction::Scroll { direction, amount },
                    Some(BrowserAction::Scroll {
                        direction: last_dir,
                        amount: last_amount,
                    }),
                ) if direction == last_dir => {
                    *last_amount = last_amount.saturating_add(*amount);
                }
                // Keep all other actions
                _ => {
                    optimized.push(action);
                }
            }
        }

        self.actions = optimized;
    }
}

/// Information about a macro for listing purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroInfo {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Number of actions in the macro
    pub action_count: usize,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl From<&Macro> for MacroInfo {
    fn from(m: &Macro) -> Self {
        Self {
            id: m.id.clone(),
            name: m.name.clone(),
            action_count: m.action_count(),
            created_at: m.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::browser_action::ScrollDirection;

    #[test]
    fn test_macro_creation() {
        let actions = vec![
            BrowserAction::navigate("https://example.com"),
            BrowserAction::click("#btn"),
        ];
        let m = Macro::new("test_macro", actions);

        assert!(!m.id.is_empty());
        assert_eq!(m.name, "test_macro");
        assert_eq!(m.action_count(), 2);
        assert!(!m.is_empty());
    }

    #[test]
    fn test_empty_macro() {
        let m = Macro::new("empty", vec![]);
        assert!(m.is_empty());
        assert_eq!(m.action_count(), 0);
    }

    #[test]
    fn test_optimize_consecutive_navigations() {
        let mut m = Macro::new(
            "test",
            vec![
                BrowserAction::navigate("https://first.com"),
                BrowserAction::navigate("https://second.com"),
                BrowserAction::navigate("https://third.com"),
            ],
        );

        m.optimize();

        assert_eq!(m.action_count(), 1);
        assert!(
            matches!(&m.actions[0], BrowserAction::Navigate { url } if url == "https://third.com")
        );
    }

    #[test]
    fn test_optimize_empty_type_actions() {
        let mut m = Macro::new(
            "test",
            vec![
                BrowserAction::type_text("#input", "hello"),
                BrowserAction::type_text("#input", ""),
                BrowserAction::click("#btn"),
            ],
        );

        m.optimize();

        assert_eq!(m.action_count(), 2);
    }

    #[test]
    fn test_optimize_consecutive_scrolls() {
        let mut m = Macro::new(
            "test",
            vec![
                BrowserAction::scroll(ScrollDirection::Down, 100),
                BrowserAction::scroll(ScrollDirection::Down, 50),
                BrowserAction::scroll(ScrollDirection::Up, 30),
            ],
        );

        m.optimize();

        assert_eq!(m.action_count(), 2);
        assert!(
            matches!(&m.actions[0], BrowserAction::Scroll { direction, amount } if *direction == ScrollDirection::Down && *amount == 150)
        );
    }

    #[test]
    fn test_macro_serialization() {
        let m = Macro::new(
            "test",
            vec![BrowserAction::navigate("https://example.com")],
        );

        let json = serde_json::to_string(&m).unwrap();
        let deserialized: Macro = serde_json::from_str(&json).unwrap();

        assert_eq!(m.id, deserialized.id);
        assert_eq!(m.name, deserialized.name);
        assert_eq!(m.action_count(), deserialized.action_count());
    }

    #[test]
    fn test_macro_info() {
        let m = Macro::new(
            "test",
            vec![
                BrowserAction::navigate("https://example.com"),
                BrowserAction::click("#btn"),
            ],
        );

        let info = MacroInfo::from(&m);
        assert_eq!(info.id, m.id);
        assert_eq!(info.name, m.name);
        assert_eq!(info.action_count, 2);
    }
}
