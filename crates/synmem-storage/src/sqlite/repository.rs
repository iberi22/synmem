//! SQLite storage repository implementation

use async_trait::async_trait;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

use synmem_core::{
    ChatContext, ChatMessage, ChatSource, ContentType, Memory, SavedContext, SearchResult,
    Session, StorageError, StoragePort,
};

use super::migrations;

/// SQLite-based storage implementation
pub struct SqliteStorage {
    conn: Mutex<Connection>,
}

impl SqliteStorage {
    /// Creates a new SqliteStorage with the given database path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, StorageError> {
        let conn = Connection::open(path)
            .map_err(|e| StorageError::Connection(e.to_string()))?;

        migrations::init_schema(&conn)
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Creates a new in-memory SqliteStorage (for testing)
    pub fn in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| StorageError::Connection(e.to_string()))?;

        migrations::init_schema(&conn)
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn tags_to_string(tags: &[String]) -> String {
        tags.join(",")
    }

    fn string_to_tags(s: &str) -> Vec<String> {
        if s.is_empty() {
            Vec::new()
        } else {
            s.split(',').map(|s| s.to_string()).collect()
        }
    }
}

#[async_trait]
impl StoragePort for SqliteStorage {
    async fn search(
        &self,
        query: &str,
        limit: Option<usize>,
        content_types: Option<Vec<String>>,
        sources: Option<Vec<String>>,
    ) -> Result<Vec<SearchResult>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;
        let limit = limit.unwrap_or(10);

        // Build the query with filters
        let mut sql = String::from(
            "SELECT m.id, m.title, m.content, m.source, m.source_url, m.content_type,
                    bm25(memories_fts) as rank
             FROM memories_fts
             JOIN memories m ON memories_fts.rowid = m.rowid
             WHERE memories_fts MATCH ?1"
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(query.to_string())];
        let mut param_idx = 2;

        if let Some(ref types) = content_types {
            if !types.is_empty() {
                let placeholders: Vec<String> = types.iter().enumerate()
                    .map(|(i, _)| format!("?{}", param_idx + i))
                    .collect();
                sql.push_str(&format!(" AND m.content_type IN ({})", placeholders.join(",")));
                for t in types {
                    params.push(Box::new(t.clone()));
                }
                param_idx += types.len();
            }
        }

        if let Some(ref srcs) = sources {
            if !srcs.is_empty() {
                let placeholders: Vec<String> = srcs.iter().enumerate()
                    .map(|(i, _)| format!("?{}", param_idx + i))
                    .collect();
                sql.push_str(&format!(" AND m.source IN ({})", placeholders.join(",")));
                for s in srcs {
                    params.push(Box::new(s.clone()));
                }
            }
        }

        sql.push_str(&format!(" ORDER BY rank LIMIT {}", limit));

        let mut stmt = conn.prepare(&sql)
            .map_err(|e| StorageError::Database(e.to_string()))?;

