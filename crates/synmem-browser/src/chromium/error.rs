//! Error types for the Chromium driver

use thiserror::Error;

/// Errors that can occur in the Chromium driver
#[derive(Error, Debug)]
pub enum ChromiumError {
    /// Browser launch failed
    #[error("Failed to launch browser: {0}")]
    LaunchError(String),

    /// Navigation failed
    #[error("Navigation failed: {0}")]
    NavigationError(String),

    /// Element not found
    #[error("Element not found: {selector}")]
    ElementNotFound { selector: String },

    /// Element interaction failed
    #[error("Element interaction failed: {0}")]
    InteractionError(String),

    /// JavaScript evaluation failed
    #[error("JavaScript evaluation failed: {0}")]
    JsError(String),

    /// Screenshot failed
    #[error("Screenshot failed: {0}")]
    ScreenshotError(String),

    /// Session operation failed
    #[error("Session operation failed: {0}")]
    SessionError(String),

    /// Browser connection error
    #[error("Browser connection error: {0}")]
    ConnectionError(String),

    /// Timeout error
    #[error("Timeout after {timeout_ms}ms waiting for: {description}")]
    Timeout { timeout_ms: u64, description: String },

    /// Internal chromiumoxide error
    #[error("Chromiumoxide error: {0}")]
    ChromiumOxide(String),
}

impl From<chromiumoxide::error::CdpError> for ChromiumError {
    fn from(err: chromiumoxide::error::CdpError) -> Self {
        ChromiumError::ChromiumOxide(err.to_string())
    }
}
