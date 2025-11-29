//! SynMem MCP Server - Memory and Search Tools
//!
//! This crate provides the MCP (Model Context Protocol) server implementation
//! for SynMem, exposing memory and search tools to AI assistants.

pub mod server;
pub mod tools;

pub use server::SynMemServer;
