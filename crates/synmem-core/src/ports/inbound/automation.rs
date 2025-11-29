//! # Automation Port
//!
//! Inbound port for macro recording and playback.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

/// Errors that can occur during automation operations.
#[derive(Debug, Error)]
pub enum AutomationError {
    #[error("recording failed: {0}")]
    RecordingFailed(String),

    #[error("playback failed: {0}")]
    PlaybackFailed(String),

    #[error("macro not found: {0}")]
    MacroNotFound(String),

    #[error("invalid action: {0}")]
    InvalidAction(String),

    #[error("already recording")]
    AlreadyRecording,

    #[error("not recording")]
    NotRecording,
}

/// Result type for automation operations.
pub type AutomationResult<T> = Result<T, AutomationError>;

/// An action recorded in a macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroAction {
    /// Navigate to a URL.
    Navigate { url: Url },
    /// Click on an element.
    Click { selector: String },
    /// Type text into an element.
    Type { selector: String, text: String },
    /// Wait for an element to appear.
    WaitForElement { selector: String, timeout_ms: u64 },
    /// Wait for a fixed duration.
    Wait { duration_ms: u64 },
    /// Scroll the page.
    Scroll { x: i32, y: i32 },
    /// Press a keyboard key.
    KeyPress { key: String },
}

/// A recorded macro (sequence of actions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Unique identifier.
    pub id: Uuid,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// The sequence of recorded actions.
    pub actions: Vec<MacroAction>,
    /// When this macro was created.
    pub created_at: DateTime<Utc>,
    /// When this macro was last modified.
    pub updated_at: DateTime<Utc>,
}

/// Options for recording a macro.
#[derive(Debug, Clone, Default)]
pub struct RecordOptions {
    /// Name for the macro.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}

/// Options for playing back a macro.
#[derive(Debug, Clone)]
pub struct PlaybackOptions {
    /// Speed multiplier (1.0 = normal, 2.0 = double speed).
    pub speed: f32,
    /// Stop on first error.
    pub stop_on_error: bool,
}

impl Default for PlaybackOptions {
    fn default() -> Self {
        Self {
            speed: 1.0,
            stop_on_error: true,
        }
    }
}

/// Result of macro playback.
#[derive(Debug, Clone)]
pub struct PlaybackResult {
    /// Whether playback completed successfully.
    pub success: bool,
    /// Number of actions executed.
    pub actions_executed: usize,
    /// Error message if playback failed.
    pub error: Option<String>,
    /// Duration of playback in milliseconds.
    pub duration_ms: u64,
}

/// Inbound port for macro recording and playback.
///
/// This port defines the interface for recording sequences of browser
/// actions and playing them back.
#[async_trait]
pub trait AutomationPort: Send + Sync {
    /// Start recording a new macro.
    async fn record_macro(&self, options: RecordOptions) -> AutomationResult<()>;

    /// Stop recording and save the macro.
    async fn stop_recording(&self) -> AutomationResult<Macro>;

    /// Play back a recorded macro.
    async fn play_macro(
        &self,
        macro_id: &Uuid,
        options: PlaybackOptions,
    ) -> AutomationResult<PlaybackResult>;

    /// List all saved macros.
    async fn list_macros(&self) -> AutomationResult<Vec<Macro>>;

    /// Delete a macro.
    async fn delete_macro(&self, macro_id: &Uuid) -> AutomationResult<()>;
}
