//! Browser task entity representing automation operations

use serde::{Deserialize, Serialize};

/// Represents a browser automation task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTask {
    /// Unique identifier for the task
    pub id: String,
    /// Type of task to execute
    pub task_type: TaskType,
    /// Current status of the task
    pub status: TaskStatus,
    /// Target URL if applicable
    pub url: Option<String>,
    /// CSS selector if applicable
    pub selector: Option<String>,
    /// Text value for input operations
    pub value: Option<String>,
}

/// Types of browser tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    /// Navigate to a URL
    Navigate,
    /// Click on an element
    Click,
    /// Type text into an element
    Type,
    /// Select an option from a dropdown
    Select,
    /// Take a screenshot
    Screenshot,
    /// Extract HTML content
    GetHtml,
    /// Execute JavaScript
    EvaluateJs,
    /// Go back in history
    GoBack,
    /// Go forward in history
    GoForward,
    /// Refresh the page
    Refresh,
}

/// Status of a browser task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is pending execution
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed with an error
    Failed(String),
}

impl BrowserTask {
    /// Create a new browser task
    pub fn new(id: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            id: id.into(),
            task_type,
            status: TaskStatus::Pending,
            url: None,
            selector: None,
            value: None,
        }
    }

    /// Set the URL for the task
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the selector for the task
    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    /// Set the value for the task
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_browser_task() {
        let task = BrowserTask::new("task-1", TaskType::Navigate)
            .with_url("https://example.com");
        
        assert_eq!(task.id, "task-1");
        assert_eq!(task.task_type, TaskType::Navigate);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_task_builder() {
        let task = BrowserTask::new("task-2", TaskType::Type)
            .with_selector("#input")
            .with_value("Hello World");
        
        assert_eq!(task.selector, Some("#input".to_string()));
        assert_eq!(task.value, Some("Hello World".to_string()));
    }
}
