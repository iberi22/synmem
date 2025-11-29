//! Session entity - represents a browser session with its state

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a browser session with cookies and storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    /// Unique identifier for the session
    pub id: String,
    /// Human-readable name for the session
    pub name: String,
    /// Browser profile or user agent identifier
    pub profile: Option<String>,
    /// Cookies stored in this session (serialized)
    pub cookies: Option<String>,
    /// Local storage data (serialized)
    pub local_storage: HashMap<String, String>,
    /// Session storage data (serialized)
    pub session_storage: HashMap<String, String>,
    /// Whether the session is currently active
    pub is_active: bool,
    /// Timestamp when the session was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the session was last updated
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Creates a new Session with the given name
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            profile: None,
            cookies: None,
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Sets the browser profile
    pub fn with_profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Sets the cookies
    pub fn with_cookies(mut self, cookies: impl Into<String>) -> Self {
        self.cookies = Some(cookies.into());
        self
    }

    /// Adds a local storage entry
    pub fn with_local_storage(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.local_storage.insert(key.into(), value.into());
        self
    }

    /// Marks the session as inactive
    pub fn deactivate(mut self) -> Self {
        self.is_active = false;
        self.updated_at = Utc::now();
        self
    }

    /// Updates the timestamp
    pub fn touch(mut self) -> Self {
        self.updated_at = Utc::now();
        self
    }
}
