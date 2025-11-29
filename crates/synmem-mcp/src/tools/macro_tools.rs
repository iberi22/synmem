//! MCP tools for macro recording and playback
//!
//! Provides the following MCP tools:
//! - `record_macro`: Start recording a new macro
//! - `stop_recording`: Stop recording and save the macro
//! - `play_macro`: Execute a saved macro
//! - `list_macros`: List all saved macros

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use synmem_core::{BrowserAction, CoreError, MacroController, MacroInfo, MacroService, MacroStorage};

/// MCP tool definitions for macro management
pub struct MacroTools<S: MacroStorage + 'static> {
    service: Arc<MacroService<S>>,
}

impl<S: MacroStorage + 'static> MacroTools<S> {
    /// Create new macro tools with the given storage backend
    pub fn new(storage: S) -> Self {
        Self {
            service: Arc::new(MacroService::new(storage)),
        }
    }

    /// Get a reference to the underlying service
    pub fn service(&self) -> Arc<MacroService<S>> {
        Arc::clone(&self.service)
    }
}

// ----- Tool Input/Output Types -----

/// Input for the `record_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMacroInput {
    /// Name for the new macro
    pub name: String,
}

/// Output for the `record_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMacroOutput {
    /// Session ID for recording
    pub session_id: String,
    /// Message for the user
    pub message: String,
}

/// Input for the `stop_recording` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecordingInput {
    /// Session ID from `record_macro`
    pub session_id: String,
    /// Whether to optimize the recorded actions
    #[serde(default)]
    pub optimize: bool,
}

/// Output for the `stop_recording` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecordingOutput {
    /// ID of the saved macro
    pub macro_id: String,
    /// Name of the saved macro
    pub name: String,
    /// Number of actions recorded
    pub action_count: usize,
    /// Message for the user
    pub message: String,
}

/// Input for the `play_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayMacroInput {
    /// ID or name of the macro to play
    pub identifier: String,
    /// Whether to use name instead of ID
    #[serde(default)]
    pub by_name: bool,
}

/// Output for the `play_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayMacroOutput {
    /// Actions to be executed
    pub actions: Vec<BrowserAction>,
    /// Number of actions
    pub action_count: usize,
    /// Message for the user
    pub message: String,
}

/// Output for the `list_macros` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMacrosOutput {
    /// List of macros
    pub macros: Vec<MacroInfo>,
    /// Total count
    pub total: usize,
}

/// Input for recording a single action (internal use)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordActionInput {
    /// Session ID from `record_macro`
    pub session_id: String,
    /// The action to record
    pub action: BrowserAction,
}

/// Input for the `delete_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMacroInput {
    /// ID of the macro to delete
    pub macro_id: String,
}

/// Output for the `delete_macro` tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMacroOutput {
    /// Whether the macro was deleted
    pub deleted: bool,
    /// Message for the user
    pub message: String,
}

// ----- Tool Implementations -----

