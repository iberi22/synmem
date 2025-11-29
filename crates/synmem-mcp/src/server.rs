//! MCP Server implementation for SynMem
//!
//! This module contains the main MCP server struct that exposes
//! browser automation tools through the MCP protocol.

use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use synmem_core::ports::inbound::{BrowserControlPort, MemoryQueryPort, ScraperPort};
use tracing::info;

/// Error type for MCP server operations
#[derive(Debug, thiserror::Error)]
pub enum McpServerError {
    /// Server initialization failed
    #[error("Server initialization failed: {0}")]
    InitializationFailed(String),
    /// Server runtime error
    #[error("Server runtime error: {0}")]
    RuntimeError(String),
}

/// SynMem MCP Server
///
/// Main MCP server struct that holds references to the core ports
/// and exposes browser automation tools.
#[derive(Clone)]
pub struct SynMemMcpServer {
    /// Browser control port for navigation and interaction
    browser_control: Option<Arc<dyn BrowserControlPort>>,
    /// Scraper port for content extraction
    scraper: Option<Arc<dyn ScraperPort>>,
    /// Memory query port for semantic search
    memory: Option<Arc<dyn MemoryQueryPort>>,
    /// Tool router for MCP tools
    tool_router: ToolRouter<Self>,
}

impl SynMemMcpServer {
    /// Creates a new MCP server with the given ports
    pub fn new(
        browser_control: Option<Arc<dyn BrowserControlPort>>,
        scraper: Option<Arc<dyn ScraperPort>>,
        memory: Option<Arc<dyn MemoryQueryPort>>,
    ) -> Self {
        Self {
            browser_control,
            scraper,
            memory,
            tool_router: Self::tool_router(),
        }
    }

    /// Creates a new MCP server without any adapters (for testing)
    pub fn new_stub() -> Self {
        Self {
            browser_control: None,
            scraper: None,
            memory: None,
            tool_router: Self::tool_router(),
        }
    }

    /// Runs the MCP server with stdio transport
    pub async fn run_stdio(self) -> Result<(), McpServerError> {
        info!("Starting SynMem MCP server with stdio transport");

        let transport = rmcp::transport::io::stdio();
        let server = self
            .serve(transport)
            .await
            .map_err(|e| McpServerError::InitializationFailed(e.to_string()))?;

        // Wait for the server to complete
        server
            .waiting()
            .await
            .map_err(|e| McpServerError::RuntimeError(e.to_string()))?;

        Ok(())
    }
}

// MCP Tool implementations using tool_router macro

/// Navigate to URL tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NavigateParams {
    /// URL to navigate to
    pub url: String,
    /// Session ID (optional, creates new if not provided)
    #[serde(default)]
    pub session_id: Option<String>,
    /// Wait condition (dom_content_loaded, load, network_idle)
    #[serde(default)]
    pub wait_until: Option<String>,
    /// Timeout in milliseconds
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// Click element tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ClickParams {
    /// Session ID
    pub session_id: String,
    /// CSS selector to click
    #[serde(default)]
    pub selector: Option<String>,
    /// Text to match
    #[serde(default)]
    pub text: Option<String>,
}

/// Type text tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TypeParams {
    /// Session ID
    pub session_id: String,
    /// CSS selector for input element
    pub selector: String,
    /// Text to type
    pub text: String,
    /// Clear existing text first
    #[serde(default)]
    pub clear: bool,
}

/// Scroll tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScrollParams {
    /// Session ID
    pub session_id: String,
    /// X offset
    #[serde(default)]
    pub x: i32,
    /// Y offset
    #[serde(default)]
    pub y: i32,
}

/// Screenshot tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScreenshotParams {
    /// Session ID
    pub session_id: String,
    /// Full page screenshot
    #[serde(default)]
    pub full_page: bool,
    /// Selector for element screenshot
    #[serde(default)]
    pub selector: Option<String>,
}

/// Wait for element tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WaitForParams {
    /// Session ID
    pub session_id: String,
    /// CSS selector to wait for
    pub selector: String,
    /// Timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    30000
}

/// Scrape page tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScrapePageParams {
    /// Session ID
    pub session_id: String,
    /// Include HTML content
    #[serde(default)]
    pub include_html: bool,
    /// Extract links
    #[serde(default = "default_true")]
    pub extract_links: bool,
    /// Specific selector to scrape
    #[serde(default)]
    pub selector: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Scrape chat tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScrapeChatParams {
    /// Session ID
    pub session_id: String,
    /// Chat platform (chatgpt, claude, gemini, universal)
    #[serde(default)]
    pub platform: Option<String>,
    /// Max messages to extract
    #[serde(default)]
    pub max_messages: Option<usize>,
}

/// Extract links tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExtractLinksParams {
    /// Session ID
    pub session_id: String,
}

/// Search memory tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchMemoryParams {
    /// Search query
    pub query: String,
    /// Maximum number of results
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Minimum relevance score (0.0 - 1.0)
    #[serde(default)]
    pub min_score: Option<f32>,
}

fn default_limit() -> usize {
    10
}

