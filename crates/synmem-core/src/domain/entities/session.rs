//! Browser session entity

use serde::{Deserialize, Serialize};

/// Represents a browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: String,
    /// Current URL
    pub current_url: Option<String>,
    /// Session status
    pub status: SessionStatus,
    /// Creation timestamp
    pub created_at: i64,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Session is active
    Active,
    /// Session is idle
    Idle,
    /// Session is closed
    Closed,
}

impl Session {
    /// Creates a new session
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            current_url: None,
            status: SessionStatus::Idle,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
        }
    }
}
