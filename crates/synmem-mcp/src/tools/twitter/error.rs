//! Twitter error types

use thiserror::Error;

/// Errors that can occur during Twitter operations
#[derive(Debug, Error)]
pub enum TwitterError {
    /// Tweet text exceeds maximum length
    #[error("Tweet exceeds maximum length of {max} characters (got {actual})")]
    TweetTooLong { max: usize, actual: usize },

    /// No valid session available
    #[error("No valid Twitter session. Please log in first.")]
    NoSession,

    /// Session has expired
    #[error("Twitter session has expired. Please log in again.")]
    SessionExpired,

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please wait {wait_seconds} seconds.")]
    RateLimited { wait_seconds: u64 },

    /// Tweet not found
    #[error("Tweet not found: {tweet_id}")]
    TweetNotFound { tweet_id: String },

    /// User not found
    #[error("User not found: {username}")]
    UserNotFound { username: String },

    /// Network error
    #[error("Network error: {message}")]
    NetworkError { message: String },

    /// Authentication error
    #[error("Authentication error: {message}")]
    AuthError { message: String },

    /// Invalid input
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    /// Unknown error
    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

impl TwitterError {
    /// Check if the error is recoverable (can be retried)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            TwitterError::RateLimited { .. } | TwitterError::NetworkError { .. }
        )
    }

    /// Get the suggested wait time before retry (in seconds)
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            TwitterError::RateLimited { wait_seconds } => Some(*wait_seconds),
            TwitterError::NetworkError { .. } => Some(5),
            _ => None,
        }
    }
}
