//! SynMem Storage - Storage adapters
//!
//! This crate contains storage implementations for SynMem, including:
//! - SQLite storage for macros
//! - Database migrations

pub mod sqlite;

pub use sqlite::MacroRepository;
