//! Screenshot tool handler.

use serde::{Deserialize, Serialize};
use synmem_core::ports::inbound::BrowserControlPort;
use std::sync::Arc;

/// Input parameters for screenshot tool.
#[derive(Debug, Deserialize)]
pub struct ScreenshotInput {
    /// Capture the full scrollable page.
    #[serde(default)]
    pub full_page: Option<bool>,
    /// File path to save the screenshot.
    #[serde(default)]
    pub path: Option<String>,
}

/// Output for screenshot tool.
#[derive(Debug, Serialize)]
pub struct ScreenshotOutput {
    /// Whether screenshot was successful.
    pub success: bool,
    /// Base64-encoded screenshot data (if not saved to file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Path where screenshot was saved (if saved to file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Error message if screenshot failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Execute the screenshot tool.
pub async fn execute_screenshot(
    browser: Arc<dyn BrowserControlPort>,
    input: ScreenshotInput,
) -> ScreenshotOutput {
    let full_page = input.full_page.unwrap_or(false);

    match browser.screenshot(full_page, input.path.clone()).await {
        Ok(data) => {
            if input.path.is_some() {
                ScreenshotOutput {
                    success: true,
                    data: None,
                    path: input.path,
                    error: None,
                }
            } else {
                use std::io::Write;
                let mut encoded = Vec::new();
                {
                    let mut encoder = base64_encoder(&mut encoded);
                    encoder.write_all(&data).ok();
                }
                ScreenshotOutput {
                    success: true,
                    data: Some(String::from_utf8_lossy(&encoded).to_string()),
                    path: None,
                    error: None,
                }
            }
        }
        Err(e) => ScreenshotOutput {
            success: false,
            data: None,
            path: None,
            error: Some(e),
        },
    }
}

/// Simple base64 encoder helper.
fn base64_encoder(output: &mut Vec<u8>) -> Base64Encoder<'_> {
    Base64Encoder { output }
}

struct Base64Encoder<'a> {
    output: &'a mut Vec<u8>,
}

impl<'a> std::io::Write for Base64Encoder<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        for chunk in buf.chunks(3) {
            let b0 = chunk[0] as usize;
            let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
            let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

            self.output.push(ALPHABET[b0 >> 2]);
            self.output.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)]);

            if chunk.len() > 1 {
                self.output.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)]);
            } else {
                self.output.push(b'=');
            }

            if chunk.len() > 2 {
                self.output.push(ALPHABET[b2 & 0x3f]);
            } else {
                self.output.push(b'=');
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use synmem_core::domain::entities::{ScrollDirection, WaitCondition};

    struct MockBrowser {
        should_fail: bool,
    }

    #[async_trait]
    impl BrowserControlPort for MockBrowser {
        async fn navigate_to(&self, _url: String, _wait_for: WaitCondition) -> Result<(), String> {
            Ok(())
        }

        async fn click(&self, _selector: Option<String>, _text: Option<String>) -> Result<(), String> {
            Ok(())
        }

        async fn type_text(&self, _selector: String, _text: String) -> Result<(), String> {
            Ok(())
        }

        async fn scroll(&self, _direction: ScrollDirection, _amount: Option<i32>) -> Result<(), String> {
            Ok(())
        }

        async fn screenshot(&self, _full_page: bool, _path: Option<String>) -> Result<Vec<u8>, String> {
            if self.should_fail {
                Err("Screenshot failed".to_string())
            } else {
                Ok(vec![0x89, 0x50, 0x4E, 0x47]) // PNG magic bytes
            }
        }

        async fn wait_for(&self, _selector: String, _timeout_ms: u64) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_execute_screenshot_success() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ScreenshotInput {
            full_page: Some(true),
            path: None,
        };

        let output = execute_screenshot(browser, input).await;

        assert!(output.success);
        assert!(output.data.is_some());
        assert!(output.path.is_none());
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_screenshot_with_path() {
        let browser = Arc::new(MockBrowser { should_fail: false });
        let input = ScreenshotInput {
            full_page: Some(false),
            path: Some("/tmp/screenshot.png".to_string()),
        };

        let output = execute_screenshot(browser, input).await;

        assert!(output.success);
        assert!(output.data.is_none());
        assert_eq!(output.path, Some("/tmp/screenshot.png".to_string()));
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_execute_screenshot_failure() {
        let browser = Arc::new(MockBrowser { should_fail: true });
        let input = ScreenshotInput {
            full_page: None,
            path: None,
        };

        let output = execute_screenshot(browser, input).await;

        assert!(!output.success);
        assert!(output.error.is_some());
    }
}
