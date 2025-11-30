//! Automation service for browser interactions

use crate::ports::outbound::BrowserDriverPort;
use std::sync::Arc;

/// Service for automating browser interactions
pub struct AutomationService<D: BrowserDriverPort> {
    driver: Arc<D>,
}

impl<D: BrowserDriverPort> AutomationService<D> {
    /// Create a new automation service with the given driver
    pub fn new(driver: Arc<D>) -> Self {
        Self { driver }
    }

    /// Click on an element by CSS selector
    pub async fn click(&self, selector: &str) -> Result<(), D::Error> {
        self.driver.click(selector).await
    }

    /// Type text into an element
    pub async fn type_text(&self, selector: &str, text: &str) -> Result<(), D::Error> {
        self.driver.type_text(selector, text).await
    }

    /// Select an option in a dropdown by value
    pub async fn select(&self, selector: &str, value: &str) -> Result<(), D::Error> {
        self.driver.select(selector, value).await
    }

    /// Wait for an element to be present
    pub async fn wait_for_element(&self, selector: &str, timeout_ms: u64) -> Result<(), D::Error> {
        self.driver.wait_for_element(selector, timeout_ms).await
    }
}
