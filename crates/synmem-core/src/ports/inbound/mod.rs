//! Inbound ports (driving adapters interface)

mod browser_control;
mod memory_query;
mod scraper;

pub use browser_control::*;
pub use memory_query::*;
pub use scraper::*;
