//! Session entity representing a browser session

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier
    pub id: Uuid,

    /// Session name/description
    pub name: String,

    /// When the session was created
    pub created_at: DateTime<Utc>,

    /// When the session was last active
    pub last_active_at: DateTime<Utc>,

    /// Whether the session is currently active
    pub is_active: bool,

    /// Number of items in this session
    pub item_count: u32,

    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

impl Session {
    /// Creates a new Session
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: now,
            last_active_at: now,
            is_active: true,
            item_count: 0,
            metadata: None,
        }
    }

    /// Updates the last active timestamp
    pub fn touch(&mut self) {
        self.last_active_at = Utc::now();
    }

    /// Increments the item count
    pub fn increment_items(&mut self) {
        self.item_count += 1;
        self.touch();
    }

    /// Marks the session as inactive
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("Test Session".to_string());

        assert_eq!(session.name, "Test Session");
        assert!(session.is_active);
        assert_eq!(session.item_count, 0);
    }

    #[test]
    fn test_session_operations() {
        let mut session = Session::new("Test".to_string());
        session.increment_items();
        session.increment_items();

        assert_eq!(session.item_count, 2);

        session.deactivate();
        assert!(!session.is_active);
    }
}
