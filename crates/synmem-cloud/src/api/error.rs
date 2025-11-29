//! API error types for SynMem Cloud

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::services::{
    auth::AuthError, browser_pool::BrowserPoolError, session::SessionError, storage::StorageError,
};

/// API error response body
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

/// Error detail for API responses
#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// API error type that implements IntoResponse
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    /// Creates a new API error
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Adds details to the error
    #[must_use]
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Creates a bad request error
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    /// Creates an unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    /// Creates a forbidden error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "forbidden", message)
    }

    /// Creates a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// Creates a rate limit error
    pub fn rate_limited(reset_at: i64) -> Self {
        Self::new(
            StatusCode::TOO_MANY_REQUESTS,
            "rate_limited",
            "Rate limit exceeded",
        )
        .with_details(serde_json::json!({ "reset_at": reset_at }))
    }

    /// Creates an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorResponse {
            error: ErrorDetail {
                code: self.code,
                message: self.message,
                details: self.details,
            },
        };

        (self.status, axum::Json(body)).into_response()
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => Self::unauthorized("Invalid credentials"),
            AuthError::InvalidApiKey => Self::unauthorized("Invalid API key"),
            AuthError::ApiKeyExpired => Self::unauthorized("API key has expired"),
            AuthError::InsufficientPermissions(scope) => {
                Self::forbidden(format!("Insufficient permissions: missing scope {scope}"))
            }
            AuthError::UserNotFound => Self::not_found("User not found"),
            AuthError::TokenExpired => Self::unauthorized("Token has expired"),
            AuthError::Internal(msg) => Self::internal(msg),
        }
    }
}

impl From<BrowserPoolError> for ApiError {
    fn from(err: BrowserPoolError) -> Self {
        match err {
            BrowserPoolError::CapacityReached(tier) => Self::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "pool_capacity_reached",
                format!("Browser pool capacity reached for {tier} tier"),
            ),
            BrowserPoolError::SessionNotFound(id) => {
                Self::not_found(format!("Browser session not found: {id}"))
            }
            BrowserPoolError::SessionExpired(id) => Self::bad_request(format!("Browser session expired: {id}")),
            BrowserPoolError::BrowserCrashed(msg) => Self::internal(format!("Browser crashed: {msg}")),
            BrowserPoolError::Internal(msg) => Self::internal(msg),
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(err: SessionError) -> Self {
        match err {
            SessionError::NotFound(id) => Self::not_found(format!("Session not found: {id}")),
            SessionError::SyncConflict => Self::new(
                StatusCode::CONFLICT,
                "sync_conflict",
                "Session sync conflict detected",
            ),
            SessionError::Unauthorized => Self::forbidden("Not authorized to access this session"),
            SessionError::DataTooLarge => Self::bad_request("Session data exceeds size limit"),
            SessionError::Internal(msg) => Self::internal(msg),
        }
    }
}

impl From<StorageError> for ApiError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(key) => Self::not_found(format!("Object not found: {key}")),
            StorageError::AlreadyExists(key) => Self::new(
                StatusCode::CONFLICT,
                "already_exists",
                format!("Object already exists: {key}"),
            ),
            StorageError::QuotaExceeded => Self::new(
                StatusCode::INSUFFICIENT_STORAGE,
                "quota_exceeded",
                "Storage quota exceeded",
            ),
            StorageError::InvalidKey => Self::bad_request("Invalid storage key format"),
            StorageError::Database(msg) => Self::internal(format!("Database error: {msg}")),
            StorageError::Internal(msg) => Self::internal(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_creation() {
        let err = ApiError::bad_request("Invalid input");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.code, "bad_request");
    }

    #[test]
    fn test_api_error_with_details() {
        let err = ApiError::rate_limited(1234567890);
        assert_eq!(err.status, StatusCode::TOO_MANY_REQUESTS);
        assert!(err.details.is_some());
    }

    #[test]
    fn test_auth_error_conversion() {
        let auth_err = AuthError::InvalidApiKey;
        let api_err: ApiError = auth_err.into();
        assert_eq!(api_err.status, StatusCode::UNAUTHORIZED);
    }
}
