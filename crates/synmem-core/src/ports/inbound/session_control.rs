//! Session control port - Inbound interface for session management
//!
//! This port defines the interface for external systems to interact with
//! the session management functionality.

use async_trait::async_trait;
use crate::domain::entities::session::{Cookie, Session, SessionProfile};
use crate::domain::services::session_manager::SessionError;

/// Inbound port for session control operations
#[async_trait]
pub trait SessionControlPort: Send + Sync {
    /// Create a new session with the given cookies
    async fn create_session(
        &mut self,
        profile_name: &str,
        cookies: Vec<Cookie>,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError>;

    /// Load an existing session
    async fn load_session(
        &mut self,
        profile_name: &str,
        master_password: &str,
    ) -> Result<Session, SessionError>;

    /// List all available session profiles
    async fn list_profiles(&self) -> Result<Vec<String>, SessionError>;

    /// Get information about a specific profile (without decrypting)
    async fn get_profile_info(&self, profile_name: &str) -> Result<SessionProfile, SessionError>;

    /// Delete a session profile
    async fn delete_profile(&mut self, profile_name: &str) -> Result<(), SessionError>;

    /// Update cookies in an existing session
    async fn update_cookies(
        &mut self,
        profile_name: &str,
        cookies: Vec<Cookie>,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError>;

    /// Refresh a session (extend expiration)
    async fn refresh_session(
        &mut self,
        profile_name: &str,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError>;

    /// Unload a session from memory (secure cleanup)
    async fn unload_session(&mut self, profile_name: &str);

    /// Check if a session is currently loaded in memory
    fn is_session_loaded(&self, profile_name: &str) -> bool;
}
