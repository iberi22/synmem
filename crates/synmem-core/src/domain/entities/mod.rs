//! Domain Entities
//!
//! Core domain entities for the SynMem system.

mod browser_task;
mod memory;
mod scraped_page;
mod session;

pub use browser_task::{BrowserAction, BrowserTask, TaskStatus};
pub use memory::Memory;
pub use scraped_page::{PageMetadata, ScrapedPage};
pub use session::Session;
