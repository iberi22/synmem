//! ChatContext entity for AI chat conversations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an AI chat conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContext {
    /// Unique identifier
    pub id: Uuid,

    /// Chat title or topic
    pub title: String,

    /// Source AI (gemini, chatgpt, claude, etc.)
    pub source: ChatSource,

    /// Conversation messages
    pub messages: Vec<ChatMessage>,

    /// When the chat was created
    pub created_at: DateTime<Utc>,

    /// When the chat was last updated
    pub updated_at: DateTime<Utc>,

    /// URL to the chat if available
    pub url: Option<String>,

    /// Session ID this chat belongs to
    pub session_id: Option<Uuid>,
}

/// Source AI for a chat conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatSource {
    Gemini,
    ChatGPT,
    Claude,
    Other(String),
}

/// A single message in a chat conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role (user, assistant, system)
    pub role: MessageRole,

    /// Message content
    pub content: String,

    /// Timestamp of the message
    pub timestamp: DateTime<Utc>,
}

/// Role of a chat message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl ChatContext {
    /// Creates a new ChatContext
    pub fn new(title: String, source: ChatSource) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            source,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            url: None,
            session_id: None,
        }
    }

    /// Adds a message to the conversation
    pub fn add_message(&mut self, role: MessageRole, content: String) {
        self.messages.push(ChatMessage {
            role,
            content,
            timestamp: Utc::now(),
        });
        self.updated_at = Utc::now();
    }

    /// Sets the URL
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the session ID
    pub fn with_session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Gets the full conversation as text
    pub fn to_text(&self) -> String {
        self.messages
            .iter()
            .map(|m| {
                let role = match m.role {
                    MessageRole::User => "User",
                    MessageRole::Assistant => "Assistant",
                    MessageRole::System => "System",
                };
                format!("{}: {}", role, m.content)
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl std::fmt::Display for ChatSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatSource::Gemini => write!(f, "gemini"),
            ChatSource::ChatGPT => write!(f, "chatgpt"),
            ChatSource::Claude => write!(f, "claude"),
            ChatSource::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for ChatSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gemini" => Ok(ChatSource::Gemini),
            "chatgpt" | "gpt" | "openai" => Ok(ChatSource::ChatGPT),
            "claude" | "anthropic" => Ok(ChatSource::Claude),
            other => Ok(ChatSource::Other(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_context_creation() {
        let mut chat = ChatContext::new("Test Chat".to_string(), ChatSource::Gemini);

        chat.add_message(MessageRole::User, "Hello!".to_string());
        chat.add_message(MessageRole::Assistant, "Hi there!".to_string());

        assert_eq!(chat.title, "Test Chat");
        assert_eq!(chat.source, ChatSource::Gemini);
        assert_eq!(chat.messages.len(), 2);
    }

    #[test]
    fn test_chat_to_text() {
        let mut chat = ChatContext::new("Test".to_string(), ChatSource::Claude);
        chat.add_message(MessageRole::User, "Hello".to_string());
        chat.add_message(MessageRole::Assistant, "Hi!".to_string());

        let text = chat.to_text();
        assert!(text.contains("User: Hello"));
        assert!(text.contains("Assistant: Hi!"));
    }
}
