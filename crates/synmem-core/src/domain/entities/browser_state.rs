//! Browser state entity for simple session management without encryption
//!
//! This module provides a lightweight session representation for browser state
//! that doesn't require encryption. Use Session/SessionProfile for encrypted sessions.

use serde::{Deserialize, Serialize};

/// Represents a browser session with cookies and state (unencrypted)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserState {
    /// Unique identifier for the session
    pub id: String,
    /// Session name for identification
    pub name: String,
    /// Stored cookies for the session
    pub cookies: Vec<SimpleCookie>,
    /// Local storage data
    pub local_storage: Vec<StorageEntry>,
    /// Session storage data
    pub session_storage: Vec<StorageEntry>,
}

/// Represents a browser cookie (simple, unencrypted version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Domain the cookie is valid for
    pub domain: String,
    /// Path the cookie is valid for
    pub path: String,
    /// Whether the cookie is secure
    pub secure: bool,
    /// Whether the cookie is HTTP only
    pub http_only: bool,
    /// Expiration timestamp (Unix epoch)
    pub expires: Option<i64>,
    /// SameSite attribute
    pub same_site: SameSite,
}

/// SameSite cookie attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SameSite {
    /// No SameSite attribute
    #[default]
    None,
    /// Lax SameSite
    Lax,
    /// Strict SameSite
    Strict,
}

/// Entry in local or session storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    /// Storage key
    pub key: String,
    /// Storage value
    pub value: String,
    /// Origin URL
    pub origin: String,
}

impl BrowserState {
    /// Create a new browser state with the given ID
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: String::new(),
            cookies: Vec::new(),
            local_storage: Vec::new(),
            session_storage: Vec::new(),
        }
    }

    /// Set the session name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Add a cookie to the session
    pub fn add_cookie(&mut self, cookie: SimpleCookie) {
        self.cookies.push(cookie);
    }

    /// Add a local storage entry
    pub fn add_local_storage(&mut self, entry: StorageEntry) {
        self.local_storage.push(entry);
    }

    /// Add a session storage entry  
    pub fn add_session_storage(&mut self, entry: StorageEntry) {
        self.session_storage.push(entry);
    }
}

impl SimpleCookie {
    /// Create a new cookie with required fields
    pub fn new(name: impl Into<String>, value: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: domain.into(),
            path: "/".to_string(),
            secure: false,
            http_only: false,
            expires: None,
            same_site: SameSite::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_browser_state() {
        let state = BrowserState::new("session-1").with_name("My Session");
        
        assert_eq!(state.id, "session-1");
        assert_eq!(state.name, "My Session");
        assert!(state.cookies.is_empty());
    }

    #[test]
    fn test_add_cookie() {
        let mut state = BrowserState::new("session-1");
        let cookie = SimpleCookie::new("auth", "token123", "example.com");
        
        state.add_cookie(cookie);
        
        assert_eq!(state.cookies.len(), 1);
        assert_eq!(state.cookies[0].name, "auth");
    }
}
