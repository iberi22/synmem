//! Chromium driver module

mod driver;
mod session_manager;
mod dom_extractor;
mod error;

pub use driver::ChromiumDriver;
pub use error::ChromiumError;
