//! Macro controller port
//!
//! Defines the inbound interface for macro recording and playback.

use async_trait::async_trait;

use crate::domain::entities::{BrowserAction, Macro, MacroInfo};
use crate::error::CoreError;

/// Port for macro recording and playback operations (inbound)
#[async_trait]
pub trait MacroController: Send + Sync {
    /// Start recording a new macro with the given name
    async fn start_recording(&self, name: &str) -> Result<String, CoreError>;

    /// Record an action to the current recording session
    async fn record_action(&self, session_id: &str, action: BrowserAction)
        -> Result<(), CoreError>;

    /// Stop recording and save the macro
    async fn stop_recording(&self, session_id: &str, optimize: bool)
        -> Result<Macro, CoreError>;

    /// Cancel the current recording session without saving
    async fn cancel_recording(&self, session_id: &str) -> Result<(), CoreError>;

    /// Play a macro by ID
    async fn play_macro(&self, id: &str) -> Result<Vec<BrowserAction>, CoreError>;

    /// Play a macro by name
    async fn play_macro_by_name(&self, name: &str) -> Result<Vec<BrowserAction>, CoreError>;

    /// List all available macros
    async fn list_macros(&self) -> Result<Vec<MacroInfo>, CoreError>;

    /// Delete a macro by ID
    async fn delete_macro(&self, id: &str) -> Result<bool, CoreError>;
}
