//! Inbound ports - interfaces provided by the application

mod browser_control;
mod scraper;
mod memory_query;

pub use browser_control::*;
pub use scraper::*;
pub use memory_query::*;
