//! Core domain entities and ports for SynMem
//!
//! This crate contains the core domain logic, entities, and port definitions
//! that are used throughout the SynMem browser automation system.

pub mod domain;

pub use domain::entities::{ExtractedContent, ImageInfo, LinkInfo, Page, StructuredData};
