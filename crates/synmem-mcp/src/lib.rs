//! SynMem MCP Server Library
//!
//! MCP server adapter for exposing browser automation tools.

pub mod tools;

use rmcp::model::{Implementation, ServerCapabilities, ServerInfo, ToolsCapability};

/// Get the MCP server information.
pub fn server_info() -> ServerInfo {
    ServerInfo {
        protocol_version: Default::default(),
        capabilities: server_capabilities(),
        server_info: Implementation {
            name: "synmem".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        instructions: Some("SynMem browser automation MCP server".to_string()),
    }
}

/// Get the MCP server capabilities.
pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        tools: Some(ToolsCapability { list_changed: None }),
        ..Default::default()
    }
}
