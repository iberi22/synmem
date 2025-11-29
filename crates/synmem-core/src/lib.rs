//! SynMem Core - Domain and Application Layer
//!
//! This crate contains the core business logic for SynMem,
//! implementing semantic memory search with hybrid FTS + vector search.

pub mod domain;
pub mod ports;

pub use domain::entities::{Memory, SearchResult};
pub use domain::services::SearchService;
pub use ports::inbound::MemoryQuery;
pub use ports::outbound::{EmbeddingPort, StoragePort};
