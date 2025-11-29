//! Browser session entity.

use serde::{Deserialize, Serialize};

/// Represents a browser session with authentication state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier for the session.
    pub id: String,
    /// Human-readable name for the session.
    pub name: String,
    /// Whether the session is currently active.
    pub active: bool,
    /// Browser user data directory path.
    pub user_data_dir: Option<String>,
}

impl Session {
    /// Create a new session.
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            active: false,
            user_data_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new("session-1".to_string(), "Default".to_string());
        assert_eq!(session.id, "session-1");
        assert_eq!(session.name, "Default");
        assert!(!session.active);
        assert!(session.user_data_dir.is_none());
    }
}
