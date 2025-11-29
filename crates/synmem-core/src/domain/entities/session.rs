//! Session entity for browser credential management
//!
//! This module defines the core session data structures for storing
//! encrypted browser cookies and credentials securely.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Represents a browser cookie with all its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    /// Cookie name
    pub name: String,
    /// Cookie value (sensitive - zeroized on drop)
    pub value: String,
    /// Domain for which the cookie is valid
    pub domain: String,
    /// Path for which the cookie is valid
    pub path: String,
    /// Whether the cookie is secure (HTTPS only)
    pub secure: bool,
    /// Whether the cookie is HTTP-only
    pub http_only: bool,
    /// Same-site policy
    pub same_site: Option<String>,
    /// Expiration timestamp
    pub expires: Option<DateTime<Utc>>,
}

impl Zeroize for Cookie {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.value.zeroize();
        self.domain.zeroize();
        self.path.zeroize();
        self.same_site.take();
    }
}

impl Drop for Cookie {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Encrypted cookie container stored on disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCookie {
    /// Base64-encoded encrypted cookie data
    pub encrypted_data: String,
    /// Base64-encoded nonce used for encryption
    pub nonce: String,
}

/// Session profile containing encrypted browser credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProfile {
    /// Unique identifier for this profile
    pub id: uuid::Uuid,
    /// Human-readable profile name (e.g., "twitter-main")
    pub profile: String,
    /// Encrypted cookies data (base64-encoded)
    pub encrypted_cookies: String,
    /// Base64-encoded nonce used for AES-GCM encryption
    pub nonce: String,
    /// Base64-encoded salt used for Argon2 key derivation
    pub salt: String,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// Last time the session was refreshed
    pub last_refreshed: Option<DateTime<Utc>>,
}

impl SessionProfile {
    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the session needs refresh (within 10% of expiration)
    pub fn needs_refresh(&self) -> bool {
        let total_duration = self.expires_at.signed_duration_since(self.created_at);
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        
        // Refresh if 90% of the session lifetime has passed
        elapsed > total_duration * 9 / 10
    }
}

/// Active session with decrypted cookies in memory
#[derive(Debug)]
pub struct Session {
    /// Profile information
    pub profile: SessionProfile,
    /// Decrypted cookies (only held in memory, zeroized on drop)
    cookies: Vec<Cookie>,
}

impl Session {
    /// Create a new session with decrypted cookies
    pub fn new(profile: SessionProfile, cookies: Vec<Cookie>) -> Self {
        Self { profile, cookies }
    }

    /// Get a reference to the decrypted cookies
    pub fn cookies(&self) -> &[Cookie] {
        &self.cookies
    }

    /// Get a mutable reference to the cookies
    pub fn cookies_mut(&mut self) -> &mut Vec<Cookie> {
        &mut self.cookies
    }

    /// Find a cookie by name and domain (exact domain match for security)
    pub fn find_cookie(&self, name: &str, domain: &str) -> Option<&Cookie> {
        self.cookies.iter().find(|c| c.name == name && c.domain == domain)
    }

    /// Remove expired cookies
    pub fn remove_expired_cookies(&mut self) {
        let now = Utc::now();
        self.cookies.retain(|c| {
            c.expires.map(|exp| exp > now).unwrap_or(true)
        });
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Explicitly zeroize all cookies when session is dropped
        for cookie in &mut self.cookies {
            cookie.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_cookie() -> Cookie {
        Cookie {
            name: "session_id".to_string(),
            value: "secret_value_12345".to_string(),
            domain: ".example.com".to_string(),
            path: "/".to_string(),
            secure: true,
            http_only: true,
            same_site: Some("Lax".to_string()),
            expires: Some(Utc::now() + Duration::days(30)),
        }
    }

    fn create_test_profile() -> SessionProfile {
        SessionProfile {
            id: uuid::Uuid::new_v4(),
            profile: "test-profile".to_string(),
            encrypted_cookies: "encrypted_data_here".to_string(),
            nonce: "nonce_here".to_string(),
            salt: "salt_here".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(30),
            last_refreshed: None,
        }
    }

    #[test]
    fn test_session_profile_not_expired() {
        let profile = create_test_profile();
        assert!(!profile.is_expired());
    }

    #[test]
    fn test_session_profile_expired() {
        let mut profile = create_test_profile();
        profile.expires_at = Utc::now() - Duration::hours(1);
        assert!(profile.is_expired());
    }

    #[test]
    fn test_session_needs_refresh() {
        let mut profile = create_test_profile();
        // Set created_at to 28 days ago with expires_at still 30 days from created_at
        // This means only 2 days are left (less than 10% of 30 days = 3 days)
        profile.created_at = Utc::now() - Duration::days(28);
        profile.expires_at = profile.created_at + Duration::days(30); // 2 days from now
        assert!(profile.needs_refresh());
    }

    #[test]
    fn test_session_no_refresh_needed() {
        let profile = create_test_profile();
        assert!(!profile.needs_refresh());
    }

    #[test]
    fn test_session_find_cookie() {
        let profile = create_test_profile();
        let cookies = vec![create_test_cookie()];
        let session = Session::new(profile, cookies);

        // Exact domain match required
        let found = session.find_cookie("session_id", ".example.com");
        assert!(found.is_some());
        assert_eq!(found.unwrap().value, "secret_value_12345");
        
        // Non-matching domain should return None
        let not_found = session.find_cookie("session_id", "example.com");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_session_remove_expired_cookies() {
        let profile = create_test_profile();
        let mut expired_cookie = create_test_cookie();
        expired_cookie.expires = Some(Utc::now() - Duration::hours(1));
        
        let valid_cookie = create_test_cookie();
        let cookies = vec![expired_cookie, valid_cookie];
        let mut session = Session::new(profile, cookies);

        assert_eq!(session.cookies().len(), 2);
        session.remove_expired_cookies();
        assert_eq!(session.cookies().len(), 1);
    }
}
