//! Embeddings module containing adapters for different embedding providers.

#[cfg(feature = "fastembed")]
pub mod fastembed_adapter;

pub mod mock_adapter;
