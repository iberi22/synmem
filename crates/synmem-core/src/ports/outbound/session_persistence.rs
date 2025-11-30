//! # Session Persistence Port
//!
//! Outbound port for cookies and session management.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during session persistence operations.
#[derive(Debug, Error)]
pub enum SessionPersistenceError {
    #[error("load failed: {0}")]
    LoadFailed(String),

    #[error("save failed: {0}")]
    SaveFailed(String),

    #[error("session not found: {0}")]
    SessionNotFound(String),

    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("decryption failed: {0}")]
    DecryptionFailed(String),
}

/// Result type for session persistence operations.
pub type SessionPersistenceResult<T> = Result<T, SessionPersistenceError>;

/// A browser cookie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    /// Cookie name.
    pub name: String,
    /// Cookie value.
    pub value: String,
    /// Domain the cookie belongs to.
    pub domain: String,
    /// Path the cookie is valid for.
    pub path: String,
    /// Expiration time.
    pub expires: Option<DateTime<Utc>>,
    /// HTTP-only flag.
    pub http_only: bool,
    /// Secure flag.
    pub secure: bool,
    /// SameSite attribute.
    pub same_site: Option<SameSite>,
}

/// SameSite cookie attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Local storage entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalStorageEntry {
    /// Storage key.
    pub key: String,
    /// Storage value.
    pub value: String,
}

/// A complete browser session state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    /// Session identifier (e.g., domain or profile name).
    pub session_id: String,
    /// All cookies.
    pub cookies: Vec<Cookie>,
    /// Local storage entries.
    pub local_storage: Vec<LocalStorageEntry>,
    /// When this session was captured.
    pub captured_at: DateTime<Utc>,
}

/// Outbound port for session persistence.
///
/// This port defines the interface for saving and restoring browser
/// session state (cookies, local storage, etc.). Session data should
/// be encrypted at rest.
#[async_trait]
pub trait SessionPersistencePort: Send + Sync {
    /// Save a session state.
    async fn save_session(&self, session: &SessionState) -> SessionPersistenceResult<()>;

    /// Load a session state by ID.
    async fn load_session(&self, session_id: &str) -> SessionPersistenceResult<SessionState>;

    /// List all saved session IDs.
    async fn list_sessions(&self) -> SessionPersistenceResult<Vec<String>>;

    /// Delete a saved session.
    async fn delete_session(&self, session_id: &str) -> SessionPersistenceResult<()>;

    /// Check if a session exists.
    async fn session_exists(&self, session_id: &str) -> SessionPersistenceResult<bool>;
}
