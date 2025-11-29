//! Outbound ports (driven adapters interface)

mod browser_driver;
mod embedding;
mod storage;

pub use browser_driver::*;
pub use embedding::*;
pub use storage::*;
