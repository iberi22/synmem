//! SynMem MCP Server Binary
//!
//! This binary runs the SynMem MCP server for memory and search tools.

use rmcp::service::ServiceExt;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use synmem_mcp::SynMemServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "synmem_mcp=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting SynMem MCP Server");

    // Get database path from environment or use default
    let db_path = env::var("SYNMEM_DB_PATH")
        .unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/.synmem/synmem.db", home)
        });

    // Ensure database directory exists
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    tracing::info!("Using database at: {}", db_path);

    // Create the server
    let server = SynMemServer::new(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to create server: {}", e))?;

    tracing::info!("SynMem MCP Server initialized successfully");

    // Run the server using stdio transport
    let service = server.serve(rmcp::transport::stdio()).await?;

    // Wait for the service to complete
    service.waiting().await?;

    tracing::info!("SynMem MCP Server shutting down");

    Ok(())
}
