//! Session sync service for cross-device synchronization

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

/// Session sync errors
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    NotFound(Uuid),

    #[error("Session sync conflict")]
    SyncConflict,

    #[error("User not authorized to access session")]
    Unauthorized,

    #[error("Session data too large")]
    DataTooLarge,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// A synced session containing browser state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncedSession {
    /// Unique session identifier
    pub id: Uuid,
    /// User who owns this session
    pub user_id: Uuid,
    /// Human-readable session name
    pub name: String,
    /// Device identifier where session was created
    pub device_id: String,
    /// Serialized cookies
    pub cookies: Option<Vec<u8>>,
    /// Serialized local storage
    pub local_storage: Option<Vec<u8>>,
    /// Serialized session storage
    pub session_storage: Option<Vec<u8>>,
    /// URLs in the session
    pub urls: Vec<String>,
    /// Version for conflict resolution
    pub version: u64,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last sync timestamp
    pub synced_at: chrono::DateTime<chrono::Utc>,
}

impl SyncedSession {
    /// Creates a new synced session
    #[must_use]
    pub fn new(user_id: Uuid, name: String, device_id: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            device_id,
            cookies: None,
            local_storage: None,
            session_storage: None,
            urls: Vec::new(),
            version: 1,
            created_at: now,
            synced_at: now,
        }
    }

    /// Calculates the approximate size of the session data in bytes
    #[must_use]
    pub fn data_size(&self) -> usize {
        let mut size = 0;
        if let Some(ref cookies) = self.cookies {
            size += cookies.len();
        }
        if let Some(ref ls) = self.local_storage {
            size += ls.len();
        }
        if let Some(ref ss) = self.session_storage {
            size += ss.len();
        }
        size
    }
}

/// Session change event for sync
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionChange {
    /// Session that changed
    pub session_id: Uuid,
    /// Type of change
    pub change_type: SessionChangeType,
    /// Device that made the change
    pub device_id: String,
    /// Timestamp of the change
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Types of session changes
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionChangeType {
    Created,
    Updated,
    Deleted,
}

/// Trait for session sync operations
#[async_trait::async_trait]
pub trait SessionServiceTrait: Send + Sync {
    /// Creates a new synced session
    async fn create(&self, session: SyncedSession) -> Result<SyncedSession, SessionError>;

    /// Gets a session by ID
    async fn get(&self, session_id: Uuid, user_id: Uuid) -> Result<SyncedSession, SessionError>;

    /// Updates a session (with conflict detection)
    async fn update(&self, session: SyncedSession) -> Result<SyncedSession, SessionError>;

    /// Deletes a session
    async fn delete(&self, session_id: Uuid, user_id: Uuid) -> Result<(), SessionError>;

    /// Lists all sessions for a user
    async fn list(&self, user_id: Uuid) -> Result<Vec<SyncedSession>, SessionError>;

    /// Gets changes since a given version
    async fn get_changes(
        &self,
        user_id: Uuid,
        since_version: u64,
    ) -> Result<Vec<SessionChange>, SessionError>;
}

/// Default implementation of the session service
///
/// # Note
/// This is a placeholder implementation. The actual implementation will need:
/// - Database connection pool
/// - Encryption service for session data
/// - Sync protocol handler
///
/// TODO: Implement `SessionServiceTrait` when infrastructure is ready
#[derive(Default)]
pub struct SessionService {
    _private: (),
}

impl SessionService {
    /// Creates a new session service
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { _private: () })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synced_session_creation() {
        let user_id = Uuid::new_v4();
        let session = SyncedSession::new(user_id, "Test Session".to_string(), "device-1".to_string());

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.name, "Test Session");
        assert_eq!(session.version, 1);
        assert_eq!(session.data_size(), 0);
    }

    #[test]
    fn test_session_data_size() {
        let user_id = Uuid::new_v4();
        let mut session = SyncedSession::new(user_id, "Test".to_string(), "device".to_string());

        session.cookies = Some(vec![0u8; 1000]);
        session.local_storage = Some(vec![0u8; 500]);

        assert_eq!(session.data_size(), 1500);
    }

    #[test]
    fn test_session_error_display() {
        let session_id = Uuid::new_v4();
        let err = SessionError::NotFound(session_id);
        assert!(err.to_string().contains(&session_id.to_string()));
    }
}
