//! SynMem Storage - SQLite storage adapter with FTS5
//!
//! This crate provides the storage implementation for SynMem using SQLite
//! with Full-Text Search (FTS5) for semantic search capabilities.

pub mod sqlite;

pub use sqlite::SqliteStorage;
