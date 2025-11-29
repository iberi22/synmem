//! # Storage Port
//!
//! Outbound port for persistence operations.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur during storage operations.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("query failed: {0}")]
    QueryFailed(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("serialization failed: {0}")]
    SerializationFailed(String),

    #[error("constraint violation: {0}")]
    ConstraintViolation(String),
}

/// Result type for storage operations.
pub type StorageResult<T> = Result<T, StorageError>;

/// A stored record with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRecord {
    /// Unique identifier.
    pub id: Uuid,
    /// Record type/collection name.
    pub record_type: String,
    /// Serialized data (JSON).
    pub data: String,
    /// When this record was created.
    pub created_at: DateTime<Utc>,
    /// When this record was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Query filters for listing records.
#[derive(Debug, Clone, Default)]
pub struct QueryFilter {
    /// Filter by record type.
    pub record_type: Option<String>,
    /// Filter by creation date (after).
    pub created_after: Option<DateTime<Utc>>,
    /// Filter by creation date (before).
    pub created_before: Option<DateTime<Utc>>,
    /// Maximum number of results.
    pub limit: Option<usize>,
    /// Offset for pagination.
    pub offset: Option<usize>,
}

/// Outbound port for persistence operations.
///
/// This port defines the interface for storing and retrieving records.
/// Implementations might use SQLite, PostgreSQL, etc.
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Initialize the storage (create tables, run migrations, etc.).
    async fn initialize(&self) -> StorageResult<()>;

    /// Save a record to storage.
    async fn save(&self, record_type: &str, id: &Uuid, data: &str) -> StorageResult<StoredRecord>;

    /// Get a record by ID.
    async fn get(&self, record_type: &str, id: &Uuid) -> StorageResult<StoredRecord>;

    /// List records matching the filter.
    async fn list(&self, filter: QueryFilter) -> StorageResult<Vec<StoredRecord>>;

    /// Delete a record by ID.
    async fn delete(&self, record_type: &str, id: &Uuid) -> StorageResult<()>;

    /// Check if a record exists.
    async fn exists(&self, record_type: &str, id: &Uuid) -> StorageResult<bool>;
}
