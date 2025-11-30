//! Navigation service for browser control

use crate::ports::outbound::BrowserDriverPort;
use std::sync::Arc;

/// Service for handling browser navigation operations
pub struct NavigationService<D: BrowserDriverPort> {
    driver: Arc<D>,
}

impl<D: BrowserDriverPort> NavigationService<D> {
    /// Create a new navigation service with the given driver
    pub fn new(driver: Arc<D>) -> Self {
        Self { driver }
    }

    /// Navigate to a URL
    pub async fn goto(&self, url: &str) -> Result<(), D::Error> {
        self.driver.goto(url).await
    }

    /// Go back in browser history
    pub async fn back(&self) -> Result<(), D::Error> {
        self.driver.back().await
    }

    /// Go forward in browser history
    pub async fn forward(&self) -> Result<(), D::Error> {
        self.driver.forward().await
    }

    /// Refresh the current page
    pub async fn refresh(&self) -> Result<(), D::Error> {
        self.driver.refresh().await
    }

    /// Get the current URL
    pub async fn current_url(&self) -> Result<String, D::Error> {
        self.driver.current_url().await
    }
}