impl<S: MacroStorage + 'static> MacroTools<S> {
    /// Start recording a new macro
    ///
    /// MCP Tool: `record_macro`
    pub async fn record_macro(&self, input: RecordMacroInput) -> Result<RecordMacroOutput, CoreError> {
        let session_id = self.service.start_recording(&input.name).await?;

        Ok(RecordMacroOutput {
            session_id,
            message: format!("Started recording macro '{}'", input.name),
        })
    }

    /// Record a single action to the current session
    pub async fn record_action(&self, input: RecordActionInput) -> Result<(), CoreError> {
        self.service
            .record_action(&input.session_id, input.action)
            .await
    }

    /// Stop recording and save the macro
    ///
    /// MCP Tool: `stop_recording`
    pub async fn stop_recording(
        &self,
        input: StopRecordingInput,
    ) -> Result<StopRecordingOutput, CoreError> {
        let macro_data = self
            .service
            .stop_recording(&input.session_id, input.optimize)
            .await?;

        let action_count = macro_data.action_count();
        let message = if input.optimize {
            format!(
                "Saved optimized macro '{}' with {} action(s)",
                macro_data.name,
                action_count
            )
        } else {
            format!(
                "Saved macro '{}' with {} action(s)",
                macro_data.name,
                action_count
            )
        };

        Ok(StopRecordingOutput {
            macro_id: macro_data.id,
            name: macro_data.name,
            action_count,
            message,
        })
    }

    /// Execute a saved macro
    ///
    /// MCP Tool: `play_macro`
    pub async fn play_macro(&self, input: PlayMacroInput) -> Result<PlayMacroOutput, CoreError> {
        let actions = if input.by_name {
            self.service.play_macro_by_name(&input.identifier).await?
        } else {
            self.service.play_macro(&input.identifier).await?
        };

        let action_count = actions.len();
        Ok(PlayMacroOutput {
            actions,
            action_count,
            message: format!("Playing {} action(s)", action_count),
        })
    }

    /// List all saved macros
    ///
    /// MCP Tool: `list_macros`
    pub async fn list_macros(&self) -> Result<ListMacrosOutput, CoreError> {
        let macros = self.service.list_macros().await?;
        let total = macros.len();

        Ok(ListMacrosOutput { macros, total })
    }

    /// Delete a macro by ID
    ///
    /// MCP Tool: `delete_macro`
    pub async fn delete_macro(&self, input: DeleteMacroInput) -> Result<DeleteMacroOutput, CoreError> {
        let deleted = self.service.delete_macro(&input.macro_id).await?;

        let message = if deleted {
            format!("Deleted macro '{}'", input.macro_id)
        } else {
            format!("Macro '{}' not found", input.macro_id)
        };

        Ok(DeleteMacroOutput { deleted, message })
    }

    /// Cancel a recording session without saving
    pub async fn cancel_recording(&self, session_id: &str) -> Result<(), CoreError> {
        self.service.cancel_recording(session_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synmem_storage::MacroRepository;

    fn create_tools() -> MacroTools<MacroRepository> {
        let storage = MacroRepository::in_memory().unwrap();
        MacroTools::new(storage)
    }

    #[tokio::test]
    async fn test_record_and_stop() {
        let tools = create_tools();

        // Start recording
        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "test_macro".to_string(),
            })
            .await
            .unwrap();

        assert!(!record_output.session_id.is_empty());
        assert!(record_output.message.contains("test_macro"));

        // Record some actions
        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::navigate("https://example.com"),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::click("#btn"),
            })
            .await
            .unwrap();

        // Stop recording
        let stop_output = tools
            .stop_recording(StopRecordingInput {
                session_id: record_output.session_id,
                optimize: false,
            })
            .await
            .unwrap();

        assert_eq!(stop_output.name, "test_macro");
        assert_eq!(stop_output.action_count, 2);
    }

    #[tokio::test]
    async fn test_play_macro_by_id() {
        let tools = create_tools();

        // Create a macro
        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "test".to_string(),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::click("#btn"),
            })
            .await
            .unwrap();

        let stop_output = tools
            .stop_recording(StopRecordingInput {
                session_id: record_output.session_id,
                optimize: false,
            })
            .await
            .unwrap();

        // Play by ID
        let play_output = tools
            .play_macro(PlayMacroInput {
                identifier: stop_output.macro_id,
                by_name: false,
            })
            .await
            .unwrap();

        assert_eq!(play_output.action_count, 1);
    }

    #[tokio::test]
    async fn test_play_macro_by_name() {
        let tools = create_tools();

        // Create a macro
        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "my_macro".to_string(),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::click("#btn"),
            })
            .await
            .unwrap();

        tools
            .stop_recording(StopRecordingInput {
                session_id: record_output.session_id,
                optimize: false,
            })
            .await
            .unwrap();

        // Play by name
        let play_output = tools
            .play_macro(PlayMacroInput {
                identifier: "my_macro".to_string(),
                by_name: true,
            })
            .await
            .unwrap();

        assert_eq!(play_output.action_count, 1);
    }

    #[tokio::test]
    async fn test_list_macros() {
        let tools = create_tools();

        // Create some macros
        for name in ["macro1", "macro2"] {
            let record_output = tools
                .record_macro(RecordMacroInput {
                    name: name.to_string(),
                })
                .await
                .unwrap();

            tools
                .record_action(RecordActionInput {
                    session_id: record_output.session_id.clone(),
                    action: BrowserAction::click("#btn"),
                })
                .await
                .unwrap();

            tools
                .stop_recording(StopRecordingInput {
                    session_id: record_output.session_id,
                    optimize: false,
                })
                .await
                .unwrap();
        }

        // List macros
        let list_output = tools.list_macros().await.unwrap();
        assert_eq!(list_output.total, 2);
    }

    #[tokio::test]
    async fn test_delete_macro() {
        let tools = create_tools();

        // Create a macro
        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "to_delete".to_string(),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::click("#btn"),
            })
            .await
            .unwrap();

        let stop_output = tools
            .stop_recording(StopRecordingInput {
                session_id: record_output.session_id,
                optimize: false,
            })
            .await
            .unwrap();

        // Delete
        let delete_output = tools
            .delete_macro(DeleteMacroInput {
                macro_id: stop_output.macro_id,
            })
            .await
            .unwrap();

        assert!(delete_output.deleted);

        // List should be empty
        let list_output = tools.list_macros().await.unwrap();
        assert_eq!(list_output.total, 0);
    }

    #[tokio::test]
    async fn test_cancel_recording() {
        let tools = create_tools();

        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "to_cancel".to_string(),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::click("#btn"),
            })
            .await
            .unwrap();

        // Cancel
        tools
            .cancel_recording(&record_output.session_id)
            .await
            .unwrap();

        // List should be empty
        let list_output = tools.list_macros().await.unwrap();
        assert_eq!(list_output.total, 0);
    }

    #[tokio::test]
    async fn test_stop_with_optimization() {
        let tools = create_tools();

        let record_output = tools
            .record_macro(RecordMacroInput {
                name: "optimize_test".to_string(),
            })
            .await
            .unwrap();

        // Record redundant navigations
        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::navigate("https://first.com"),
            })
            .await
            .unwrap();

        tools
            .record_action(RecordActionInput {
                session_id: record_output.session_id.clone(),
                action: BrowserAction::navigate("https://second.com"),
            })
            .await
            .unwrap();

        // Stop with optimization
        let stop_output = tools
            .stop_recording(StopRecordingInput {
                session_id: record_output.session_id,
                optimize: true,
            })
            .await
            .unwrap();

        // Should have been optimized to 1 action
        assert_eq!(stop_output.action_count, 1);
        assert!(stop_output.message.contains("optimized"));
    }
}
