//! Authentication service for SynMem Cloud

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{ApiKey, ApiScope, User};

/// Authentication errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("API key expired")]
    ApiKeyExpired,

    #[error("Insufficient permissions: missing scope {0}")]
    InsufficientPermissions(ApiScope),

    #[error("User not found")]
    UserNotFound,

    #[error("Token expired")]
    TokenExpired,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Authentication result containing user and optional API key
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub user: User,
    pub api_key: Option<ApiKey>,
}

/// Trait for authentication operations
#[async_trait::async_trait]
pub trait AuthServiceTrait: Send + Sync {
    /// Authenticates a user by API key
    async fn authenticate_api_key(&self, key: &str) -> Result<AuthResult, AuthError>;

    /// Authenticates a user by JWT token (from external auth provider)
    async fn authenticate_token(&self, token: &str) -> Result<AuthResult, AuthError>;

    /// Creates a new API key for a user
    async fn create_api_key(
        &self,
        user_id: Uuid,
        name: String,
        scopes: Vec<ApiScope>,
    ) -> Result<(ApiKey, String), AuthError>;

    /// Revokes an API key
    async fn revoke_api_key(&self, key_id: Uuid, user_id: Uuid) -> Result<(), AuthError>;

    /// Lists all API keys for a user
    async fn list_api_keys(&self, user_id: Uuid) -> Result<Vec<ApiKey>, AuthError>;
}

/// Default implementation of the authentication service
///
/// # Note
/// This is a placeholder implementation. The actual implementation will need:
/// - Database connection pool for user/API key storage
/// - JWT validation configuration
/// - External auth provider client (Clerk/Auth0)
///
/// TODO: Implement `AuthServiceTrait` when infrastructure is ready
#[derive(Default)]
pub struct AuthService {
    _private: (),
}

impl AuthService {
    /// Creates a new authentication service
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { _private: () })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_error_display() {
        let err = AuthError::InsufficientPermissions(ApiScope::Write);
        assert!(err.to_string().contains("write"));
    }

    #[test]
    fn test_auth_service_creation() {
        let _service = AuthService::new();
    }
}
