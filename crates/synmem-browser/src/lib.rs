//! Browser automation and DOM extraction for SynMem
//!
//! This crate provides high-performance DOM extraction with Rayon parallelization
//! for processing web pages.

pub mod chromium;
pub mod parallel;

pub use chromium::dom_extractor::DomExtractor;
pub use parallel::batch_processor::BatchProcessor;