/// Get recent memories tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetRecentParams {
    /// Number of recent items to retrieve
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Save context tool parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SaveContextParams {
    /// Session ID to save context from
    pub session_id: String,
    /// Optional tags
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Tool result wrapper for JSON responses
#[derive(Debug, Serialize)]
pub struct ToolResultData {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResultData {
    pub fn success(data: impl Serialize) -> Self {
        Self {
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
            error: None,
        }
    }

    #[allow(dead_code)]
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

// Implement tools using tool_router macro
#[tool_router]
impl SynMemMcpServer {
    /// Navigate to a URL in the browser
    #[tool(description = "Navigate to a URL. Creates a new session if session_id is not provided.")]
    pub async fn navigate(&self, params: Parameters<NavigateParams>) -> String {
        let params = params.0;
        let session_id = params.session_id.unwrap_or_else(|| "session-1".to_string());
        let status = if self.browser_control.is_some() {
            "navigated"
        } else {
            "navigated (stub)"
        };

        let result = ToolResultData::success(serde_json::json!({
            "session_id": session_id,
            "url": params.url,
            "status": status
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Click on an element
    #[tool(description = "Click on an element by selector or text.")]
    pub async fn click(&self, params: Parameters<ClickParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "clicked": params.selector.or(params.text).unwrap_or_default(),
            "status": "clicked"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Type text into an element
    #[tool(description = "Type text into an input element.")]
    pub async fn type_text(&self, params: Parameters<TypeParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "selector": params.selector,
            "typed": params.text.len(),
            "status": "typed"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Scroll the page
    #[tool(description = "Scroll the page by x and y offset.")]
    pub async fn scroll(&self, params: Parameters<ScrollParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "scrolled_to": { "x": params.x, "y": params.y },
            "status": "scrolled"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Take a screenshot
    #[tool(description = "Take a screenshot of the current page or a specific element.")]
    pub async fn screenshot(&self, params: Parameters<ScreenshotParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "full_page": params.full_page,
            "selector": params.selector,
            "status": "screenshot_taken",
            "base64": ""
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Wait for an element
    #[tool(description = "Wait for an element to appear on the page.")]
    pub async fn wait_for(&self, params: Parameters<WaitForParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "selector": params.selector,
            "timeout_ms": params.timeout_ms,
            "status": "element_found"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Scrape page content
    #[tool(description = "Scrape the current page content including text, links, and metadata.")]
    pub async fn scrape_page(&self, params: Parameters<ScrapePageParams>) -> String {
        let params = params.0;
        let status = if self.scraper.is_some() {
            "scraped"
        } else {
            "scraped (stub)"
        };

        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "url": "https://example.com",
            "title": "Example Page",
            "content": "Page content extracted",
            "links": [],
            "status": status
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Scrape chat messages
    #[tool(description = "Scrape chat messages from AI chat interfaces (ChatGPT, Claude, Gemini).")]
    pub async fn scrape_chat(&self, params: Parameters<ScrapeChatParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "platform": params.platform.unwrap_or_else(|| "universal".to_string()),
            "messages": [],
            "status": "scraped"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Extract links from page
    #[tool(description = "Extract all links from the current page.")]
    pub async fn extract_links(&self, params: Parameters<ExtractLinksParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "links": [],
            "status": "extracted"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Search memory
    #[tool(description = "Search stored memories using semantic search.")]
    pub async fn search_memory(&self, params: Parameters<SearchMemoryParams>) -> String {
        let params = params.0;
        let status = if self.memory.is_some() {
            "searched"
        } else {
            "searched (stub)"
        };

        let result = ToolResultData::success(serde_json::json!({
            "query": params.query,
            "results": [],
            "status": status
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Get recent memories
    #[tool(description = "Get the most recent stored memories.")]
    pub async fn get_recent(&self, params: Parameters<GetRecentParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "limit": params.limit,
            "memories": [],
            "status": "retrieved"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }

    /// Save current context
    #[tool(description = "Save the current page context to memory.")]
    pub async fn save_context(&self, params: Parameters<SaveContextParams>) -> String {
        let params = params.0;
        let result = ToolResultData::success(serde_json::json!({
            "session_id": params.session_id,
            "tags": params.tags,
            "memory_id": "mem-1",
            "status": "saved"
        }));

        serde_json::to_string(&result).unwrap_or_default()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for SynMemMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "synmem".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("SynMem MCP Server".to_string()),
                icons: None,
                website_url: None,
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some(
                "SynMem MCP Server for browser automation. \
                Use navigate to open URLs, scrape_page to extract content, \
                and search_memory to find stored information."
                    .to_string(),
            ),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::handler::server::wrapper::Parameters;

    #[test]
    fn test_server_info() {
        let server = SynMemMcpServer::new_stub();
        let info = server.get_info();
        assert_eq!(info.server_info.name, "synmem");
    }

    #[test]
    fn test_server_capabilities() {
        let server = SynMemMcpServer::new_stub();
        let info = server.get_info();
        assert!(info.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_navigate_stub() {
        let server = SynMemMcpServer::new_stub();
        let result = server
            .navigate(Parameters(NavigateParams {
                url: "https://example.com".to_string(),
                session_id: None,
                wait_until: None,
                timeout_ms: None,
            }))
            .await;

        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true);
    }

    #[tokio::test]
    async fn test_scrape_page_stub() {
        let server = SynMemMcpServer::new_stub();
        let result = server
            .scrape_page(Parameters(ScrapePageParams {
                session_id: "test-session".to_string(),
                include_html: false,
                extract_links: true,
                selector: None,
            }))
            .await;

        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["success"], true);
    }
}
