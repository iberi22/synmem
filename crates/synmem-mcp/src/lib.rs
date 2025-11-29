//! # SynMem MCP Server
//!
//! MCP (Model Context Protocol) server implementation for SynMem browser automation.
//! This crate provides tools for browser navigation, scraping, and memory management.

pub mod server;
pub mod tools;
pub mod resources;

pub use server::SynMemMcpServer;
