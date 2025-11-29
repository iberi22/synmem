//! ChromiumDriver implementation using chromiumoxide

use async_trait::async_trait;
use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

use synmem_core::domain::entities::{Cookie, Session, SameSite, StorageEntry};
use synmem_core::ports::outbound::BrowserDriverPort;

use super::error::ChromiumError;
use super::session_manager::SessionManager;

/// ChromiumDriver provides browser automation using chromiumoxide
///
/// This driver implements the `BrowserDriverPort` trait and provides
/// all navigation, interaction, and session management capabilities.
pub struct ChromiumDriver {
    browser: Arc<Browser>,
    page: Arc<RwLock<Option<Page>>>,
    session_manager: SessionManager,
}

impl ChromiumDriver {
    /// Create a new ChromiumDriver with default configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the browser fails to launch
    pub async fn new() -> Result<Self, ChromiumError> {
        Self::with_config(BrowserConfig::builder().build().map_err(|e| {
            ChromiumError::LaunchError(format!("Invalid browser config: {}", e))
        })?)
        .await
    }

    /// Create a new ChromiumDriver with headless mode
    ///
    /// # Errors
    ///
    /// Returns an error if the browser fails to launch
    pub async fn headless() -> Result<Self, ChromiumError> {
        let config = BrowserConfig::builder()
            .with_head()
            .build()
            .map_err(|e| ChromiumError::LaunchError(format!("Invalid browser config: {}", e)))?;
        Self::with_config(config).await
    }

    /// Create a new ChromiumDriver with custom configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the browser fails to launch
    pub async fn with_config(config: BrowserConfig) -> Result<Self, ChromiumError> {
        info!("Launching browser with chromiumoxide");

        let (browser, mut handler) = Browser::launch(config)
            .await
            .map_err(|e| ChromiumError::LaunchError(e.to_string()))?;

        // Spawn handler to process browser events
        tokio::spawn(async move {
            while handler.next().await.is_some() {}
        });

        let browser = Arc::new(browser);
        let page = Arc::new(RwLock::new(None));
        let session_manager = SessionManager::new();

        info!("Browser launched successfully");

        Ok(Self {
            browser,
            page,
            session_manager,
        })
    }

    /// Get or create the active page
    async fn get_or_create_page(&self) -> Result<Page, ChromiumError> {
        let mut page_guard = self.page.write().await;

        if let Some(ref page) = *page_guard {
            return Ok(page.clone());
        }

        debug!("Creating new page");
        let new_page = self
            .browser
            .new_page("about:blank")
            .await
            .map_err(|e| ChromiumError::ConnectionError(e.to_string()))?;

        *page_guard = Some(new_page.clone());
        Ok(new_page)
    }

    /// Convert chromiumoxide cookie to domain cookie
    fn convert_cookie(cdp_cookie: &chromiumoxide::cdp::browser_protocol::network::Cookie) -> Cookie {
        Cookie {
            name: cdp_cookie.name.clone(),
            value: cdp_cookie.value.clone(),
            domain: cdp_cookie.domain.clone(),
            path: cdp_cookie.path.clone(),
            secure: cdp_cookie.secure,
            http_only: cdp_cookie.http_only,
            expires: if cdp_cookie.expires > 0.0 {
                Some(cdp_cookie.expires as i64)
            } else {
                None
            },
            same_site: match cdp_cookie.same_site {
                Some(chromiumoxide::cdp::browser_protocol::network::CookieSameSite::Strict) => SameSite::Strict,
                Some(chromiumoxide::cdp::browser_protocol::network::CookieSameSite::Lax) => SameSite::Lax,
                _ => SameSite::None,
            },
        }
    }
}

#[async_trait]
impl BrowserDriverPort for ChromiumDriver {
    type Error = ChromiumError;

    // === Navigation ===

