//! # Browser Driver Port
//!
//! Outbound port for actual browser interaction.

use async_trait::async_trait;
use thiserror::Error;
use url::Url;

/// Errors that can occur during browser driver operations.
#[derive(Debug, Error)]
pub enum BrowserDriverError {
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("navigation failed: {0}")]
    NavigationFailed(String),

    #[error("element not found: {0}")]
    ElementNotFound(String),

    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("screenshot failed: {0}")]
    ScreenshotFailed(String),

    #[error("timeout: {0}")]
    Timeout(String),

    #[error("browser closed")]
    BrowserClosed,
}

/// Result type for browser driver operations.
pub type BrowserDriverResult<T> = Result<T, BrowserDriverError>;

/// Browser viewport dimensions.
#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

/// Options for launching a browser instance.
#[derive(Debug, Clone)]
pub struct LaunchOptions {
    /// Run browser in headless mode.
    pub headless: bool,
    /// Path to browser executable.
    pub executable_path: Option<String>,
    /// User data directory for profiles.
    pub user_data_dir: Option<String>,
    /// Initial viewport size.
    pub viewport: Option<Viewport>,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            headless: true,
            executable_path: None,
            user_data_dir: None,
            viewport: Some(Viewport {
                width: 1280,
                height: 720,
            }),
        }
    }
}

/// Raw page information from the browser.
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// Current page URL.
    pub url: Url,
    /// Page title.
    pub title: Option<String>,
}

/// Options for waiting operations.
#[derive(Debug, Clone, Default)]
pub struct WaitOptions {
    /// Timeout in milliseconds.
    pub timeout_ms: Option<u64>,
    /// Polling interval in milliseconds.
    pub poll_interval_ms: Option<u64>,
}

/// Outbound port for actual browser interaction.
///
/// This port defines the low-level interface for controlling a browser
/// instance. Implementations might use chromiumoxide, Playwright, etc.
#[async_trait]
pub trait BrowserDriverPort: Send + Sync {
    /// Launch a new browser instance.
    async fn launch(&self, options: LaunchOptions) -> BrowserDriverResult<()>;

    /// Close the browser instance.
    async fn close(&self) -> BrowserDriverResult<()>;

    /// Navigate to a URL.
    async fn navigate_to(&self, url: &Url) -> BrowserDriverResult<PageInfo>;

    /// Execute JavaScript in the page context.
    async fn execute_js(&self, script: &str) -> BrowserDriverResult<String>;

    /// Click on an element by selector.
    async fn click_element(&self, selector: &str) -> BrowserDriverResult<()>;

    /// Type text into an element by selector.
    async fn type_into_element(&self, selector: &str, text: &str) -> BrowserDriverResult<()>;

    /// Wait for an element to be present in the DOM.
    async fn wait_for_selector(
        &self,
        selector: &str,
        options: WaitOptions,
    ) -> BrowserDriverResult<()>;

    /// Take a screenshot and return PNG data.
    async fn take_screenshot(&self, full_page: bool) -> BrowserDriverResult<Vec<u8>>;

    /// Get the current page HTML.
    async fn get_html(&self) -> BrowserDriverResult<String>;

    /// Get the current page URL.
    async fn get_current_url(&self) -> BrowserDriverResult<Url>;
}
