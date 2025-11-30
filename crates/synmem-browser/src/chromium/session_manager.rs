//! Session manager for storing and retrieving browser state

use std::collections::HashMap;
use std::sync::RwLock;
use synmem_core::domain::entities::BrowserState;

/// Manages browser state in memory
pub struct BrowserStateManager {
    states: RwLock<HashMap<String, BrowserState>>,
}

impl BrowserStateManager {
    /// Create a new browser state manager
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    /// Save a browser state
    pub fn save(&self, state: &BrowserState) {
        let mut states = self.states.write().unwrap();
        states.insert(state.id.clone(), state.clone());
    }

    /// Get a browser state by ID
    pub fn get(&self, id: &str) -> Option<BrowserState> {
        let states = self.states.read().unwrap();
        states.get(id).cloned()
    }

    /// List all state IDs
    pub fn list(&self) -> Vec<String> {
        let states = self.states.read().unwrap();
        states.keys().cloned().collect()
    }

    /// Delete a browser state by ID
    pub fn delete(&self, id: &str) -> Option<BrowserState> {
        let mut states = self.states.write().unwrap();
        states.remove(id)
    }
}

impl Default for BrowserStateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_state_manager() {
        let manager = BrowserStateManager::new();
        let state = BrowserState::new("test-session").with_name("Test Session");

        manager.save(&state);
        
        let retrieved = manager.get("test-session");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Session");

        let ids = manager.list();
        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&"test-session".to_string()));

        let deleted = manager.delete("test-session");
        assert!(deleted.is_some());
        assert!(manager.get("test-session").is_none());
    }
}