    #[instrument(skip(self))]
    async fn goto(&self, url: &str) -> Result<(), Self::Error> {
        info!(url = %url, "Navigating to URL");
        let page = self.get_or_create_page().await?;
        page.goto(url)
            .await
            .map_err(|e| ChromiumError::NavigationError(e.to_string()))?;
        debug!("Navigation completed");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn back(&self) -> Result<(), Self::Error> {
        debug!("Going back in history");
        let page = self.get_or_create_page().await?;
        page.evaluate("window.history.back()")
            .await
            .map_err(|e| ChromiumError::NavigationError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn forward(&self) -> Result<(), Self::Error> {
        debug!("Going forward in history");
        let page = self.get_or_create_page().await?;
        page.evaluate("window.history.forward()")
            .await
            .map_err(|e| ChromiumError::NavigationError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn refresh(&self) -> Result<(), Self::Error> {
        debug!("Refreshing page");
        let page = self.get_or_create_page().await?;
        page.reload()
            .await
            .map_err(|e| ChromiumError::NavigationError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn current_url(&self) -> Result<String, Self::Error> {
        let page = self.get_or_create_page().await?;
        let url = page
            .url()
            .await
            .map_err(|e| ChromiumError::NavigationError(e.to_string()))?
            .map(|u| u.to_string())
            .unwrap_or_else(|| "about:blank".to_string());
        debug!(url = %url, "Current URL");
        Ok(url)
    }

    // === Element Interaction ===

    #[instrument(skip(self))]
    async fn click(&self, selector: &str) -> Result<(), Self::Error> {
        info!(selector = %selector, "Clicking element");
        let page = self.get_or_create_page().await?;
        let element = page
            .find_element(selector)
            .await
            .map_err(|_| ChromiumError::ElementNotFound {
                selector: selector.to_string(),
            })?;
        element
            .click()
            .await
            .map_err(|e| ChromiumError::InteractionError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn type_text(&self, selector: &str, text: &str) -> Result<(), Self::Error> {
        info!(selector = %selector, "Typing text into element");
        let page = self.get_or_create_page().await?;
        let element = page
            .find_element(selector)
            .await
            .map_err(|_| ChromiumError::ElementNotFound {
                selector: selector.to_string(),
            })?;
        element
            .click()
            .await
            .map_err(|e| ChromiumError::InteractionError(e.to_string()))?;
        element
            .type_str(text)
            .await
            .map_err(|e| ChromiumError::InteractionError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn select(&self, selector: &str, value: &str) -> Result<(), Self::Error> {
        info!(selector = %selector, value = %value, "Selecting option");
        let page = self.get_or_create_page().await?;

        // Properly escape strings for JavaScript to prevent injection
        let escaped_selector = escape_js_string(selector);
        let escaped_value = escape_js_string(value);

        // Use JavaScript to select the option
        let script = format!(
            r#"
            (function() {{
                const select = document.querySelector('{}');
                if (!select) throw new Error('Select element not found');
                select.value = '{}';
                select.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }})()
            "#,
            escaped_selector,
            escaped_value
        );

        page.evaluate(script)
            .await
            .map_err(|e| ChromiumError::InteractionError(e.to_string()))?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn wait_for_element(&self, selector: &str, timeout_ms: u64) -> Result<(), Self::Error> {
        debug!(selector = %selector, timeout_ms = %timeout_ms, "Waiting for element");
        let page = self.get_or_create_page().await?;

        let timeout = std::time::Duration::from_millis(timeout_ms);
        let start = std::time::Instant::now();

        loop {
            if page.find_element(selector).await.is_ok() {
                return Ok(());
            }

            if start.elapsed() > timeout {
                return Err(ChromiumError::Timeout {
                    timeout_ms,
                    description: format!("element '{}'", selector),
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    // === Page Operations ===

    #[instrument(skip(self))]
    async fn screenshot(&self) -> Result<Vec<u8>, Self::Error> {
        debug!("Taking screenshot");
        let page = self.get_or_create_page().await?;
        let screenshot = page
            .screenshot(
                chromiumoxide::page::ScreenshotParams::builder()
                    .full_page(true)
                    .build(),
            )
            .await
            .map_err(|e| ChromiumError::ScreenshotError(e.to_string()))?;
        Ok(screenshot)
    }

    #[instrument(skip(self))]
    async fn get_html(&self) -> Result<String, Self::Error> {
        debug!("Getting HTML content");
        let page = self.get_or_create_page().await?;
        let html = page
            .content()
            .await
            .map_err(|e| ChromiumError::JsError(e.to_string()))?;
        Ok(html)
    }

    #[instrument(skip(self))]
    async fn evaluate_js(&self, script: &str) -> Result<String, Self::Error> {
        debug!("Evaluating JavaScript");
        let page = self.get_or_create_page().await?;
        let result: serde_json::Value = page
            .evaluate(script)
            .await
            .map_err(|e| ChromiumError::JsError(e.to_string()))?
            .into_value()
            .map_err(|e| ChromiumError::JsError(e.to_string()))?;

        match result {
            serde_json::Value::String(s) => Ok(s),
            other => Ok(other.to_string()),
        }
    }

    // === Session Management ===

    #[instrument(skip(self))]
    async fn get_cookies(&self) -> Result<Vec<Cookie>, Self::Error> {
        debug!("Getting cookies");
        let page = self.get_or_create_page().await?;
        let cookies = page
            .get_cookies()
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?;

        Ok(cookies.iter().map(Self::convert_cookie).collect())
    }

    #[instrument(skip(self))]
    async fn set_cookies(&self, cookies: &[Cookie]) -> Result<(), Self::Error> {
        debug!(count = %cookies.len(), "Setting cookies");
        let page = self.get_or_create_page().await?;

        for cookie in cookies {
            let cdp_cookie = chromiumoxide::cdp::browser_protocol::network::CookieParam::builder()
                .name(&cookie.name)
                .value(&cookie.value)
                .domain(&cookie.domain)
                .path(&cookie.path)
                .secure(cookie.secure)
                .http_only(cookie.http_only)
                .build()
                .map_err(|e| ChromiumError::SessionError(e))?;

            page.set_cookie(cdp_cookie)
                .await
                .map_err(|e| ChromiumError::SessionError(e.to_string()))?;
        }
        Ok(())
    }

    #[instrument(skip(self))]
    async fn save_session(&self) -> Result<Session, Self::Error> {
        debug!("Saving session");
        let cookies = self.get_cookies().await?;
        let page = self.get_or_create_page().await?;
        let url = self.current_url().await.unwrap_or_default();

        // Get local storage
        let local_storage_script = r#"
            JSON.stringify(Object.entries(localStorage))
        "#;
        let local_storage_json: String = page
            .evaluate(local_storage_script)
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?
            .into_value()
            .unwrap_or_else(|_| "[]".to_string());

        let local_storage_entries: Vec<(String, String)> =
            serde_json::from_str(&local_storage_json).unwrap_or_default();
        let local_storage: Vec<StorageEntry> = local_storage_entries
            .into_iter()
            .map(|(key, value)| StorageEntry {
                key,
                value,
                origin: url.clone(),
            })
            .collect();

        // Get session storage
        let session_storage_script = r#"
            JSON.stringify(Object.entries(sessionStorage))
        "#;
        let session_storage_json: String = page
            .evaluate(session_storage_script)
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?
            .into_value()
            .unwrap_or_else(|_| "[]".to_string());

        let session_storage_entries: Vec<(String, String)> =
            serde_json::from_str(&session_storage_json).unwrap_or_default();
        let session_storage: Vec<StorageEntry> = session_storage_entries
            .into_iter()
            .map(|(key, value)| StorageEntry {
                key,
                value,
                origin: url.clone(),
            })
            .collect();

        let session_id = uuid_v4();
        let session = Session {
            id: session_id,
            name: format!("Session at {}", url),
            cookies,
            local_storage,
            session_storage,
        };

        self.session_manager.save(&session);
        Ok(session)
    }

    #[instrument(skip(self, session))]
    async fn load_session(&self, session: &Session) -> Result<(), Self::Error> {
        debug!(session_id = %session.id, "Loading session");

        // Set cookies
        self.set_cookies(&session.cookies).await?;

        let page = self.get_or_create_page().await?;

        // Restore local storage
        for entry in &session.local_storage {
            let escaped_key = escape_js_string(&entry.key);
            let escaped_value = escape_js_string(&entry.value);
            let script = format!(
                "localStorage.setItem('{}', '{}')",
                escaped_key,
                escaped_value
            );
            let _ = page.evaluate(script).await;
        }

        // Restore session storage
        for entry in &session.session_storage {
            let escaped_key = escape_js_string(&entry.key);
            let escaped_value = escape_js_string(&entry.value);
            let script = format!(
                "sessionStorage.setItem('{}', '{}')",
                escaped_key,
                escaped_value
            );
            let _ = page.evaluate(script).await;
        }

        Ok(())
    }

    #[instrument(skip(self))]
    async fn clear_session(&self) -> Result<(), Self::Error> {
        debug!("Clearing session");
        let page = self.get_or_create_page().await?;

        // Clear cookies
        page.delete_cookies(vec![])
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?;

        // Clear local storage
        page.evaluate("localStorage.clear()")
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?;

        // Clear session storage
        page.evaluate("sessionStorage.clear()")
            .await
            .map_err(|e| ChromiumError::SessionError(e.to_string()))?;

        Ok(())
    }

    // === Lifecycle ===

    #[instrument(skip(self))]
    async fn close(&self) -> Result<(), Self::Error> {
        info!("Closing browser");

        // Close the page
        let mut page_guard = self.page.write().await;
        if let Some(page) = page_guard.take() {
            let _ = page.close().await;
        }

        Ok(())
    }
}

/// Generate a simple UUID v4
fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:032x}", timestamp)
}

/// Escape a string for safe use in JavaScript string literals.
/// This prevents XSS attacks by properly escaping control characters,
/// quotes, backslashes, and other potentially dangerous characters.
fn escape_js_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\'' => result.push_str("\\'"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '<' => result.push_str("\\u003c"),
            '>' => result.push_str("\\u003e"),
            '/' => result.push_str("\\/"),
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generation() {
        let id1 = uuid_v4();
        let id2 = uuid_v4();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 32);
    }

    #[test]
    fn test_escape_js_string() {
        // Test basic escaping
        assert_eq!(escape_js_string("hello"), "hello");
        assert_eq!(escape_js_string("it's"), "it\\'s");
        assert_eq!(escape_js_string("say \"hello\""), "say \\\"hello\\\"");
        assert_eq!(escape_js_string("back\\slash"), "back\\\\slash");
        
        // Test control characters
        assert_eq!(escape_js_string("line\nbreak"), "line\\nbreak");
        assert_eq!(escape_js_string("tab\there"), "tab\\there");
        
        // Test XSS prevention
        assert_eq!(escape_js_string("<script>"), "\\u003cscript\\u003e");
        assert_eq!(escape_js_string("</script>"), "\\u003c\\/script\\u003e");
        
        // Test Unicode line separators
        assert_eq!(escape_js_string("test\u{2028}sep"), "test\\u2028sep");
        assert_eq!(escape_js_string("test\u{2029}sep"), "test\\u2029sep");
    }
}
