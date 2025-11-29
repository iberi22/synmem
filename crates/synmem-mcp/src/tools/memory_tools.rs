//! Memory tools for MCP

use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use synmem_core::{MemoryQueryPort, MemoryService, StoragePort};
use synmem_storage::SqliteStorage;

/// Tool input for search_memory
#[derive(Debug, Deserialize)]
pub struct SearchMemoryInput {
    /// The search query
    pub query: String,
    /// Maximum number of results (default: 10)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Filter by content types (page, chat, context, tweet)
    #[serde(default)]
    pub content_types: Option<Vec<String>>,
    /// Filter by sources (gemini, chatgpt, claude, web, etc.)
    #[serde(default)]
    pub sources: Option<Vec<String>>,
}

/// Tool input for get_recent
#[derive(Debug, Deserialize)]
pub struct GetRecentInput {
    /// Number of recent items to retrieve (default: 10)
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Tool input for save_context
#[derive(Debug, Deserialize)]
pub struct SaveContextInput {
    /// Name/title for the context
    pub name: String,
    /// The context content to save
    pub content: String,
    /// Tags for categorization and retrieval
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Tool input for get_chat_context
#[derive(Debug, Deserialize)]
pub struct GetChatContextInput {
    /// The chat ID to retrieve
    pub chat_id: String,
}

/// Memory tools implementation
pub struct MemoryTools<S: StoragePort + Send + Sync> {
    service: MemoryService<S>,
}

impl MemoryTools<SqliteStorage> {
    /// Creates a new MemoryTools instance with default SQLite storage
    pub fn new(db_path: &str) -> Result<Self, String> {
        let storage = SqliteStorage::new(db_path)
            .map_err(|e| format!("Failed to initialize storage: {}", e))?;
        
        Ok(Self {
            service: MemoryService::new(Arc::new(storage)),
        })
    }

    /// Creates a new MemoryTools instance with in-memory storage (for testing)
    pub fn in_memory() -> Result<Self, String> {
        let storage = SqliteStorage::in_memory()
            .map_err(|e| format!("Failed to initialize storage: {}", e))?;
        
        Ok(Self {
            service: MemoryService::new(Arc::new(storage)),
        })
    }
}

impl<S: StoragePort + Send + Sync> MemoryTools<S> {
    /// Creates a new MemoryTools instance with custom storage
    pub fn with_storage(storage: Arc<S>) -> Self {
        Self {
            service: MemoryService::new(storage),
        }
    }

    /// Search memory using semantic/full-text search
    pub async fn search_memory(&self, input: SearchMemoryInput) -> Result<Value, String> {
        use synmem_core::ports::inbound::memory_query::SearchOptions;

        let options = SearchOptions {
            limit: input.limit,
            content_types: input.content_types,
            sources: input.sources,
        };

        let results = self.service
            .search_memory(&input.query, Some(options))
            .await
            .map_err(|e| format!("Search failed: {}", e))?;

        let response: Vec<Value> = results
            .into_iter()
            .map(|r| {
                json!({
                    "id": r.id.to_string(),
                    "title": r.title,
                    "snippet": r.snippet,
                    "source": r.source,
                    "relevance": r.relevance,
                    "url": r.url,
                    "content_type": r.content_type
                })
            })
            .collect();

        Ok(json!({ "results": response }))
    }

    /// Get the most recent scraped items
    pub async fn get_recent(&self, input: GetRecentInput) -> Result<Value, String> {
        let memories = self.service
            .get_recent(input.limit)
            .await
            .map_err(|e| format!("Failed to get recent items: {}", e))?;

        let response: Vec<Value> = memories
            .into_iter()
            .map(|m| {
                json!({
                    "id": m.id.to_string(),
                    "content_type": m.content_type.to_string(),
                    "title": m.title,
                    "source": m.source,
                    "source_url": m.source_url,
                    "tags": m.tags,
                    "created_at": m.created_at.to_rfc3339(),
                    "snippet": if m.content.len() > 200 {
                        format!("{}...", &m.content[..200])
                    } else {
                        m.content.clone()
                    }
                })
            })
            .collect();

        Ok(json!({ "items": response, "count": response.len() }))
    }

