//! Audit logging for SynMem Cloud (SOC2 prep)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types of auditable actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    // Authentication
    UserLogin,
    UserLogout,
    PasswordReset,
    MfaEnabled,
    MfaDisabled,

    // API Keys
    ApiKeyCreated,
    ApiKeyRevoked,
    ApiKeyUsed,

    // Scraping
    ScrapeRequested,
    ScrapeCompleted,
    ScrapeFailed,

    // Sessions
    SessionCreated,
    SessionSynced,
    SessionDeleted,

    // Browser Pool
    BrowserSpawned,
    BrowserTerminated,

    // Billing
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionCanceled,
    PaymentSucceeded,
    PaymentFailed,

    // Admin
    UserCreated,
    UserUpdated,
    UserDeleted,
    SettingsChanged,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self)
            .unwrap_or_else(|_| "\"unknown\"".to_string())
            .trim_matches('"')
            .to_string();
        write!(f, "{s}")
    }
}

/// An audit log entry for compliance and security tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// Unique log entry identifier
    pub id: Uuid,
    /// User who performed the action (if applicable)
    pub user_id: Option<Uuid>,
    /// API key used (if applicable)
    pub api_key_id: Option<Uuid>,
    /// The action that was performed
    pub action: AuditAction,
    /// Resource type affected (e.g., "session", "scrape_job")
    pub resource_type: Option<String>,
    /// Resource ID affected
    pub resource_id: Option<Uuid>,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// Additional metadata as JSON
    pub metadata: Option<serde_json::Value>,
    /// Whether the action succeeded
    pub success: bool,
    /// Error message if action failed
    pub error_message: Option<String>,
    /// Timestamp of the action
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    /// Creates a new successful audit log entry
    #[must_use]
    pub fn success(action: AuditAction) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            api_key_id: None,
            action,
            resource_type: None,
            resource_id: None,
            ip_address: None,
            user_agent: None,
            metadata: None,
            success: true,
            error_message: None,
            created_at: Utc::now(),
        }
    }

    /// Creates a new failed audit log entry
    #[must_use]
    pub fn failure(action: AuditAction, error: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            api_key_id: None,
            action,
            resource_type: None,
            resource_id: None,
            ip_address: None,
            user_agent: None,
            metadata: None,
            success: false,
            error_message: Some(error.into()),
            created_at: Utc::now(),
        }
    }

    /// Builder method to set the user ID
    #[must_use]
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Builder method to set the API key ID
    #[must_use]
    pub fn with_api_key(mut self, api_key_id: Uuid) -> Self {
        self.api_key_id = Some(api_key_id);
        self
    }

    /// Builder method to set the resource
    #[must_use]
    pub fn with_resource(mut self, resource_type: impl Into<String>, resource_id: Uuid) -> Self {
        self.resource_type = Some(resource_type.into());
        self.resource_id = Some(resource_id);
        self
    }

    /// Builder method to set the IP address
    #[must_use]
    pub fn with_ip(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Builder method to set the user agent
    #[must_use]
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Builder method to set additional metadata
    #[must_use]
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_success() {
        let log = AuditLog::success(AuditAction::UserLogin);
        assert!(log.success);
        assert!(log.error_message.is_none());
        assert_eq!(log.action, AuditAction::UserLogin);
    }

    #[test]
    fn test_audit_log_failure() {
        let log = AuditLog::failure(AuditAction::ScrapeRequested, "Rate limit exceeded");
        assert!(!log.success);
        assert_eq!(log.error_message, Some("Rate limit exceeded".to_string()));
    }

    #[test]
    fn test_audit_log_builder() {
        let user_id = Uuid::new_v4();
        let resource_id = Uuid::new_v4();

        let log = AuditLog::success(AuditAction::SessionCreated)
            .with_user(user_id)
            .with_resource("session", resource_id)
            .with_ip("192.168.1.1")
            .with_user_agent("Mozilla/5.0");

        assert_eq!(log.user_id, Some(user_id));
        assert_eq!(log.resource_type, Some("session".to_string()));
        assert_eq!(log.resource_id, Some(resource_id));
        assert_eq!(log.ip_address, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_action_display() {
        assert_eq!(AuditAction::UserLogin.to_string(), "user_login");
        assert_eq!(AuditAction::ScrapeCompleted.to_string(), "scrape_completed");
    }
}
