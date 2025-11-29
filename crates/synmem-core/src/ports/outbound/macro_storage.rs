//! Macro storage port
//!
//! Defines the interface for storing and retrieving macros.

use async_trait::async_trait;

use crate::domain::entities::{Macro, MacroInfo};
use crate::error::CoreError;

/// Port for macro storage operations
#[async_trait]
pub trait MacroStorage: Send + Sync {
    /// Save a new macro to storage
    async fn save(&self, macro_data: &Macro) -> Result<(), CoreError>;

    /// Get a macro by its ID
    async fn get(&self, id: &str) -> Result<Option<Macro>, CoreError>;

    /// Get a macro by its name
    async fn get_by_name(&self, name: &str) -> Result<Option<Macro>, CoreError>;

    /// Delete a macro by its ID
    async fn delete(&self, id: &str) -> Result<bool, CoreError>;

    /// List all macros with basic info
    async fn list(&self) -> Result<Vec<MacroInfo>, CoreError>;
}
