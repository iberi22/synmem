//! Browser task entity.

use serde::{Deserialize, Serialize};

/// Represents a browser navigation/automation task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTask {
    /// Unique identifier for the task.
    pub id: String,
    /// Type of task (navigate, click, type, etc.).
    pub task_type: TaskType,
    /// Current status of the task.
    pub status: TaskStatus,
    /// Optional error message if task failed.
    pub error: Option<String>,
}

/// Types of browser tasks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    Navigate { url: String, wait_for: WaitCondition },
    Click { selector: Option<String>, text: Option<String> },
    TypeText { selector: String, text: String },
    Scroll { direction: ScrollDirection, amount: Option<i32> },
    Screenshot { full_page: bool, path: Option<String> },
    WaitFor { selector: String, timeout_ms: u64 },
}

/// Wait conditions for navigation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum WaitCondition {
    #[default]
    Load,
    DomContentLoaded,
    NetworkIdle,
}

/// Scroll direction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ScrollDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

/// Task execution status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum TaskStatus {
    #[default]
    Pending,
    Running,
    Completed,
    Failed,
}

impl BrowserTask {
    /// Create a new browser task.
    pub fn new(id: String, task_type: TaskType) -> Self {
        Self {
            id,
            task_type,
            status: TaskStatus::Pending,
            error: None,
        }
    }

    /// Mark task as completed.
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    /// Mark task as failed with an error message.
    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.error = Some(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_task_creation() {
        let task = BrowserTask::new(
            "task-1".to_string(),
            TaskType::Navigate {
                url: "https://example.com".to_string(),
                wait_for: WaitCondition::Load,
            },
        );

        assert_eq!(task.id, "task-1");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.error.is_none());
    }

    #[test]
    fn test_browser_task_complete() {
        let mut task = BrowserTask::new(
            "task-1".to_string(),
            TaskType::Click {
                selector: Some("#button".to_string()),
                text: None,
            },
        );

        task.complete();
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_browser_task_fail() {
        let mut task = BrowserTask::new(
            "task-1".to_string(),
            TaskType::TypeText {
                selector: "#input".to_string(),
                text: "hello".to_string(),
            },
        );

        task.fail("Element not found".to_string());
        assert_eq!(task.status, TaskStatus::Failed);
        assert_eq!(task.error, Some("Element not found".to_string()));
    }
}
