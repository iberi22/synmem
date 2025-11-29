//! Navigation service for browser control.

use crate::domain::entities::{BrowserTask, ScrollDirection, TaskType, WaitCondition};
use crate::ports::outbound::BrowserDriver;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur during navigation.
#[derive(Debug, Error)]
pub enum NavigationError {
    #[error("Navigation failed: {0}")]
    NavigationFailed(String),
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    #[error("Timeout waiting for element: {0}")]
    Timeout(String),
    #[error("Browser driver error: {0}")]
    DriverError(String),
}

/// Navigation service providing browser control operations.
pub struct NavigationService<D: BrowserDriver> {
    driver: Arc<D>,
}

impl<D: BrowserDriver> NavigationService<D> {
    /// Create a new navigation service with the given driver.
    pub fn new(driver: Arc<D>) -> Self {
        Self { driver }
    }

    /// Navigate to a URL.
    pub async fn navigate_to(
        &self,
        url: String,
        wait_for: WaitCondition,
    ) -> Result<(), NavigationError> {
        self.driver
            .navigate(url, wait_for)
            .await
            .map_err(|e| NavigationError::NavigationFailed(e.to_string()))
    }

    /// Click an element by selector or text.
    pub async fn click(
        &self,
        selector: Option<String>,
        text: Option<String>,
    ) -> Result<(), NavigationError> {
        self.driver
            .click(selector, text)
            .await
            .map_err(|e| NavigationError::ElementNotFound(e.to_string()))
    }

    /// Type text into an element.
    pub async fn type_text(&self, selector: String, text: String) -> Result<(), NavigationError> {
        self.driver
            .type_text(selector, text)
            .await
            .map_err(|e| NavigationError::ElementNotFound(e.to_string()))
    }

    /// Scroll the page.
    pub async fn scroll(
        &self,
        direction: ScrollDirection,
        amount: Option<i32>,
    ) -> Result<(), NavigationError> {
        self.driver
            .scroll(direction, amount)
            .await
            .map_err(|e| NavigationError::DriverError(e.to_string()))
    }

    /// Take a screenshot.
    pub async fn screenshot(
        &self,
        full_page: bool,
        path: Option<String>,
    ) -> Result<Vec<u8>, NavigationError> {
        self.driver
            .screenshot(full_page, path)
            .await
            .map_err(|e| NavigationError::DriverError(e.to_string()))
    }

    /// Wait for an element to appear.
    pub async fn wait_for(&self, selector: String, timeout_ms: u64) -> Result<(), NavigationError> {
        self.driver
            .wait_for_selector(selector, timeout_ms)
            .await
            .map_err(|e| NavigationError::Timeout(e.to_string()))
    }

    /// Execute a browser task.
    pub async fn execute_task(&self, task: &mut BrowserTask) -> Result<(), NavigationError> {
        task.status = crate::domain::entities::TaskStatus::Running;

        let result = match &task.task_type {
            TaskType::Navigate { url, wait_for } => {
                self.navigate_to(url.clone(), wait_for.clone()).await
            }
            TaskType::Click { selector, text } => {
                self.click(selector.clone(), text.clone()).await
            }
            TaskType::TypeText { selector, text } => self.type_text(selector.clone(), text.clone()).await,
            TaskType::Scroll { direction, amount } => {
                self.scroll(direction.clone(), *amount).await
            }
            TaskType::Screenshot { full_page, path } => {
                self.screenshot(*full_page, path.clone()).await.map(|_| ())
            }
            TaskType::WaitFor {
                selector,
                timeout_ms,
            } => self.wait_for(selector.clone(), *timeout_ms).await,
        };

        match result {
            Ok(_) => {
                task.complete();
                Ok(())
            }
            Err(e) => {
                task.fail(e.to_string());
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::outbound::MockBrowserDriver;

    #[tokio::test]
    async fn test_navigate_to() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_navigate()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.navigate_to("https://example.com".to_string(), WaitCondition::Load).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_click_by_selector() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_click()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.click(Some("#button".to_string()), None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_type_text() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_type_text()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.type_text("#input".to_string(), "hello world".to_string()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scroll() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_scroll()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.scroll(ScrollDirection::Down, Some(500)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_screenshot() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_screenshot()
            .returning(|_, _| Ok(vec![1, 2, 3]));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.screenshot(true, None).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_wait_for() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_wait_for_selector()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let result = service.wait_for("#element".to_string(), 5000).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_task_navigate() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_navigate()
            .returning(|_, _| Ok(()));

        let service = NavigationService::new(Arc::new(mock_driver));
        let mut task = BrowserTask::new(
            "task-1".to_string(),
            TaskType::Navigate {
                url: "https://example.com".to_string(),
                wait_for: WaitCondition::Load,
            },
        );

        let result = service.execute_task(&mut task).await;

        assert!(result.is_ok());
        assert_eq!(task.status, crate::domain::entities::TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_execute_task_failure() {
        let mut mock_driver = MockBrowserDriver::new();
        mock_driver
            .expect_navigate()
            .returning(|_, _| Err(Box::new(std::io::Error::other("Connection refused"))));

        let service = NavigationService::new(Arc::new(mock_driver));
        let mut task = BrowserTask::new(
            "task-1".to_string(),
            TaskType::Navigate {
                url: "https://example.com".to_string(),
                wait_for: WaitCondition::Load,
            },
        );

        let result = service.execute_task(&mut task).await;

        assert!(result.is_err());
        assert_eq!(task.status, crate::domain::entities::TaskStatus::Failed);
        assert!(task.error.is_some());
    }
}