        // Convert params to references
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let results = stmt
            .query_map(param_refs.as_slice(), |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let content: String = row.get(2)?;
                let source: String = row.get(3)?;
                let source_url: Option<String> = row.get(4)?;
                let content_type: String = row.get(5)?;
                let rank: f64 = row.get(6)?;

                // Create snippet from content (first 200 chars around match)
                let snippet = if content.len() > 200 {
                    format!("{}...", &content[..200])
                } else {
                    content
                };

                // Normalize rank to 0-1 scale (BM25 returns negative values, lower is better)
                let relevance = 1.0 / (1.0 + rank.abs());

                Ok(SearchResult::new(
                    Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                    title,
                    snippet,
                    source,
                    relevance,
                )
                .with_url(source_url.unwrap_or_default())
                .with_content_type(content_type))
            })
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let mut search_results = Vec::new();
        for result in results {
            search_results.push(result.map_err(|e| StorageError::Database(e.to_string()))?);
        }

        Ok(search_results)
    }

    async fn get_recent_memories(&self, limit: usize) -> Result<Vec<Memory>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, content_type, title, content, source_url, source, tags,
                        session_id, created_at, updated_at, metadata
                 FROM memories
                 ORDER BY created_at DESC
                 LIMIT ?1",
            )
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let results = stmt
            .query_map([limit], |row| {
                let id: String = row.get(0)?;
                let content_type: String = row.get(1)?;
                let title: String = row.get(2)?;
                let content: String = row.get(3)?;
                let source_url: Option<String> = row.get(4)?;
                let source: String = row.get(5)?;
                let tags: String = row.get(6)?;
                let session_id: Option<String> = row.get(7)?;
                let created_at: String = row.get(8)?;
                let updated_at: String = row.get(9)?;
                let metadata: Option<String> = row.get(10)?;

                Ok(Memory {
                    id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                    content_type: content_type.parse().unwrap_or(ContentType::Other),
                    title,
                    content,
                    source_url,
                    source,
                    tags: Self::string_to_tags(&tags),
                    session_id: session_id.and_then(|s| Uuid::parse_str(&s).ok()),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    metadata: metadata.and_then(|m| serde_json::from_str(&m).ok()),
                })
            })
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let mut memories = Vec::new();
        for result in results {
            memories.push(result.map_err(|e| StorageError::Database(e.to_string()))?);
        }

        Ok(memories)
    }

    async fn save_memory(&self, memory: &Memory) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO memories
             (id, content_type, title, content, source_url, source, tags, session_id,
              created_at, updated_at, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                memory.id.to_string(),
                memory.content_type.to_string(),
                memory.title,
                memory.content,
                memory.source_url,
                memory.source,
                Self::tags_to_string(&memory.tags),
                memory.session_id.map(|id| id.to_string()),
                memory.created_at.to_rfc3339(),
                memory.updated_at.to_rfc3339(),
                memory.metadata.as_ref().map(|m| m.to_string()),
            ],
        )
        .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(())
    }

    async fn save_context(&self, context: &SavedContext) -> Result<(), StorageError> {
        {
            let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

            conn.execute(
                "INSERT OR REPLACE INTO saved_contexts
                 (id, name, content, tags, created_at, session_id, metadata)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    context.id.to_string(),
                    context.name,
                    context.content,
                    Self::tags_to_string(&context.tags),
                    context.created_at.to_rfc3339(),
                    context.session_id.map(|id| id.to_string()),
                    context.metadata.as_ref().map(|m| m.to_string()),
                ],
            )
            .map_err(|e| StorageError::Database(e.to_string()))?;
        } // conn lock is released here

        // Also save as memory for searchability
        let memory = Memory::new(
            ContentType::Context,
            context.name.clone(),
            context.content.clone(),
            "context".to_string(),
        )
        .with_tags(context.tags.clone());

        self.save_memory(&memory).await?;

        Ok(())
    }

    async fn list_sessions(&self) -> Result<Vec<Session>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, name, created_at, last_active_at, is_active, item_count, metadata
                 FROM sessions
                 ORDER BY last_active_at DESC",
            )
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let results = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let created_at: String = row.get(2)?;
                let last_active_at: String = row.get(3)?;
                let is_active: bool = row.get(4)?;
                let item_count: u32 = row.get(5)?;
                let metadata: Option<String> = row.get(6)?;

                Ok(Session {
                    id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                    name,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    last_active_at: chrono::DateTime::parse_from_rfc3339(&last_active_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    is_active,
                    item_count,
                    metadata: metadata.and_then(|m| serde_json::from_str(&m).ok()),
                })
            })
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let mut sessions = Vec::new();
        for result in results {
            sessions.push(result.map_err(|e| StorageError::Database(e.to_string()))?);
        }

        Ok(sessions)
    }

    async fn get_session(&self, id: &Uuid) -> Result<Option<Session>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        let result = conn.query_row(
            "SELECT id, name, created_at, last_active_at, is_active, item_count, metadata
             FROM sessions
             WHERE id = ?1",
            [id.to_string()],
            |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let created_at: String = row.get(2)?;
                let last_active_at: String = row.get(3)?;
                let is_active: bool = row.get(4)?;
                let item_count: u32 = row.get(5)?;
                let metadata: Option<String> = row.get(6)?;

                Ok(Session {
                    id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                    name,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    last_active_at: chrono::DateTime::parse_from_rfc3339(&last_active_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    is_active,
                    item_count,
                    metadata: metadata.and_then(|m| serde_json::from_str(&m).ok()),
                })
            },
        );

        match result {
            Ok(session) => Ok(Some(session)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StorageError::Database(e.to_string())),
        }
    }

    async fn save_session(&self, session: &Session) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO sessions
             (id, name, created_at, last_active_at, is_active, item_count, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                session.id.to_string(),
                session.name,
                session.created_at.to_rfc3339(),
                session.last_active_at.to_rfc3339(),
                session.is_active,
                session.item_count,
                session.metadata.as_ref().map(|m| m.to_string()),
            ],
        )
        .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_chat_context(&self, id: &Uuid) -> Result<Option<ChatContext>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

        let result = conn.query_row(
            "SELECT id, title, source, messages, created_at, updated_at, url, session_id
             FROM chat_contexts
             WHERE id = ?1",
            [id.to_string()],
            |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let source: String = row.get(2)?;
                let messages_json: String = row.get(3)?;
                let created_at: String = row.get(4)?;
                let updated_at: String = row.get(5)?;
                let url: Option<String> = row.get(6)?;
                let session_id: Option<String> = row.get(7)?;

                Ok((id, title, source, messages_json, created_at, updated_at, url, session_id))
            },
        );

        match result {
            Ok((id, title, source, messages_json, created_at, updated_at, url, session_id)) => {
                let messages: Vec<ChatMessage> =
                    serde_json::from_str(&messages_json)
                        .map_err(|e| StorageError::Serialization(e.to_string()))?;

                let chat = ChatContext {
                    id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                    title,
                    source: source.parse().unwrap_or(ChatSource::Other("unknown".to_string())),
                    messages,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    url,
                    session_id: session_id.and_then(|s| Uuid::parse_str(&s).ok()),
                };

                Ok(Some(chat))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StorageError::Database(e.to_string())),
        }
    }

    async fn save_chat_context(&self, chat: &ChatContext) -> Result<(), StorageError> {
        let messages_json = serde_json::to_string(&chat.messages)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        {
            let conn = self.conn.lock().map_err(|e| StorageError::Database(e.to_string()))?;

            conn.execute(
                "INSERT OR REPLACE INTO chat_contexts
                 (id, title, source, messages, created_at, updated_at, url, session_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    chat.id.to_string(),
                    chat.title,
                    chat.source.to_string(),
                    messages_json,
                    chat.created_at.to_rfc3339(),
                    chat.updated_at.to_rfc3339(),
                    chat.url,
                    chat.session_id.map(|id| id.to_string()),
                ],
            )
            .map_err(|e| StorageError::Database(e.to_string()))?;
        } // conn lock is released here

        // Also save as memory for searchability
        let memory = Memory::new(
            ContentType::Chat,
            chat.title.clone(),
            chat.to_text(),
            chat.source.to_string(),
        );

        self.save_memory(&memory).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synmem_core::domain::entities::ContentType;

    #[tokio::test]
    async fn test_save_and_get_memory() {
        let storage = SqliteStorage::in_memory().unwrap();

        let memory = Memory::new(
            ContentType::Page,
            "Test Page".to_string(),
            "This is test content for searching".to_string(),
            "web".to_string(),
        )
        .with_tags(vec!["test".to_string()]);

        storage.save_memory(&memory).await.unwrap();

        let recent = storage.get_recent_memories(10).await.unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].title, "Test Page");
    }

    #[tokio::test]
    async fn test_fts_search() {
        let storage = SqliteStorage::in_memory().unwrap();

        let memory1 = Memory::new(
            ContentType::Chat,
            "Architecture Discussion".to_string(),
            "We discussed hexagonal architecture patterns".to_string(),
            "gemini".to_string(),
        );

        let memory2 = Memory::new(
            ContentType::Page,
            "Random Page".to_string(),
            "Some unrelated content here".to_string(),
            "web".to_string(),
        );

        storage.save_memory(&memory1).await.unwrap();
        storage.save_memory(&memory2).await.unwrap();

        let results = storage.search("hexagonal architecture", None, None, None).await.unwrap();
        assert!(!results.is_empty());
        assert!(results[0].title.contains("Architecture"));
    }

    #[tokio::test]
    async fn test_save_and_list_sessions() {
        let storage = SqliteStorage::in_memory().unwrap();

        let session = Session::new("Test Session".to_string());
        storage.save_session(&session).await.unwrap();

        let sessions = storage.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].name, "Test Session");
    }

    #[tokio::test]
    async fn test_save_context() {
        let storage = SqliteStorage::in_memory().unwrap();

        let context = SavedContext::new(
            "My Context".to_string(),
            "Important context information".to_string(),
            vec!["important".to_string()],
        );

        storage.save_context(&context).await.unwrap();

        // Should be searchable as memory
        let results = storage.search("Important context", None, None, None).await.unwrap();
        assert!(!results.is_empty());
    }
}
