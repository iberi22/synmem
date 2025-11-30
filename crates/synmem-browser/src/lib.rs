//! SynMem Browser - Browser Driver Adapter
//!
//! This crate provides browser automation capabilities using chromiumoxide (pure Rust CDP).
//! It implements the `BrowserDriverPort` trait from synmem-core.

pub mod chromium;
pub mod parallel;

pub use chromium::ChromiumDriver;
