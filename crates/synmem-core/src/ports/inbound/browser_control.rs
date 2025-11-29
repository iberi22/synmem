//! Browser control inbound port.

use crate::domain::entities::{ScrollDirection, WaitCondition};
use async_trait::async_trait;

/// Inbound port for browser control operations.
/// This is the interface that adapters (like MCP server) will use to control the browser.
#[async_trait]
pub trait BrowserControlPort: Send + Sync {
    /// Navigate to a URL.
    async fn navigate_to(&self, url: String, wait_for: WaitCondition) -> Result<(), String>;

    /// Click an element.
    async fn click(&self, selector: Option<String>, text: Option<String>) -> Result<(), String>;

    /// Type text into an element.
    async fn type_text(&self, selector: String, text: String) -> Result<(), String>;

    /// Scroll the page.
    async fn scroll(&self, direction: ScrollDirection, amount: Option<i32>) -> Result<(), String>;

    /// Take a screenshot.
    async fn screenshot(&self, full_page: bool, path: Option<String>) -> Result<Vec<u8>, String>;

    /// Wait for an element.
    async fn wait_for(&self, selector: String, timeout_ms: u64) -> Result<(), String>;
}
