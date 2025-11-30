//! Session manager for storing and retrieving browser sessions

use std::collections::HashMap;
use std::sync::RwLock;
use synmem_core::domain::entities::Session;

/// Manages browser sessions in memory
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Save a session
    pub fn save(&self, session: &Session) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session.id.clone(), session.clone());
    }

    /// Get a session by ID
    pub fn get(&self, id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(id).cloned()
    }

    /// List all session IDs
    pub fn list(&self) -> Vec<String> {
        let sessions = self.sessions.read().unwrap();
        sessions.keys().cloned().collect()
    }

    /// Delete a session by ID
    pub fn delete(&self, id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(id)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_manager() {
        let manager = SessionManager::new();
        let session = Session::new("test-session").with_name("Test Session");

        manager.save(&session);
        
        let retrieved = manager.get("test-session");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Session");

        let ids = manager.list();
        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&"test-session".to_string()));

        let deleted = manager.delete("test-session");
        assert!(deleted.is_some());
        assert!(manager.get("test-session").is_none());
    }
}
