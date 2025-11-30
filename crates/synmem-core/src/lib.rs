//! SynMem Core - Domain and Application Layer
//!
//! This crate implements the core business logic for SynMem, including:
//! - Session management with encryption
//! - Cookie handling with AES-256-GCM encryption
//! - Master key derivation with Argon2
//! - Secure memory handling (zeroize on drop)
//! - Domain entities (BrowserTask, ScrapedPage, Session)
//! - Hexagonal architecture ports (inbound/outbound)

pub mod domain;
pub mod ports;

// Re-export domain and ports
pub use domain::*;
pub use ports::*;

// Session management exports
pub use domain::entities::session::{Session, SessionProfile, EncryptedCookie};
pub use domain::services::session_manager::{SessionManager, SessionManagerConfig};
pub use domain::services::crypto::{CryptoService, MasterKey};
