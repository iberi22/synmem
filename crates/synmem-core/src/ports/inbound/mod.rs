//! # Inbound Ports (Driving)
//!
//! These ports define how external actors can interact with the SynMem system.

mod automation;
mod browser_control;
mod memory_query;
mod scraper;

pub use automation::AutomationPort;
pub use browser_control::BrowserControlPort;
pub use memory_query::MemoryQueryPort;
pub use scraper::ScraperPort;
