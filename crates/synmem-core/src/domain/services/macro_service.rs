//! Macro service implementation
//!
//! Implements the MacroController port for managing macro recording and playback.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::entities::{BrowserAction, Macro, MacroInfo};
use crate::error::CoreError;
use crate::ports::inbound::MacroController;
use crate::ports::outbound::MacroStorage;

/// In-progress recording session
struct RecordingSession {
    name: String,
    actions: Vec<BrowserAction>,
}

/// Service for managing macro recording and playback
pub struct MacroService<S: MacroStorage> {
    storage: S,
    sessions: Arc<RwLock<HashMap<String, RecordingSession>>>,
}

impl<S: MacroStorage> MacroService<S> {
    /// Create a new macro service with the given storage backend
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl<S: MacroStorage + 'static> MacroController for MacroService<S> {
    async fn start_recording(&self, name: &str) -> Result<String, CoreError> {
        if name.is_empty() {
            return Err(CoreError::InvalidInput("Macro name cannot be empty".into()));
        }

        let session_id = Uuid::new_v4().to_string();
        let session = RecordingSession {
            name: name.to_string(),
            actions: Vec::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    async fn record_action(
        &self,
        session_id: &str,
        action: BrowserAction,
    ) -> Result<(), CoreError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| CoreError::SessionNotFound(session_id.to_string()))?;

        session.actions.push(action);
        Ok(())
    }

    async fn stop_recording(&self, session_id: &str, optimize: bool) -> Result<Macro, CoreError> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .remove(session_id)
            .ok_or_else(|| CoreError::SessionNotFound(session_id.to_string()))?;

        let mut macro_data = Macro::new(session.name, session.actions);

        if optimize {
            macro_data.optimize();
        }

        self.storage.save(&macro_data).await?;
        Ok(macro_data)
    }

    async fn cancel_recording(&self, session_id: &str) -> Result<(), CoreError> {
        let mut sessions = self.sessions.write().await;
        if sessions.remove(session_id).is_none() {
            return Err(CoreError::SessionNotFound(session_id.to_string()));
        }
        Ok(())
    }

    async fn play_macro(&self, id: &str) -> Result<Vec<BrowserAction>, CoreError> {
        let macro_data = self
            .storage
            .get(id)
            .await?
            .ok_or_else(|| CoreError::MacroNotFound(id.to_string()))?;

        Ok(macro_data.actions)
    }

    async fn play_macro_by_name(&self, name: &str) -> Result<Vec<BrowserAction>, CoreError> {
        let macro_data = self
            .storage
            .get_by_name(name)
            .await?
            .ok_or_else(|| CoreError::MacroNotFound(name.to_string()))?;

        Ok(macro_data.actions)
    }

    async fn list_macros(&self) -> Result<Vec<MacroInfo>, CoreError> {
        self.storage.list().await
    }

    async fn delete_macro(&self, id: &str) -> Result<bool, CoreError> {
        self.storage.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Mock storage for testing
    struct MockStorage {
        macros: Arc<Mutex<HashMap<String, Macro>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                macros: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl MacroStorage for MockStorage {
        async fn save(&self, macro_data: &Macro) -> Result<(), CoreError> {
            let mut macros = self.macros.lock().unwrap();
            macros.insert(macro_data.id.clone(), macro_data.clone());
            Ok(())
        }

        async fn get(&self, id: &str) -> Result<Option<Macro>, CoreError> {
            let macros = self.macros.lock().unwrap();
            Ok(macros.get(id).cloned())
        }

        async fn get_by_name(&self, name: &str) -> Result<Option<Macro>, CoreError> {
            let macros = self.macros.lock().unwrap();
            Ok(macros.values().find(|m| m.name == name).cloned())
        }

        async fn delete(&self, id: &str) -> Result<bool, CoreError> {
            let mut macros = self.macros.lock().unwrap();
            Ok(macros.remove(id).is_some())
        }

        async fn list(&self) -> Result<Vec<MacroInfo>, CoreError> {
            let macros = self.macros.lock().unwrap();
            Ok(macros.values().map(MacroInfo::from).collect())
        }
    }

    #[tokio::test]
    async fn test_record_and_play_macro() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        // Start recording
        let session_id = service.start_recording("test_macro").await.unwrap();
        assert!(!session_id.is_empty());

        // Record actions
        service
            .record_action(&session_id, BrowserAction::navigate("https://example.com"))
            .await
            .unwrap();
        service
            .record_action(&session_id, BrowserAction::click("#btn"))
            .await
            .unwrap();

        // Stop recording
        let macro_data = service.stop_recording(&session_id, false).await.unwrap();
        assert_eq!(macro_data.name, "test_macro");
        assert_eq!(macro_data.action_count(), 2);

        // Play macro
        let actions = service.play_macro(&macro_data.id).await.unwrap();
        assert_eq!(actions.len(), 2);
    }

    #[tokio::test]
    async fn test_stop_recording_with_optimization() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        let session_id = service.start_recording("test").await.unwrap();

        // Record redundant navigations
        service
            .record_action(&session_id, BrowserAction::navigate("https://first.com"))
            .await
            .unwrap();
        service
            .record_action(&session_id, BrowserAction::navigate("https://second.com"))
            .await
            .unwrap();

        // Stop with optimization
        let macro_data = service.stop_recording(&session_id, true).await.unwrap();
        assert_eq!(macro_data.action_count(), 1);
    }

    #[tokio::test]
    async fn test_cancel_recording() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        let session_id = service.start_recording("test").await.unwrap();
        service
            .record_action(&session_id, BrowserAction::navigate("https://example.com"))
            .await
            .unwrap();

        // Cancel recording
        service.cancel_recording(&session_id).await.unwrap();

        // List should be empty
        let macros = service.list_macros().await.unwrap();
        assert!(macros.is_empty());
    }

    #[tokio::test]
    async fn test_list_and_delete_macros() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        // Create a macro
        let session_id = service.start_recording("test_macro").await.unwrap();
        service
            .record_action(&session_id, BrowserAction::navigate("https://example.com"))
            .await
            .unwrap();
        let macro_data = service.stop_recording(&session_id, false).await.unwrap();

        // List macros
        let macros = service.list_macros().await.unwrap();
        assert_eq!(macros.len(), 1);
        assert_eq!(macros[0].name, "test_macro");

        // Delete macro
        let deleted = service.delete_macro(&macro_data.id).await.unwrap();
        assert!(deleted);

        // List should be empty
        let macros = service.list_macros().await.unwrap();
        assert!(macros.is_empty());
    }

    #[tokio::test]
    async fn test_play_macro_by_name() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        // Create a macro
        let session_id = service.start_recording("my_macro").await.unwrap();
        service
            .record_action(&session_id, BrowserAction::click("#btn"))
            .await
            .unwrap();
        service.stop_recording(&session_id, false).await.unwrap();

        // Play by name
        let actions = service.play_macro_by_name("my_macro").await.unwrap();
        assert_eq!(actions.len(), 1);
    }

    #[tokio::test]
    async fn test_invalid_session_id() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        let result = service
            .record_action("invalid", BrowserAction::click("#btn"))
            .await;
        assert!(matches!(result, Err(CoreError::SessionNotFound(_))));
    }

    #[tokio::test]
    async fn test_empty_name_error() {
        let storage = MockStorage::new();
        let service = MacroService::new(storage);

        let result = service.start_recording("").await;
        assert!(matches!(result, Err(CoreError::InvalidInput(_))));
    }
}
