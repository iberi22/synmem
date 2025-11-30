//! Browser driver outbound port
//!
//! This port defines the interface that browser drivers must implement
//! to provide browser automation capabilities.

use crate::domain::entities::{BrowserState, SimpleCookie};
use async_trait::async_trait;
use std::error::Error;

/// Port for browser driver implementations
///
/// This is the core interface for browser automation. Implementations
/// can use different browser engines (chromiumoxide, headless_chrome, etc.)
#[async_trait]
pub trait BrowserDriverPort: Send + Sync {
    /// Error type for this driver
    type Error: Error + Send + Sync + 'static;

    // === Navigation ===

    /// Navigate to a URL
    async fn goto(&self, url: &str) -> Result<(), Self::Error>;

    /// Go back in browser history
    async fn back(&self) -> Result<(), Self::Error>;

    /// Go forward in browser history
    async fn forward(&self) -> Result<(), Self::Error>;

    /// Refresh the current page
    async fn refresh(&self) -> Result<(), Self::Error>;

    /// Get the current URL
    async fn current_url(&self) -> Result<String, Self::Error>;

    // === Element Interaction ===

    /// Click on an element by CSS selector
    async fn click(&self, selector: &str) -> Result<(), Self::Error>;

    /// Type text into an element
    async fn type_text(&self, selector: &str, text: &str) -> Result<(), Self::Error>;

    /// Select an option from a dropdown by value
    async fn select(&self, selector: &str, value: &str) -> Result<(), Self::Error>;

    /// Wait for an element to be present
    async fn wait_for_element(&self, selector: &str, timeout_ms: u64) -> Result<(), Self::Error>;

    // === Page Operations ===

    /// Take a screenshot of the current page
    async fn screenshot(&self) -> Result<Vec<u8>, Self::Error>;

    /// Get the HTML content of the page
    async fn get_html(&self) -> Result<String, Self::Error>;

    /// Evaluate JavaScript and return the result as a string
    async fn evaluate_js(&self, script: &str) -> Result<String, Self::Error>;

    // === Session Management ===

    /// Get all cookies
    async fn get_cookies(&self) -> Result<Vec<SimpleCookie>, Self::Error>;

    /// Set cookies
    async fn set_cookies(&self, cookies: &[SimpleCookie]) -> Result<(), Self::Error>;

    /// Save the current browser state (cookies, storage)
    async fn save_session(&self) -> Result<BrowserState, Self::Error>;

    /// Load a saved browser state
    async fn load_session(&self, state: &BrowserState) -> Result<(), Self::Error>;

    /// Clear all session data (cookies, storage)
    async fn clear_session(&self) -> Result<(), Self::Error>;

    // === Lifecycle ===

    /// Close the browser
    async fn close(&self) -> Result<(), Self::Error>;
}
