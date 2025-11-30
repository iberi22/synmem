//! Outbound ports - interfaces required by the application

mod browser_driver;
mod storage;
mod embedding;

pub use browser_driver::*;
pub use storage::*;
pub use embedding::*;
