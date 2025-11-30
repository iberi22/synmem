//! Session persistence port - Outbound interface for session storage
//!
//! This port defines the interface for storing and retrieving encrypted
//! session data from external storage systems.

use async_trait::async_trait;
use crate::domain::entities::session::SessionProfile;
use crate::domain::services::session_manager::SessionError;

/// Outbound port for session persistence operations
#[async_trait]
pub trait SessionPersistencePort: Send + Sync {
    /// Save a session profile to persistent storage
    async fn save_profile(&self, profile: &SessionProfile) -> Result<(), SessionError>;

    /// Load a session profile from persistent storage
    async fn load_profile(&self, profile_name: &str) -> Result<SessionProfile, SessionError>;

    /// Delete a session profile from persistent storage
    async fn delete_profile(&self, profile_name: &str) -> Result<(), SessionError>;

    /// List all stored profile names
    async fn list_profiles(&self) -> Result<Vec<String>, SessionError>;

    /// Check if a profile exists in storage
    async fn profile_exists(&self, profile_name: &str) -> Result<bool, SessionError>;
}
