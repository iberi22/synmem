//! # Browser Control Port
//!
//! Inbound port for browser navigation and interaction.

use async_trait::async_trait;
use thiserror::Error;
use url::Url;

/// Errors that can occur during browser control operations.
#[derive(Debug, Error)]
pub enum BrowserControlError {
    #[error("navigation failed: {0}")]
    NavigationFailed(String),

    #[error("element not found: {0}")]
    ElementNotFound(String),

    #[error("click failed: {0}")]
    ClickFailed(String),

    #[error("type failed: {0}")]
    TypeFailed(String),

    #[error("screenshot failed: {0}")]
    ScreenshotFailed(String),

    #[error("timeout: {0}")]
    Timeout(String),

    #[error("browser not connected")]
    NotConnected,
}

/// Result type for browser control operations.
pub type BrowserControlResult<T> = Result<T, BrowserControlError>;

/// Screenshot data with format information.
#[derive(Debug, Clone)]
pub struct Screenshot {
    /// Raw image data (PNG format).
    pub data: Vec<u8>,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

/// Options for taking screenshots.
#[derive(Debug, Clone, Default)]
pub struct ScreenshotOptions {
    /// Capture the full page (scroll capture).
    pub full_page: bool,
    /// Specific element selector to capture.
    pub selector: Option<String>,
}

/// Options for navigation.
#[derive(Debug, Clone, Default)]
pub struct NavigateOptions {
    /// Wait for network idle before returning.
    pub wait_for_network_idle: bool,
    /// Timeout in milliseconds.
    pub timeout_ms: Option<u64>,
}

/// Inbound port for browser control operations.
///
/// This port defines the interface for navigating, clicking, typing,
/// and capturing screenshots in a browser.
#[async_trait]
pub trait BrowserControlPort: Send + Sync {
    /// Navigate to a URL.
    async fn navigate(&self, url: &Url, options: NavigateOptions) -> BrowserControlResult<()>;

    /// Click on an element identified by a CSS selector.
    async fn click(&self, selector: &str) -> BrowserControlResult<()>;

    /// Type text into an element identified by a CSS selector.
    async fn type_text(&self, selector: &str, text: &str) -> BrowserControlResult<()>;

    /// Take a screenshot of the current page.
    async fn screenshot(&self, options: ScreenshotOptions) -> BrowserControlResult<Screenshot>;
}
