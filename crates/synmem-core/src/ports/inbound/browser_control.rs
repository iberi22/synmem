//! Browser control inbound port
//!
//! This port defines the interface for controlling browser automation.

use crate::domain::entities::{BrowserTask, Session};
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during browser control operations
#[derive(Debug, Error)]
pub enum BrowserControlError {
    #[error("Navigation failed: {0}")]
    NavigationFailed(String),
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    #[error("Click failed: {0}")]
    ClickFailed(String),
    #[error("Type failed: {0}")]
    TypeFailed(String),
    #[error("Screenshot failed: {0}")]
    ScreenshotFailed(String),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("Session error: {0}")]
    SessionError(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for browser control operations
pub type BrowserControlResult<T> = Result<T, BrowserControlError>;

/// Navigation options
#[derive(Debug, Clone, Default)]
pub struct NavigateOptions {
    /// Wait for page to fully load
    pub wait_until: Option<WaitUntil>,
    /// Timeout in milliseconds
    pub timeout_ms: Option<u64>,
}

/// Wait conditions for navigation
#[derive(Debug, Clone, Default)]
pub enum WaitUntil {
    /// Wait until DOM content is loaded
    #[default]
    DomContentLoaded,
    /// Wait until page is fully loaded
    Load,
    /// Wait until network is idle
    NetworkIdle,
}

/// Click options
#[derive(Debug, Clone, Default)]
pub struct ClickOptions {
    /// Selector to find the element
    pub selector: Option<String>,
    /// Text content to match
    pub text: Option<String>,
    /// Number of clicks (default: 1)
    pub click_count: Option<u32>,
    /// Button to click (default: left)
    pub button: Option<MouseButton>,
}

/// Mouse button
#[derive(Debug, Clone, Default)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

/// Type options
#[derive(Debug, Clone)]
pub struct TypeOptions {
    /// Selector to find the input
    pub selector: String,
    /// Text to type
    pub text: String,
    /// Clear existing text first
    pub clear: bool,
    /// Delay between keystrokes in milliseconds
    pub delay_ms: Option<u64>,
}

/// Screenshot options
#[derive(Debug, Clone, Default)]
pub struct ScreenshotOptions {
    /// Full page screenshot
    pub full_page: bool,
    /// Optional selector for element screenshot
    pub selector: Option<String>,
    /// Image format
    pub format: ScreenshotFormat,
}

/// Screenshot format
#[derive(Debug, Clone, Default)]
pub enum ScreenshotFormat {
    #[default]
    Png,
    Jpeg,
}

/// Browser control inbound port trait
#[async_trait]
pub trait BrowserControlPort: Send + Sync {
    /// Creates a new browser session
    async fn create_session(&self) -> BrowserControlResult<Session>;

    /// Closes a browser session
    async fn close_session(&self, session_id: &str) -> BrowserControlResult<()>;

    /// Navigates to a URL
    async fn navigate(
        &self,
        session_id: &str,
        url: &str,
        options: NavigateOptions,
    ) -> BrowserControlResult<BrowserTask>;

    /// Clicks on an element
    async fn click(
        &self,
        session_id: &str,
        options: ClickOptions,
    ) -> BrowserControlResult<BrowserTask>;

    /// Types text into an element
    async fn type_text(
        &self,
        session_id: &str,
        options: TypeOptions,
    ) -> BrowserControlResult<BrowserTask>;

    /// Scrolls the page
    async fn scroll(
        &self,
        session_id: &str,
        x: i32,
        y: i32,
    ) -> BrowserControlResult<BrowserTask>;

    /// Takes a screenshot
    async fn screenshot(
        &self,
        session_id: &str,
        options: ScreenshotOptions,
    ) -> BrowserControlResult<Vec<u8>>;

    /// Waits for a condition
    async fn wait_for(
        &self,
        session_id: &str,
        selector: &str,
        timeout_ms: u64,
    ) -> BrowserControlResult<BrowserTask>;
}
