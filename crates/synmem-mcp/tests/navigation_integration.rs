//! Integration tests for navigation MCP tools with mock browser.

use async_trait::async_trait;
use std::sync::Arc;
use synmem_core::domain::entities::{ScrollDirection, WaitCondition};
use synmem_core::ports::inbound::BrowserControlPort;
use synmem_mcp::tools::{
    execute_click, execute_navigate_to, execute_screenshot, execute_scroll, execute_type_text,
    execute_wait_for, ClickInput, NavigateToInput, ScreenshotInput, ScrollInput, TypeTextInput,
    WaitForInput,
};

/// Mock browser implementation for integration testing.
struct MockBrowser {
    /// Tracks navigated URLs.
    pub navigated_urls: std::sync::Mutex<Vec<String>>,
    /// Tracks clicked elements.
    pub clicked_elements: std::sync::Mutex<Vec<String>>,
    /// Tracks typed text.
    pub typed_text: std::sync::Mutex<Vec<(String, String)>>,
    /// Tracks scroll actions.
    pub scroll_actions: std::sync::Mutex<Vec<(String, Option<i32>)>>,
    /// Tracks screenshot requests.
    pub screenshot_requests: std::sync::Mutex<Vec<(bool, Option<String>)>>,
    /// Tracks wait_for requests.
    pub wait_for_requests: std::sync::Mutex<Vec<(String, u64)>>,
}

