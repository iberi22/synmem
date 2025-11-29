//! Browser task entity representing an automation task

use serde::{Deserialize, Serialize};

/// Represents a browser automation task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTask {
    /// Unique identifier for the task
    pub id: String,
    /// Type of task (navigate, click, type, etc.)
    pub task_type: TaskType,
    /// Current status of the task
    pub status: TaskStatus,
    /// Optional error message if task failed
    pub error: Option<String>,
}

/// Types of browser tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Navigate,
    Click,
    Type,
    Scroll,
    Screenshot,
    WaitFor,
    Scrape,
}

/// Status of a browser task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl BrowserTask {
    /// Creates a new pending browser task
    pub fn new(id: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            id: id.into(),
            task_type,
            status: TaskStatus::Pending,
            error: None,
        }
    }
}
