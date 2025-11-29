//! Session Entity
//!
//! Represents a browser session with cookies and storage state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A browser session with authentication state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier for the session.
    pub id: Uuid,
    /// Session name or label.
    pub name: Option<String>,
    /// Stored cookies as a JSON string.
    pub cookies: Option<String>,
    /// Browser storage state (localStorage, sessionStorage) as JSON.
    pub storage_state: Option<String>,
    /// User agent string for the session.
    pub user_agent: Option<String>,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// When the session was last used.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Whether the session is still valid.
    pub is_active: bool,
}

impl Session {
    /// Creates a new session with a generated UUID and current timestamp.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: None,
            cookies: None,
            storage_state: None,
            user_agent: None,
            created_at: Utc::now(),
            last_used_at: None,
            is_active: true,
        }
    }

    /// Creates a new session with a specific name.
    pub fn with_name(name: impl Into<String>) -> Self {
        let mut session = Self::new();
        session.name = Some(name.into());
        session
    }

    /// Sets the cookies for this session.
    pub fn set_cookies(&mut self, cookies: impl Into<String>) {
        self.cookies = Some(cookies.into());
    }

    /// Sets the storage state for this session.
    pub fn set_storage_state(&mut self, storage_state: impl Into<String>) {
        self.storage_state = Some(storage_state.into());
    }

    /// Sets the user agent for this session.
    pub fn set_user_agent(&mut self, user_agent: impl Into<String>) {
        self.user_agent = Some(user_agent.into());
    }

    /// Updates the last used timestamp to now.
    pub fn touch(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    /// Marks the session as inactive.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Returns true if the session has cookies.
    pub fn has_cookies(&self) -> bool {
        self.cookies.is_some()
    }

    /// Returns true if the session has storage state.
    pub fn has_storage_state(&self) -> bool {
        self.storage_state.is_some()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert!(session.is_active);
        assert!(session.cookies.is_none());
        assert!(session.storage_state.is_none());
    }

    #[test]
    fn test_session_with_name() {
        let session = Session::with_name("twitter-auth");
        assert_eq!(session.name, Some("twitter-auth".to_string()));
    }

    #[test]
    fn test_session_cookies() {
        let mut session = Session::new();
        assert!(!session.has_cookies());

        session.set_cookies(r#"[{"name": "auth", "value": "token123"}]"#);
        assert!(session.has_cookies());
    }

    #[test]
    fn test_session_deactivation() {
        let mut session = Session::new();
        assert!(session.is_active);

        session.deactivate();
        assert!(!session.is_active);
    }

    #[test]
    fn test_session_serialization() {
        let session = Session::with_name("test-session");
        let json = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(session.id, deserialized.id);
        assert_eq!(session.name, deserialized.name);
    }
}
