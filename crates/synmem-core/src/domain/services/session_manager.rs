//! Session Manager service for secure cookie handling
//!
//! This module provides:
//! - Secure session creation with encrypted cookies
//! - Session persistence (save/load to disk)
//! - Multiple profile support
//! - Auto-refresh for expired sessions

use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::session::{Cookie, Session, SessionProfile};
use crate::domain::services::crypto::{CryptoError, CryptoService, MasterKey};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Errors that can occur during session management
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    NotFound(String),
    
    #[error("Session has expired")]
    Expired,
    
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(#[from] CryptoError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Profile already exists: {0}")]
    ProfileExists(String),
    
    #[error("Invalid password")]
    InvalidPassword,
}

/// Configuration for SessionManager
#[derive(Debug, Clone)]
pub struct SessionManagerConfig {
    /// Directory where session files are stored
    pub storage_path: PathBuf,
    /// Default session lifetime in days
    pub default_lifetime_days: i64,
    /// Whether to auto-refresh sessions nearing expiration
    pub auto_refresh: bool,
}

impl Default for SessionManagerConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from(".synmem/sessions"),
            default_lifetime_days: 30,
            auto_refresh: true,
        }
    }
}

/// Session storage file format
#[derive(Debug, Serialize, Deserialize)]
struct SessionStorage {
    /// All stored session profiles
    profiles: HashMap<String, SessionProfile>,
    /// Version for future migrations
    version: u32,
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
            version: 1,
        }
    }
}

/// Session manager for secure cookie handling
pub struct SessionManager {
    config: SessionManagerConfig,
    crypto: CryptoService,
    /// In-memory cache of loaded sessions
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    /// Create a new SessionManager with the given configuration
    pub fn new(config: SessionManagerConfig) -> Self {
        Self {
            config,
            crypto: CryptoService::new(),
            sessions: HashMap::new(),
        }
    }

    /// Create a new session with encrypted cookies
    pub fn create_session(
        &mut self,
        profile_name: &str,
        cookies: Vec<Cookie>,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError> {
        // Generate a unique salt for this session
        let salt = self.crypto.generate_salt()?;
        let salt_b64 = BASE64.encode(&salt);
        
        // Derive the master key from password
        let key = MasterKey::derive(master_password, &salt)?;
        
        // Serialize cookies to JSON
        let cookies_json = serde_json::to_vec(&cookies)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        
        // Encrypt the cookies
        let (encrypted_data, nonce) = self.crypto.encrypt_to_base64(&cookies_json, &key)?;
        
        // Create the session profile
        let now = Utc::now();
        let profile = SessionProfile {
            id: Uuid::new_v4(),
            profile: profile_name.to_string(),
            encrypted_cookies: encrypted_data,
            nonce,
            salt: salt_b64,
            created_at: now,
            expires_at: now + Duration::days(self.config.default_lifetime_days),
            last_refreshed: None,
        };
        
        // Store in memory cache
        let session = Session::new(profile.clone(), cookies);
        self.sessions.insert(profile_name.to_string(), session);
        
        Ok(profile)
    }

    /// Load a session from storage and decrypt it
    pub fn load_session(
        &mut self,
        profile_name: &str,
        master_password: &str,
    ) -> Result<&Session, SessionError> {
        // Check if already loaded and not expired
        if self.sessions.contains_key(profile_name) {
            let session = self.sessions.get(profile_name).unwrap();
            if session.profile.is_expired() {
                return Err(SessionError::Expired);
            }
            return Ok(self.sessions.get(profile_name).unwrap());
        }
        
        // Load from storage
        let storage = self.load_storage()?;
        let profile = storage
            .profiles
            .get(profile_name)
            .ok_or_else(|| SessionError::NotFound(profile_name.to_string()))?
            .clone();
        
        // Check if expired
        if profile.is_expired() {
            return Err(SessionError::Expired);
        }
        
        // Decrypt the cookies
        let salt = BASE64.decode(&profile.salt)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))?;
        
        let key = MasterKey::derive(master_password, &salt)?;
        
