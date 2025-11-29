//! Browser driver outbound port
//!
//! This port defines the interface for browser driver implementations.

use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during browser driver operations
#[derive(Debug, Error)]
pub enum BrowserDriverError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Command failed: {0}")]
    CommandFailed(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for browser driver operations
pub type BrowserDriverResult<T> = Result<T, BrowserDriverError>;

/// Browser driver outbound port trait
#[async_trait]
pub trait BrowserDriverPort: Send + Sync {
    /// Launches a new browser instance
    async fn launch(&self) -> BrowserDriverResult<String>;

    /// Closes a browser instance
    async fn close(&self, instance_id: &str) -> BrowserDriverResult<()>;

    /// Navigates to a URL
    async fn navigate_to(&self, instance_id: &str, url: &str) -> BrowserDriverResult<()>;

    /// Gets the current URL
    async fn get_url(&self, instance_id: &str) -> BrowserDriverResult<String>;

    /// Executes JavaScript
    async fn execute_script(
        &self,
        instance_id: &str,
        script: &str,
    ) -> BrowserDriverResult<String>;
}
