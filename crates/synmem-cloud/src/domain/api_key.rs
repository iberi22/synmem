//! API key management for SynMem Cloud

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Scopes that can be assigned to an API key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiScope {
    /// Read-only access to scraping results
    Read,
    /// Write access for creating scrape jobs
    Write,
    /// Full access including settings and billing
    Admin,
    /// Access to browser sessions
    BrowserSessions,
    /// Access to session sync
    SessionSync,
}

impl ApiScope {
    /// Returns all available scopes
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Read,
            Self::Write,
            Self::Admin,
            Self::BrowserSessions,
            Self::SessionSync,
        ]
    }
}

impl std::fmt::Display for ApiScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
            Self::Admin => write!(f, "admin"),
            Self::BrowserSessions => write!(f, "browser_sessions"),
            Self::SessionSync => write!(f, "session_sync"),
        }
    }
}

/// An API key for programmatic access to SynMem Cloud
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Unique API key identifier
    pub id: Uuid,
    /// User who owns this key
    pub user_id: Uuid,
    /// Human-readable name for the key
    pub name: String,
    /// The hashed key value (never store plaintext)
    #[serde(skip_serializing)]
    pub key_hash: String,
    /// Key prefix for identification (e.g., "sk_live_abc...")
    pub key_prefix: String,
    /// Scopes granted to this key
    pub scopes: Vec<ApiScope>,
    /// Optional expiration date
    pub expires_at: Option<DateTime<Utc>>,
    /// Last time this key was used
    pub last_used_at: Option<DateTime<Utc>>,
    /// Whether the key is active
    pub is_active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl ApiKey {
    /// Creates a new API key with the given scopes
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user who owns this key
    /// * `name` - Human-readable name for the key
    /// * `key_hash` - Hash of the actual key value
    /// * `key_prefix` - First few characters of the key for identification
    /// * `scopes` - Scopes to grant to this key
    #[must_use]
    pub fn new(
        user_id: Uuid,
        name: String,
        key_hash: String,
        key_prefix: String,
        scopes: Vec<ApiScope>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            key_hash,
            key_prefix,
            scopes,
            expires_at: None,
            last_used_at: None,
            is_active: true,
            created_at: Utc::now(),
        }
    }

    /// Checks if this key has the given scope
    #[must_use]
    pub fn has_scope(&self, scope: ApiScope) -> bool {
        self.scopes.contains(&scope) || self.scopes.contains(&ApiScope::Admin)
    }

    /// Checks if this key is valid (active and not expired)
    #[must_use]
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            if expires_at < Utc::now() {
                return false;
            }
        }

        true
    }

    /// Records a usage of this key
    pub fn record_usage(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    /// Revokes this key
    pub fn revoke(&mut self) {
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_creation() {
        let user_id = Uuid::new_v4();
        let key = ApiKey::new(
            user_id,
            "Test Key".to_string(),
            "hashed_value".to_string(),
            "sk_test_abc".to_string(),
            vec![ApiScope::Read, ApiScope::Write],
        );

        assert_eq!(key.user_id, user_id);
        assert_eq!(key.name, "Test Key");
        assert!(key.is_active);
        assert!(key.is_valid());
    }

    #[test]
    fn test_scope_checking() {
        let key = ApiKey::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "hash".to_string(),
            "sk_".to_string(),
            vec![ApiScope::Read],
        );

        assert!(key.has_scope(ApiScope::Read));
        assert!(!key.has_scope(ApiScope::Write));
    }

    #[test]
    fn test_admin_scope_grants_all() {
        let key = ApiKey::new(
            Uuid::new_v4(),
            "Admin Key".to_string(),
            "hash".to_string(),
            "sk_".to_string(),
            vec![ApiScope::Admin],
        );

        assert!(key.has_scope(ApiScope::Read));
        assert!(key.has_scope(ApiScope::Write));
        assert!(key.has_scope(ApiScope::BrowserSessions));
    }

    #[test]
    fn test_key_revocation() {
        let mut key = ApiKey::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "hash".to_string(),
            "sk_".to_string(),
            vec![ApiScope::Read],
        );

        assert!(key.is_valid());
        key.revoke();
        assert!(!key.is_valid());
    }

    #[test]
    fn test_expired_key() {
        let mut key = ApiKey::new(
            Uuid::new_v4(),
            "Test".to_string(),
            "hash".to_string(),
            "sk_".to_string(),
            vec![ApiScope::Read],
        );

        key.expires_at = Some(Utc::now() - chrono::Duration::hours(1));
        assert!(!key.is_valid());
    }
}
