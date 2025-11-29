//! Error types for the native host

use thiserror::Error;

/// Errors that can occur in the native host
#[derive(Error, Debug)]
pub enum NativeHostError {
    /// Error reading from stdin
    #[error("Failed to read from stdin: {0}")]
    ReadError(#[from] std::io::Error),

    /// Error parsing JSON message
    #[error("Failed to parse message: {0}")]
    ParseError(#[from] serde_json::Error),

    /// Message too large
    #[error("Message too large: {size} bytes (max: {max} bytes)")]
    MessageTooLarge { size: usize, max: usize },

    /// Unknown action
    #[allow(dead_code)]
    #[error("Unknown action: {0}")]
    UnknownAction(String),

    /// Internal error
    #[allow(dead_code)]
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for native host operations
pub type Result<T> = std::result::Result<T, NativeHostError>;
