//! Inbound ports - interfaces provided by the application

pub mod session_control;

mod browser_control;
mod scraper;
mod memory_query;

pub use session_control::SessionControlPort;
pub use browser_control::*;
pub use scraper::*;
pub use memory_query::*;