        let cookies_json = self.crypto.decrypt_from_base64(
            &profile.encrypted_cookies,
            &profile.nonce,
            &key,
        ).map_err(|_| SessionError::InvalidPassword)?;
        
        let cookies: Vec<Cookie> = serde_json::from_slice(&cookies_json)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        
        // Store in memory cache
        let session = Session::new(profile, cookies);
        self.sessions.insert(profile_name.to_string(), session);
        
        Ok(self.sessions.get(profile_name).unwrap())
    }

    /// Save a session profile to storage
    pub fn save_session(&self, profile: &SessionProfile) -> Result<(), SessionError> {
        let mut storage = self.load_storage().unwrap_or_default();
        storage.profiles.insert(profile.profile.clone(), profile.clone());
        self.save_storage(&storage)
    }

    /// List all stored session profiles (without decrypting)
    pub fn list_profiles(&self) -> Result<Vec<String>, SessionError> {
        let storage = self.load_storage().unwrap_or_default();
        Ok(storage.profiles.keys().cloned().collect())
    }

    /// Get profile info without decrypting
    pub fn get_profile_info(&self, profile_name: &str) -> Result<SessionProfile, SessionError> {
        let storage = self.load_storage()?;
        storage
            .profiles
            .get(profile_name)
            .cloned()
            .ok_or_else(|| SessionError::NotFound(profile_name.to_string()))
    }

    /// Delete a session profile
    pub fn delete_profile(&mut self, profile_name: &str) -> Result<(), SessionError> {
        // Remove from memory
        self.sessions.remove(profile_name);
        
        // Remove from storage
        let mut storage = self.load_storage().unwrap_or_default();
        storage.profiles.remove(profile_name);
        self.save_storage(&storage)
    }

    /// Refresh a session (re-encrypt with new nonce, update timestamps)
    pub fn refresh_session(
        &mut self,
        profile_name: &str,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError> {
        // Get the current session
        let session = self.sessions.get(profile_name)
            .ok_or_else(|| SessionError::NotFound(profile_name.to_string()))?;
        
        // Get cookies
        let cookies = session.cookies().to_vec();
        let mut profile = session.profile.clone();
        
        // Remove from memory (will be re-added)
        self.sessions.remove(profile_name);
        
        // Generate new nonce
        let salt = BASE64.decode(&profile.salt)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))?;
        let key = MasterKey::derive(master_password, &salt)?;
        
        // Re-encrypt with new nonce
        let cookies_json = serde_json::to_vec(&cookies)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        let (encrypted_data, nonce) = self.crypto.encrypt_to_base64(&cookies_json, &key)?;
        
        // Update profile
        let now = Utc::now();
        profile.encrypted_cookies = encrypted_data;
        profile.nonce = nonce;
        profile.last_refreshed = Some(now);
        profile.expires_at = now + Duration::days(self.config.default_lifetime_days);
        
        // Store in memory
        let new_session = Session::new(profile.clone(), cookies);
        self.sessions.insert(profile_name.to_string(), new_session);
        
        // Save to storage
        self.save_session(&profile)?;
        
        Ok(profile)
    }

    /// Check all sessions and refresh those that need it
    pub fn auto_refresh_sessions(&mut self, master_password: &str) -> Result<Vec<String>, SessionError> {
        if !self.config.auto_refresh {
            return Ok(vec![]);
        }
        
        let mut refreshed = vec![];
        let profile_names: Vec<String> = self.sessions.keys().cloned().collect();
        
        for profile_name in profile_names {
            if let Some(session) = self.sessions.get(&profile_name) {
                if session.profile.needs_refresh() {
                    match self.refresh_session(&profile_name, master_password) {
                        Ok(_) => refreshed.push(profile_name),
                        Err(e) => {
                            tracing::warn!("Failed to refresh session {}: {}", profile_name, e);
                        }
                    }
                }
            }
        }
        
        Ok(refreshed)
    }

    /// Update cookies in an existing session
    pub fn update_cookies(
        &mut self,
        profile_name: &str,
        cookies: Vec<Cookie>,
        master_password: &str,
    ) -> Result<SessionProfile, SessionError> {
        let profile = self.get_profile_info(profile_name)?;
        
        // Decrypt with existing salt
        let salt = BASE64.decode(&profile.salt)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))?;
        let key = MasterKey::derive(master_password, &salt)?;
        
        // Re-encrypt with new cookies
        let cookies_json = serde_json::to_vec(&cookies)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        let (encrypted_data, nonce) = self.crypto.encrypt_to_base64(&cookies_json, &key)?;
        
        // Update profile
        let mut updated_profile = profile;
        updated_profile.encrypted_cookies = encrypted_data;
        updated_profile.nonce = nonce;
        updated_profile.last_refreshed = Some(Utc::now());
        
        // Update in memory
        let session = Session::new(updated_profile.clone(), cookies);
        self.sessions.insert(profile_name.to_string(), session);
        
        // Save to storage
        self.save_session(&updated_profile)?;
        
        Ok(updated_profile)
    }

    /// Get a reference to a loaded session
    pub fn get_session(&self, profile_name: &str) -> Option<&Session> {
        self.sessions.get(profile_name)
    }

    /// Get a mutable reference to a loaded session
    pub fn get_session_mut(&mut self, profile_name: &str) -> Option<&mut Session> {
        self.sessions.get_mut(profile_name)
    }

    /// Unload a session from memory (zeroize sensitive data)
    pub fn unload_session(&mut self, profile_name: &str) {
        // Session will be zeroized on drop due to Zeroize derive
        self.sessions.remove(profile_name);
    }

    /// Unload all sessions from memory
    pub fn unload_all_sessions(&mut self) {
        self.sessions.clear();
    }

    // Private helper methods

    fn storage_file_path(&self) -> PathBuf {
        self.config.storage_path.join("sessions.json")
    }

    fn load_storage(&self) -> Result<SessionStorage, SessionError> {
        let path = self.storage_file_path();
        if !path.exists() {
            return Ok(SessionStorage::default());
        }
        
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| SessionError::SerializationError(e.to_string()))
    }

    fn save_storage(&self, storage: &SessionStorage) -> Result<(), SessionError> {
        let path = self.storage_file_path();
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(storage)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        std::fs::write(&path, content)?;
        
        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(SessionManagerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_manager() -> (SessionManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config = SessionManagerConfig {
            storage_path: temp_dir.path().to_path_buf(),
            default_lifetime_days: 30,
            auto_refresh: true,
        };
        (SessionManager::new(config), temp_dir)
    }

    fn create_test_cookies() -> Vec<Cookie> {
        vec![
            Cookie {
                name: "session_id".to_string(),
                value: "abc123xyz789".to_string(),
                domain: ".example.com".to_string(),
                path: "/".to_string(),
                secure: true,
                http_only: true,
                same_site: Some("Lax".to_string()),
                expires: Some(Utc::now() + Duration::days(30)),
            },
            Cookie {
                name: "auth_token".to_string(),
                value: "secret_token_value".to_string(),
                domain: ".example.com".to_string(),
                path: "/api".to_string(),
                secure: true,
                http_only: true,
                same_site: Some("Strict".to_string()),
                expires: Some(Utc::now() + Duration::days(1)),
            },
        ]
    }

    #[test]
    fn test_create_session() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "my_secure_password";
        
        let profile = manager.create_session("test-profile", cookies, password).unwrap();
        
        assert_eq!(profile.profile, "test-profile");
        assert!(!profile.encrypted_cookies.is_empty());
        assert!(!profile.nonce.is_empty());
        assert!(!profile.salt.is_empty());
        assert!(!profile.is_expired());
    }

    #[test]
    fn test_save_and_load_session() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "my_secure_password";
        
        // Create and save
        let profile = manager.create_session("test-profile", cookies.clone(), password).unwrap();
        manager.save_session(&profile).unwrap();
        
        // Clear memory cache
        manager.unload_all_sessions();
        
        // Load from storage
        let session = manager.load_session("test-profile", password).unwrap();
        
        assert_eq!(session.cookies().len(), 2);
        assert_eq!(session.cookies()[0].name, "session_id");
        assert_eq!(session.cookies()[0].value, "abc123xyz789");
    }

    #[test]
    fn test_wrong_password_fails() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        
        let profile = manager.create_session("test-profile", cookies, "correct_password").unwrap();
        manager.save_session(&profile).unwrap();
        manager.unload_all_sessions();
        
        let result = manager.load_session("test-profile", "wrong_password");
        assert!(matches!(result, Err(SessionError::InvalidPassword)));
    }

    #[test]
    fn test_list_profiles() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "password";
        
        let profile1 = manager.create_session("profile-1", cookies.clone(), password).unwrap();
        let profile2 = manager.create_session("profile-2", cookies.clone(), password).unwrap();
        manager.save_session(&profile1).unwrap();
        manager.save_session(&profile2).unwrap();
        
        let profiles = manager.list_profiles().unwrap();
        assert_eq!(profiles.len(), 2);
        assert!(profiles.contains(&"profile-1".to_string()));
        assert!(profiles.contains(&"profile-2".to_string()));
    }

    #[test]
    fn test_delete_profile() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "password";
        
        let profile = manager.create_session("to-delete", cookies, password).unwrap();
        manager.save_session(&profile).unwrap();
        
        assert!(manager.list_profiles().unwrap().contains(&"to-delete".to_string()));
        
        manager.delete_profile("to-delete").unwrap();
        
        assert!(!manager.list_profiles().unwrap().contains(&"to-delete".to_string()));
    }

    #[test]
    fn test_refresh_session() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "password";
        
        let original_profile = manager.create_session("refresh-test", cookies, password).unwrap();
        let original_nonce = original_profile.nonce.clone();
        
        let refreshed_profile = manager.refresh_session("refresh-test", password).unwrap();
        
        // Nonce should be different after refresh
        assert_ne!(refreshed_profile.nonce, original_nonce);
        // Should have refresh timestamp
        assert!(refreshed_profile.last_refreshed.is_some());
    }

    #[test]
    fn test_update_cookies() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "password";
        
        let profile = manager.create_session("update-test", cookies, password).unwrap();
        manager.save_session(&profile).unwrap();
        
        // Create new cookies
        let new_cookies = vec![Cookie {
            name: "new_cookie".to_string(),
            value: "new_value".to_string(),
            domain: ".example.com".to_string(),
            path: "/".to_string(),
            secure: true,
            http_only: true,
            same_site: None,
            expires: None,
        }];
        
        manager.update_cookies("update-test", new_cookies, password).unwrap();
        
        let session = manager.get_session("update-test").unwrap();
        assert_eq!(session.cookies().len(), 1);
        assert_eq!(session.cookies()[0].name, "new_cookie");
    }

    #[test]
    fn test_session_not_found() {
        let (mut manager, _temp_dir) = create_test_manager();
        
        let result = manager.load_session("nonexistent", "password");
        assert!(matches!(result, Err(SessionError::NotFound(_))));
    }

    #[test]
    fn test_unload_session() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        let password = "password";
        
        manager.create_session("unload-test", cookies, password).unwrap();
        assert!(manager.get_session("unload-test").is_some());
        
        manager.unload_session("unload-test");
        assert!(manager.get_session("unload-test").is_none());
    }

    #[test]
    fn test_multiple_profiles_different_passwords() {
        let (mut manager, _temp_dir) = create_test_manager();
        let cookies = create_test_cookies();
        
        let profile1 = manager.create_session("profile-1", cookies.clone(), "password1").unwrap();
        let profile2 = manager.create_session("profile-2", cookies.clone(), "password2").unwrap();
        manager.save_session(&profile1).unwrap();
        manager.save_session(&profile2).unwrap();
        manager.unload_all_sessions();
        
        // Each profile can only be loaded with its own password
        assert!(manager.load_session("profile-1", "password1").is_ok());
        manager.unload_all_sessions();
        
        assert!(manager.load_session("profile-2", "password2").is_ok());
        manager.unload_all_sessions();
        
        // Wrong passwords should fail
        assert!(manager.load_session("profile-1", "password2").is_err());
        assert!(manager.load_session("profile-2", "password1").is_err());
    }
}
