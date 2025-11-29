//! SynMem Storage - Storage Adapters
//!
//! This crate contains storage adapters including embeddings support.
//!
//! # Features
//!
//! - `fastembed`: Enable local embedding generation using fastembed (requires ONNX runtime).
//!   Uses the all-MiniLM-L6-v2 model by default.
//!
//! # Example
//!
//! Using the mock adapter for testing:
//!
//! ```rust
//! use synmem_storage::embeddings::mock_adapter::MockEmbedAdapter;
//! use synmem_core::EmbeddingPort;
//!
//! let adapter = MockEmbedAdapter::default();
//! let embedding = adapter.embed("Hello, world!").unwrap();
//! assert_eq!(embedding.dimension, 384);
//! ```
//!
//! Using FastEmbed (requires `fastembed` feature):
//!
//! ```ignore
//! use synmem_storage::FastEmbedAdapter;
//! use synmem_core::EmbeddingPort;
//!
//! let adapter = FastEmbedAdapter::new().unwrap();
//! let embedding = adapter.embed("Hello, world!").unwrap();
//! ```

pub mod embeddings;

#[cfg(feature = "fastembed")]
pub use embeddings::fastembed_adapter::FastEmbedAdapter;

pub use embeddings::mock_adapter::MockEmbedAdapter;
