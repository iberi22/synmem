//! SQLite repository for macros
//!
//! Implements the MacroStorage port using SQLite.
//!
//! Note: We use `std::sync::Mutex` here instead of `tokio::sync::Mutex` because
//! rusqlite operations are synchronous and fast (local SQLite). Using tokio's
//! async mutex would provide no benefit since we're not holding the lock across
//! await points - all SQLite operations complete quickly within a single call.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

use synmem_core::{BrowserAction, CoreError, Macro, MacroInfo, MacroStorage};

/// SQLite-based macro storage
pub struct MacroRepository {
    conn: Mutex<Connection>,
}

impl MacroRepository {
    /// Create a new macro repository with the given database path
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let conn = Connection::open(db_path)
            .map_err(|e| CoreError::Storage(format!("Failed to open database: {}", e)))?;

        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Create a new in-memory macro repository (useful for testing)
    pub fn in_memory() -> Result<Self, CoreError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| CoreError::Storage(format!("Failed to create in-memory database: {}", e)))?;

        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.init_schema()?;
        Ok(repo)
    }

    /// Initialize the database schema
    fn init_schema(&self) -> Result<(), CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS macros (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                actions JSON NOT NULL,
                created_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| CoreError::Storage(format!("Failed to create macros table: {}", e)))?;

        // Create index on name for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_macros_name ON macros(name)",
            [],
        )
        .map_err(|e| CoreError::Storage(format!("Failed to create name index: {}", e)))?;

        Ok(())
    }

    /// Convert a Unix timestamp to DateTime<Utc>
    fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(timestamp, 0).unwrap_or_else(Utc::now)
    }
}

#[async_trait]
impl MacroStorage for MacroRepository {
    async fn save(&self, macro_data: &Macro) -> Result<(), CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        let actions_json = serde_json::to_string(&macro_data.actions)?;
        let created_at = macro_data.created_at.timestamp();

        conn.execute(
            "INSERT OR REPLACE INTO macros (id, name, actions, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![macro_data.id, macro_data.name, actions_json, created_at],
        )
        .map_err(|e| CoreError::Storage(format!("Failed to save macro: {}", e)))?;

        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<Macro>, CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn
            .prepare("SELECT id, name, actions, created_at FROM macros WHERE id = ?1")
            .map_err(|e| CoreError::Storage(format!("Failed to prepare statement: {}", e)))?;