    /// Save current context with tags for later retrieval
    pub async fn save_context(&self, input: SaveContextInput) -> Result<Value, String> {
        let context = self.service
            .save_context(&input.name, &input.content, input.tags)
            .await
            .map_err(|e| format!("Failed to save context: {}", e))?;

        Ok(json!({
            "success": true,
            "id": context.id.to_string(),
            "name": context.name,
            "tags": context.tags,
            "created_at": context.created_at.to_rfc3339()
        }))
    }

    /// List all saved browser sessions
    pub async fn list_sessions(&self) -> Result<Value, String> {
        let sessions = self.service
            .list_sessions()
            .await
            .map_err(|e| format!("Failed to list sessions: {}", e))?;

        let response: Vec<Value> = sessions
            .into_iter()
            .map(|s| {
                json!({
                    "id": s.id.to_string(),
                    "name": s.name,
                    "created_at": s.created_at.to_rfc3339(),
                    "last_active_at": s.last_active_at.to_rfc3339(),
                    "is_active": s.is_active,
                    "item_count": s.item_count
                })
            })
            .collect();

        Ok(json!({ "sessions": response, "count": response.len() }))
    }

    /// Get specific AI chat conversation for context injection
    pub async fn get_chat_context(&self, input: GetChatContextInput) -> Result<Value, String> {
        let chat = self.service
            .get_chat_context(&input.chat_id)
            .await
            .map_err(|e| format!("Failed to get chat context: {}", e))?;

        match chat {
            Some(c) => {
                let messages: Vec<Value> = c.messages
                    .iter()
                    .map(|m| {
                        json!({
                            "role": match m.role {
                                synmem_core::domain::entities::chat_context::MessageRole::User => "user",
                                synmem_core::domain::entities::chat_context::MessageRole::Assistant => "assistant",
                                synmem_core::domain::entities::chat_context::MessageRole::System => "system",
                            },
                            "content": m.content,
                            "timestamp": m.timestamp.to_rfc3339()
                        })
                    })
                    .collect();

                Ok(json!({
                    "found": true,
                    "id": c.id.to_string(),
                    "title": c.title,
                    "source": c.source.to_string(),
                    "messages": messages,
                    "url": c.url,
                    "created_at": c.created_at.to_rfc3339(),
                    "updated_at": c.updated_at.to_rfc3339()
                }))
            }
            None => Ok(json!({
                "found": false,
                "message": "Chat context not found"
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_memory_empty() {
        let tools = MemoryTools::in_memory().unwrap();
        
        let input = SearchMemoryInput {
            query: "test".to_string(),
            limit: None,
            content_types: None,
            sources: None,
        };

        let result = tools.search_memory(input).await.unwrap();
        assert!(result["results"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_save_context() {
        let tools = MemoryTools::in_memory().unwrap();
        
        let input = SaveContextInput {
            name: "Test Context".to_string(),
            content: "Important information".to_string(),
            tags: vec!["test".to_string()],
        };

        let result = tools.save_context(input).await.unwrap();
        assert!(result["success"].as_bool().unwrap());
        assert_eq!(result["name"], "Test Context");
    }

    #[tokio::test]
    async fn test_get_recent_empty() {
        let tools = MemoryTools::in_memory().unwrap();
        
        let input = GetRecentInput { limit: 10 };
        let result = tools.get_recent(input).await.unwrap();
        
        assert_eq!(result["count"], 0);
    }

    #[tokio::test]
    async fn test_list_sessions_empty() {
        let tools = MemoryTools::in_memory().unwrap();
        
        let result = tools.list_sessions().await.unwrap();
        assert_eq!(result["count"], 0);
    }
}
