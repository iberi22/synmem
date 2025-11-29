//! Storage service for cloud data persistence

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

/// Storage service errors
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Object not found: {0}")]
    NotFound(String),

    #[error("Object already exists: {0}")]
    AlreadyExists(String),

    #[error("Storage quota exceeded")]
    QuotaExceeded,

    #[error("Invalid key format")]
    InvalidKey,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// A stored object with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredObject {
    /// Object key (path-like identifier)
    pub key: String,
    /// User who owns this object
    pub user_id: Uuid,
    /// Object data
    pub data: Vec<u8>,
    /// Content type (MIME type)
    pub content_type: String,
    /// Size in bytes
    pub size: usize,
    /// Custom metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modification timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl StoredObject {
    /// Creates a new stored object
    #[must_use]
    pub fn new(key: String, user_id: Uuid, data: Vec<u8>, content_type: String) -> Self {
        let now = chrono::Utc::now();
        let size = data.len();
        Self {
            key,
            user_id,
            data,
            content_type,
            size,
            metadata: std::collections::HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Options for listing objects
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    /// Prefix to filter objects
    pub prefix: Option<String>,
    /// Maximum number of objects to return
    pub limit: Option<usize>,
    /// Cursor for pagination
    pub cursor: Option<String>,
}

/// Result of listing objects
#[derive(Debug, Clone)]
pub struct ListResult {
    /// Objects matching the query
    pub objects: Vec<StoredObject>,
    /// Cursor for next page (if more results exist)
    pub next_cursor: Option<String>,
    /// Total count (if available)
    pub total_count: Option<usize>,
}

/// Trait for storage operations
#[async_trait::async_trait]
pub trait StorageServiceTrait: Send + Sync {
    /// Stores an object
    async fn put(&self, object: StoredObject) -> Result<(), StorageError>;

    /// Gets an object by key
    async fn get(&self, key: &str, user_id: Uuid) -> Result<StoredObject, StorageError>;

    /// Deletes an object
    async fn delete(&self, key: &str, user_id: Uuid) -> Result<(), StorageError>;

    /// Lists objects for a user
    async fn list(&self, user_id: Uuid, options: ListOptions) -> Result<ListResult, StorageError>;

    /// Checks if an object exists
    async fn exists(&self, key: &str, user_id: Uuid) -> Result<bool, StorageError>;

    /// Gets storage usage for a user
    async fn get_usage(&self, user_id: Uuid) -> Result<StorageUsage, StorageError>;
}

/// Storage usage statistics for a user
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StorageUsage {
    /// Total bytes used
    pub bytes_used: u64,
    /// Number of objects
    pub object_count: u64,
    /// Storage quota (if applicable)
    pub quota_bytes: Option<u64>,
}

impl StorageUsage {
    /// Calculates the percentage of quota used
    #[must_use]
    pub fn quota_percentage(&self) -> Option<f64> {
        self.quota_bytes.map(|quota| {
            if quota == 0 {
                0.0
            } else {
                (self.bytes_used as f64 / quota as f64) * 100.0
            }
        })
    }
}

/// Default implementation of the storage service
#[derive(Default)]
pub struct StorageService {
    // In a real implementation, this would contain:
    // - Database connection pool (SQLite/Postgres)
    // - Object storage client (S3/GCS)
    // - Encryption service
    _private: (),
}

impl StorageService {
    /// Creates a new storage service
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { _private: () })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stored_object_creation() {
        let user_id = Uuid::new_v4();
        let data = b"test data".to_vec();
        let obj = StoredObject::new(
            "sessions/test.json".to_string(),
            user_id,
            data.clone(),
            "application/json".to_string(),
        );

        assert_eq!(obj.user_id, user_id);
        assert_eq!(obj.size, data.len());
        assert_eq!(obj.content_type, "application/json");
    }

    #[test]
    fn test_storage_usage_quota() {
        let usage = StorageUsage {
            bytes_used: 500,
            object_count: 10,
            quota_bytes: Some(1000),
        };

        assert_eq!(usage.quota_percentage(), Some(50.0));
    }

    #[test]
    fn test_storage_usage_no_quota() {
        let usage = StorageUsage {
            bytes_used: 500,
            object_count: 10,
            quota_bytes: None,
        };

        assert_eq!(usage.quota_percentage(), None);
    }

    #[test]
    fn test_storage_error_display() {
        let err = StorageError::NotFound("test.json".to_string());
        assert!(err.to_string().contains("test.json"));
    }
}
