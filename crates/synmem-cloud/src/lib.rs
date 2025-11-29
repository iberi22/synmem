//! SynMem Cloud Infrastructure
//!
//! This crate provides the cloud infrastructure components for SynMem premium features:
//! - Browser sessions in cloud (headless browsers)
//! - Session sync across devices
//! - REST API for programmatic access
//! - Usage analytics dashboard
//!
//! # Architecture
//! ```text
//! [User] → [API Gateway] → [Session Service]
//!                       → [Browser Pool]
//!                       → [Storage Service]
//! ```

pub mod api;
pub mod domain;
pub mod services;

pub use domain::{ApiKey, ApiScope, AuditLog, RateLimit, RateLimitHeaders, Tier, User};
