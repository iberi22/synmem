//! SynMem CLI - Command Line Interface
//!
//! The main entry point for the SynMem synthetic memory browser agent.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "synmem=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("SynMem v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Synthetic Memory Browser Agent initialized");

    Ok(())
}
