//! Browser control inbound port

use async_trait::async_trait;
use std::error::Error;

/// Port for controlling browser operations
#[async_trait]
pub trait BrowserControlPort: Send + Sync {
    /// Error type for this port
    type Error: Error + Send + Sync + 'static;

    /// Navigate to a URL
    async fn navigate_to(&self, url: &str) -> Result<(), Self::Error>;

    /// Go back in history
    async fn go_back(&self) -> Result<(), Self::Error>;

    /// Go forward in history
    async fn go_forward(&self) -> Result<(), Self::Error>;

    /// Refresh the page
    async fn refresh(&self) -> Result<(), Self::Error>;

    /// Click on an element
    async fn click(&self, selector: &str) -> Result<(), Self::Error>;

    /// Type text into an element
    async fn type_text(&self, selector: &str, text: &str) -> Result<(), Self::Error>;

    /// Take a screenshot
    async fn screenshot(&self) -> Result<Vec<u8>, Self::Error>;
}