        let result = stmt.query_row(params![id], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let actions_json: String = row.get(2)?;
            let created_at: i64 = row.get(3)?;
            Ok((id, name, actions_json, created_at))
        });

        match result {
            Ok((id, name, actions_json, created_at)) => {
                let actions: Vec<BrowserAction> = serde_json::from_str(&actions_json)?;
                let created_at = Self::timestamp_to_datetime(created_at);
                Ok(Some(Macro::with_id(id, name, actions, created_at)))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CoreError::Storage(format!("Failed to get macro: {}", e))),
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<Macro>, CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn
            .prepare("SELECT id, name, actions, created_at FROM macros WHERE name = ?1")
            .map_err(|e| CoreError::Storage(format!("Failed to prepare statement: {}", e)))?;

        let result = stmt.query_row(params![name], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let actions_json: String = row.get(2)?;
            let created_at: i64 = row.get(3)?;
            Ok((id, name, actions_json, created_at))
        });

        match result {
            Ok((id, name, actions_json, created_at)) => {
                let actions: Vec<BrowserAction> = serde_json::from_str(&actions_json)?;
                let created_at = Self::timestamp_to_datetime(created_at);
                Ok(Some(Macro::with_id(id, name, actions, created_at)))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CoreError::Storage(format!("Failed to get macro: {}", e))),
        }
    }

    async fn delete(&self, id: &str) -> Result<bool, CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        let rows_deleted = conn
            .execute("DELETE FROM macros WHERE id = ?1", params![id])
            .map_err(|e| CoreError::Storage(format!("Failed to delete macro: {}", e)))?;

        Ok(rows_deleted > 0)
    }

    async fn list(&self) -> Result<Vec<MacroInfo>, CoreError> {
        let conn = self.conn.lock().map_err(|e| {
            CoreError::Storage(format!("Failed to acquire database lock: {}", e))
        })?;

        let mut stmt = conn
            .prepare("SELECT id, name, actions, created_at FROM macros ORDER BY created_at DESC")
            .map_err(|e| CoreError::Storage(format!("Failed to prepare statement: {}", e)))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let actions_json: String = row.get(2)?;
                let created_at: i64 = row.get(3)?;
                Ok((id, name, actions_json, created_at))
            })
            .map_err(|e| CoreError::Storage(format!("Failed to query macros: {}", e)))?;

        let mut macros = Vec::new();
        for row in rows {
            let (id, name, actions_json, created_at) = row.map_err(|e| {
                CoreError::Storage(format!("Failed to read macro row: {}", e))
            })?;

            let actions: Vec<BrowserAction> = serde_json::from_str(&actions_json)?;
            let created_at = Self::timestamp_to_datetime(created_at);

            macros.push(MacroInfo {
                id,
                name,
                action_count: actions.len(),
                created_at,
            });
        }

        Ok(macros)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_get_macro() {
        let repo = MacroRepository::in_memory().unwrap();

        let macro_data = Macro::new(
            "test_macro",
            vec![
                BrowserAction::navigate("https://example.com"),
                BrowserAction::click("#btn"),
            ],
        );

        repo.save(&macro_data).await.unwrap();

        let retrieved = repo.get(&macro_data.id).await.unwrap().unwrap();
        assert_eq!(retrieved.id, macro_data.id);
        assert_eq!(retrieved.name, "test_macro");
        assert_eq!(retrieved.action_count(), 2);
    }

    #[tokio::test]
    async fn test_get_by_name() {
        let repo = MacroRepository::in_memory().unwrap();

        let macro_data = Macro::new("my_macro", vec![BrowserAction::click("#btn")]);
        repo.save(&macro_data).await.unwrap();

        let retrieved = repo.get_by_name("my_macro").await.unwrap().unwrap();
        assert_eq!(retrieved.name, "my_macro");
    }

    #[tokio::test]
    async fn test_get_nonexistent() {
        let repo = MacroRepository::in_memory().unwrap();

        let result = repo.get("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_delete_macro() {
        let repo = MacroRepository::in_memory().unwrap();

        let macro_data = Macro::new("test", vec![BrowserAction::click("#btn")]);
        repo.save(&macro_data).await.unwrap();

        let deleted = repo.delete(&macro_data.id).await.unwrap();
        assert!(deleted);

        let retrieved = repo.get(&macro_data.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let repo = MacroRepository::in_memory().unwrap();

        let deleted = repo.delete("nonexistent").await.unwrap();
        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_list_macros() {
        let repo = MacroRepository::in_memory().unwrap();

        let macro1 = Macro::new("macro1", vec![BrowserAction::click("#btn1")]);
        let macro2 = Macro::new(
            "macro2",
            vec![
                BrowserAction::click("#btn2"),
                BrowserAction::type_text("#input", "text"),
            ],
        );

        repo.save(&macro1).await.unwrap();
        repo.save(&macro2).await.unwrap();

        let macros = repo.list().await.unwrap();
        assert_eq!(macros.len(), 2);

        // macros are ordered by created_at DESC
        let names: Vec<&str> = macros.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"macro1"));
        assert!(names.contains(&"macro2"));
    }

    #[tokio::test]
    async fn test_update_macro() {
        let repo = MacroRepository::in_memory().unwrap();

        let mut macro_data = Macro::new("test", vec![BrowserAction::click("#btn1")]);
        repo.save(&macro_data).await.unwrap();

        // Update by saving with same ID
        macro_data.actions.push(BrowserAction::click("#btn2"));
        repo.save(&macro_data).await.unwrap();

        let retrieved = repo.get(&macro_data.id).await.unwrap().unwrap();
        assert_eq!(retrieved.action_count(), 2);
    }
}