impl MockBrowser {
    fn new() -> Self {
        Self {
            navigated_urls: std::sync::Mutex::new(Vec::new()),
            clicked_elements: std::sync::Mutex::new(Vec::new()),
            typed_text: std::sync::Mutex::new(Vec::new()),
            scroll_actions: std::sync::Mutex::new(Vec::new()),
            screenshot_requests: std::sync::Mutex::new(Vec::new()),
            wait_for_requests: std::sync::Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl BrowserControlPort for MockBrowser {
    async fn navigate_to(&self, url: String, _wait_for: WaitCondition) -> Result<(), String> {
        self.navigated_urls.lock().unwrap().push(url);
        Ok(())
    }

    async fn click(&self, selector: Option<String>, text: Option<String>) -> Result<(), String> {
        let target = selector.or(text).unwrap_or_default();
        self.clicked_elements.lock().unwrap().push(target);
        Ok(())
    }

    async fn type_text(&self, selector: String, text: String) -> Result<(), String> {
        self.typed_text.lock().unwrap().push((selector, text));
        Ok(())
    }

    async fn scroll(&self, direction: ScrollDirection, amount: Option<i32>) -> Result<(), String> {
        let dir_str = match direction {
            ScrollDirection::Up => "up",
            ScrollDirection::Down => "down",
            ScrollDirection::Left => "left",
            ScrollDirection::Right => "right",
        };
        self.scroll_actions
            .lock()
            .unwrap()
            .push((dir_str.to_string(), amount));
        Ok(())
    }

    async fn screenshot(&self, full_page: bool, path: Option<String>) -> Result<Vec<u8>, String> {
        self.screenshot_requests
            .lock()
            .unwrap()
            .push((full_page, path));
        // Return fake PNG data
        Ok(vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
    }

    async fn wait_for(&self, selector: String, timeout_ms: u64) -> Result<(), String> {
        self.wait_for_requests
            .lock()
            .unwrap()
            .push((selector, timeout_ms));
        Ok(())
    }
}

#[tokio::test]
async fn test_navigation_workflow() {
    let browser = Arc::new(MockBrowser::new());

    // Step 1: Navigate to a page
    let nav_result = execute_navigate_to(
        browser.clone(),
        NavigateToInput {
            url: "https://example.com".to_string(),
            wait_for: Some("load".to_string()),
        },
    )
    .await;

    assert!(nav_result.success);
    assert_eq!(nav_result.url, "https://example.com");

    // Step 2: Wait for an element to appear
    let wait_result = execute_wait_for(
        browser.clone(),
        WaitForInput {
            selector: "#login-form".to_string(),
            timeout: Some(5000),
        },
    )
    .await;

    assert!(wait_result.success);

    // Step 3: Type text into a form field
    let type_result = execute_type_text(
        browser.clone(),
        TypeTextInput {
            selector: "#username".to_string(),
            text: "testuser".to_string(),
        },
    )
    .await;

    assert!(type_result.success);

    // Step 4: Click a button
    let click_result = execute_click(
        browser.clone(),
        ClickInput {
            selector: Some("#submit-btn".to_string()),
            text: None,
        },
    )
    .await;

    assert!(click_result.success);

    // Step 5: Scroll the page
    let scroll_result = execute_scroll(
        browser.clone(),
        ScrollInput {
            direction: Some("down".to_string()),
            amount: Some(300),
        },
    )
    .await;

    assert!(scroll_result.success);

    // Step 6: Take a screenshot
    let screenshot_result = execute_screenshot(
        browser.clone(),
        ScreenshotInput {
            full_page: Some(true),
            path: None,
        },
    )
    .await;

    assert!(screenshot_result.success);
    assert!(screenshot_result.data.is_some());

    // Verify all actions were recorded
    let urls = browser.navigated_urls.lock().unwrap();
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0], "https://example.com");

    let wait_requests = browser.wait_for_requests.lock().unwrap();
    assert_eq!(wait_requests.len(), 1);
    assert_eq!(wait_requests[0].0, "#login-form");

    let typed = browser.typed_text.lock().unwrap();
    assert_eq!(typed.len(), 1);
    assert_eq!(typed[0], ("#username".to_string(), "testuser".to_string()));

    let clicked = browser.clicked_elements.lock().unwrap();
    assert_eq!(clicked.len(), 1);
    assert_eq!(clicked[0], "#submit-btn");

    let scrolls = browser.scroll_actions.lock().unwrap();
    assert_eq!(scrolls.len(), 1);
    assert_eq!(scrolls[0], ("down".to_string(), Some(300)));

    let screenshots = browser.screenshot_requests.lock().unwrap();
    assert_eq!(screenshots.len(), 1);
    assert_eq!(screenshots[0].0, true);
}

#[tokio::test]
async fn test_click_by_text() {
    let browser = Arc::new(MockBrowser::new());

    let click_result = execute_click(
        browser.clone(),
        ClickInput {
            selector: None,
            text: Some("Sign In".to_string()),
        },
    )
    .await;

    assert!(click_result.success);

    let clicked = browser.clicked_elements.lock().unwrap();
    assert_eq!(clicked.len(), 1);
    assert_eq!(clicked[0], "Sign In");
}

#[tokio::test]
async fn test_scroll_directions() {
    let browser = Arc::new(MockBrowser::new());

    // Test all directions
    for direction in &["up", "down", "left", "right"] {
        let scroll_result = execute_scroll(
            browser.clone(),
            ScrollInput {
                direction: Some(direction.to_string()),
                amount: Some(100),
            },
        )
        .await;

        assert!(scroll_result.success);
    }

    let scrolls = browser.scroll_actions.lock().unwrap();
    assert_eq!(scrolls.len(), 4);
    assert_eq!(scrolls[0].0, "up");
    assert_eq!(scrolls[1].0, "down");
    assert_eq!(scrolls[2].0, "left");
    assert_eq!(scrolls[3].0, "right");
}

#[tokio::test]
async fn test_tool_definitions() {
    use synmem_mcp::tools::all_navigation_tools;

    let tools = all_navigation_tools();

    // Verify all required tools are present
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    assert!(tool_names.contains(&"navigate_to"));
    assert!(tool_names.contains(&"click"));
    assert!(tool_names.contains(&"type_text"));
    assert!(tool_names.contains(&"scroll"));
    assert!(tool_names.contains(&"screenshot"));
    assert!(tool_names.contains(&"wait_for"));

    // Verify each tool has a description
    for tool in &tools {
        assert!(!tool.description.is_empty());
    }
}
