//! Browser Task Entity
//!
//! Represents a browser automation task with its URL, action, status, and result.

use serde::{Deserialize, Serialize};

/// The action to perform in the browser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserAction {
    /// Navigate to a URL.
    Navigate,
    /// Click on an element.
    Click { selector: String },
    /// Type text into an input field.
    Type { selector: String, text: String },
    /// Scroll the page.
    Scroll { direction: ScrollDirection, amount: u32 },
    /// Take a screenshot.
    Screenshot,
    /// Wait for a condition.
    Wait { selector: Option<String>, timeout_ms: u64 },
    /// Extract content from the page.
    Extract { selector: Option<String> },
}

/// Direction for scrolling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// The status of a browser task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Task is waiting to be executed.
    #[default]
    Pending,
    /// Task is currently running.
    Running,
    /// Task completed successfully.
    Completed,
    /// Task failed with an error.
    Failed,
    /// Task was cancelled.
    Cancelled,
}

/// A browser automation task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserTask {
    /// The target URL for the task.
    pub url: String,
    /// The action to perform.
    pub action: BrowserAction,
    /// The current status of the task.
    pub status: TaskStatus,
    /// The result of the task (if completed).
    pub result: Option<String>,
    /// Error message (if failed).
    pub error: Option<String>,
}

impl BrowserTask {
    /// Creates a new browser task with the given URL and action.
    pub fn new(url: impl Into<String>, action: BrowserAction) -> Self {
        Self {
            url: url.into(),
            action,
            status: TaskStatus::Pending,
            result: None,
            error: None,
        }
    }

    /// Creates a navigation task for the given URL.
    pub fn navigate(url: impl Into<String>) -> Self {
        Self::new(url, BrowserAction::Navigate)
    }

    /// Marks the task as running.
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
    }

    /// Marks the task as completed with a result.
    pub fn complete(&mut self, result: impl Into<String>) {
        self.status = TaskStatus::Completed;
        self.result = Some(result.into());
    }

    /// Marks the task as failed with an error message.
    pub fn fail(&mut self, error: impl Into<String>) {
        self.status = TaskStatus::Failed;
        self.error = Some(error.into());
    }

    /// Cancels the task.
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
    }

    /// Returns true if the task is pending.
    pub fn is_pending(&self) -> bool {
        self.status == TaskStatus::Pending
    }

    /// Returns true if the task is running.
    pub fn is_running(&self) -> bool {
        self.status == TaskStatus::Running
    }

    /// Returns true if the task is completed.
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    /// Returns true if the task failed.
    pub fn is_failed(&self) -> bool {
        self.status == TaskStatus::Failed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_task_creation() {
        let task = BrowserTask::navigate("https://example.com");
        assert_eq!(task.url, "https://example.com");
        assert_eq!(task.action, BrowserAction::Navigate);
        assert!(task.is_pending());
    }

    #[test]
    fn test_browser_task_lifecycle() {
        let mut task = BrowserTask::navigate("https://example.com");
        assert!(task.is_pending());

        task.start();
        assert!(task.is_running());

        task.complete("Page loaded successfully");
        assert!(task.is_completed());
        assert_eq!(task.result, Some("Page loaded successfully".to_string()));
    }

    #[test]
    fn test_browser_task_failure() {
        let mut task = BrowserTask::navigate("https://example.com");
        task.start();
        task.fail("Connection timeout");

        assert!(task.is_failed());
        assert_eq!(task.error, Some("Connection timeout".to_string()));
    }

    #[test]
    fn test_browser_task_serialization() {
        let task = BrowserTask::navigate("https://example.com");
        let json = serde_json::to_string(&task).unwrap();
        let deserialized: BrowserTask = serde_json::from_str(&json).unwrap();
        assert_eq!(task, deserialized);
    }
}
