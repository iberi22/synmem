//! SynMem Core - Domain and Application Layer
//!
//! This crate contains the core domain logic and ports for the SynMem application.

pub mod domain;
pub mod ports;

pub use domain::entities::{
    ChatContext, ChatMessage, ChatSource, ContentType, Memory, MessageRole, SavedContext,
    ScrapedPage, SearchResult, Session,
};
pub use domain::services::memory_service::MemoryService;
pub use ports::inbound::memory_query::{MemoryQueryError, MemoryQueryPort};
pub use ports::outbound::storage::{StorageError, StoragePort};
