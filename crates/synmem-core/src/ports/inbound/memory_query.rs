//! Memory query inbound port

use async_trait::async_trait;
use std::error::Error;

/// Port for querying memory/history
#[async_trait]
pub trait MemoryQueryPort: Send + Sync {
    /// Error type for this port
    type Error: Error + Send + Sync + 'static;

    /// Search memory by query string
    async fn search(&self, query: &str) -> Result<Vec<String>, Self::Error>;

    /// Get recent items
    async fn get_recent(&self, count: usize) -> Result<Vec<String>, Self::Error>;
}
