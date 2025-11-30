//! Storage outbound port

use async_trait::async_trait;
use std::error::Error;

/// Port for storage operations
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Error type for this port
    type Error: Error + Send + Sync + 'static;

    /// Store a value with a key
    async fn store(&self, key: &str, value: &str) -> Result<(), Self::Error>;

    /// Retrieve a value by key
    async fn retrieve(&self, key: &str) -> Result<Option<String>, Self::Error>;

    /// Delete a value by key
    async fn delete(&self, key: &str) -> Result<(), Self::Error>;

    /// List all keys with an optional prefix
    async fn list_keys(&self, prefix: Option<&str>) -> Result<Vec<String>, Self::Error>;
}
