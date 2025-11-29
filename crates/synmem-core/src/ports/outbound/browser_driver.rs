//! Browser driver outbound port.

use crate::domain::entities::{ScrollDirection, WaitCondition};
use async_trait::async_trait;
use mockall::automock;

/// Error type for browser driver operations.
pub type DriverError = Box<dyn std::error::Error + Send + Sync>;

/// Outbound port for browser automation.
/// This is the interface that browser implementations (like chromiumoxide) must implement.
#[automock]
#[async_trait]
pub trait BrowserDriver: Send + Sync {
    /// Navigate to a URL with the specified wait condition.
    async fn navigate(&self, url: String, wait_for: WaitCondition) -> Result<(), DriverError>;

    /// Click an element by selector or text content.
    async fn click(&self, selector: Option<String>, text: Option<String>) -> Result<(), DriverError>;

    /// Type text into an element identified by selector.
    async fn type_text(&self, selector: String, text: String) -> Result<(), DriverError>;

    /// Scroll the page in the specified direction.
    async fn scroll(
        &self,
        direction: ScrollDirection,
        amount: Option<i32>,
    ) -> Result<(), DriverError>;

    /// Take a screenshot of the page.
    async fn screenshot(
        &self,
        full_page: bool,
        path: Option<String>,
    ) -> Result<Vec<u8>, DriverError>;

    /// Wait for an element to appear on the page.
    async fn wait_for_selector(&self, selector: String, timeout_ms: u64)
        -> Result<(), DriverError>;
}
