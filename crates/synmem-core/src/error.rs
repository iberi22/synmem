//! Error types for synmem-core

use thiserror::Error;

/// Core domain errors
#[derive(Error, Debug)]
pub enum CoreError {
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Recording session not found
    #[error("Recording session not found: {0}")]
    SessionNotFound(String),

    /// Macro not found
    #[error("Macro not found: {0}")]
    MacroNotFound(String),

    /// Recording already in progress
    #[error("Recording already in progress")]
    RecordingInProgress,

    /// No recording in progress
    #[error("No recording in progress")]
    NoRecordingInProgress,

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        CoreError::Serialization(e.to_string())
    }
}
