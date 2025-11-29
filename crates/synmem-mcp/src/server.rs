//! MCP Server implementation for SynMem

use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    schemars, tool, tool_router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tools::MemoryTools;
use synmem_storage::SqliteStorage;

/// Input for search_memory tool
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchMemoryInput {
    /// The search query string
    #[schemars(description = "The search query string")]
    pub query: String,
    /// Maximum number of results to return (default: 10)
    #[schemars(description = "Maximum number of results to return (default: 10)")]
    pub limit: Option<usize>,
    /// Filter by content types: page, chat, context, tweet
    #[schemars(description = "Filter by content types: page, chat, context, tweet")]
    pub content_types: Option<Vec<String>>,
    /// Filter by sources: gemini, chatgpt, claude, web, etc.
    #[schemars(description = "Filter by sources: gemini, chatgpt, claude, web, etc.")]
    pub sources: Option<Vec<String>>,
}

/// Input for get_recent tool
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetRecentInput {
    /// Number of recent items to retrieve (default: 10)
    #[schemars(description = "Number of recent items to retrieve (default: 10)")]
    pub limit: Option<usize>,
}

/// Input for save_context tool
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SaveContextInput {
    /// Name or title for the saved context
    #[schemars(description = "Name or title for the saved context")]
    pub name: String,
    /// The context content to save
    #[schemars(description = "The context content to save")]
    pub content: String,
    /// Tags for categorization and retrieval
    #[schemars(description = "Tags for categorization and retrieval")]
    pub tags: Option<Vec<String>>,
}

/// Input for get_chat_context tool
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetChatContextInput {
    /// The unique ID of the chat to retrieve
    #[schemars(description = "The unique ID of the chat to retrieve")]
    pub chat_id: String,
}

/// SynMem MCP Server
#[derive(Clone)]
pub struct SynMemServer {
    tools: Arc<RwLock<MemoryTools<SqliteStorage>>>,
    tool_router: ToolRouter<Self>,
}

impl SynMemServer {
    /// Creates a new SynMemServer with the given database path
    pub fn new(db_path: &str) -> Result<Self, String> {
        let tools = MemoryTools::new(db_path)?;
        Ok(Self {
            tools: Arc::new(RwLock::new(tools)),
            tool_router: Self::tool_router(),
        })
    }

    /// Creates a new SynMemServer with in-memory storage (for testing)
    pub fn in_memory() -> Result<Self, String> {
        let tools = MemoryTools::in_memory()?;
        Ok(Self {
            tools: Arc::new(RwLock::new(tools)),
            tool_router: Self::tool_router(),
        })
    }
}

#[tool_router]
impl SynMemServer {
    /// Search through stored pages and chats using semantic/full-text search
    #[tool(description = "Search through stored pages and chats using semantic/full-text search. Returns relevant results with snippets and relevance scores.")]
    async fn search_memory(
        &self,
        Parameters(input): Parameters<SearchMemoryInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let tools_input = crate::tools::memory_tools::SearchMemoryInput {
            query: input.query,
            limit: input.limit,
            content_types: input.content_types,
            sources: input.sources,
        };

        let tools = self.tools.read().await;
        match tools.search_memory(tools_input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// Get the most recently scraped items from memory
    #[tool(description = "Get the most recently scraped items from memory. Useful for accessing recent context without searching.")]
    async fn get_recent(
        &self,
        Parameters(input): Parameters<GetRecentInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let tools_input = crate::tools::memory_tools::GetRecentInput {
            limit: input.limit.unwrap_or(10),
        };

        let tools = self.tools.read().await;
        match tools.get_recent(tools_input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// Save current context with tags for later retrieval
    #[tool(description = "Save current context with tags for later retrieval. Use this to store important information that may be useful in future sessions.")]
    async fn save_context(
        &self,
        Parameters(input): Parameters<SaveContextInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let tools_input = crate::tools::memory_tools::SaveContextInput {
            name: input.name,
            content: input.content,
            tags: input.tags.unwrap_or_default(),
        };

        let tools = self.tools.read().await;
        match tools.save_context(tools_input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// List all saved browser sessions
    #[tool(description = "List all saved browser sessions. Shows session metadata including creation time, last activity, and item count.")]
    async fn list_sessions(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let tools = self.tools.read().await;
        match tools.list_sessions().await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// Get a specific AI chat conversation for context injection
    #[tool(description = "Get a specific AI chat conversation for context injection. Retrieves the full conversation history from a stored chat.")]
    async fn get_chat_context(
        &self,
        Parameters(input): Parameters<GetChatContextInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let tools_input = crate::tools::memory_tools::GetChatContextInput {
            chat_id: input.chat_id,
        };

        let tools = self.tools.read().await;
        match tools.get_chat_context(tools_input).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }
}

impl ServerHandler for SynMemServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "SynMem Memory & Search MCP Server. Use these tools to search, store, and retrieve context from scraped web pages and AI conversations.".to_string()
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let server = SynMemServer::in_memory().unwrap();
        let info = server.get_info();
        
        assert!(info.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_tool_list() {
        let server = SynMemServer::in_memory().unwrap();
        let tools = server.tool_router.list_all();
        
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(tool_names.contains(&"search_memory"));
        assert!(tool_names.contains(&"get_recent"));
        assert!(tool_names.contains(&"save_context"));
        assert!(tool_names.contains(&"list_sessions"));
        assert!(tool_names.contains(&"get_chat_context"));
    }
}
