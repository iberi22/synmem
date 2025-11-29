//! Browser pool service for headless browser management

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::Tier;

/// Browser pool errors
#[derive(Debug, Error)]
pub enum BrowserPoolError {
    #[error("Pool capacity reached for tier {0}")]
    CapacityReached(Tier),

    #[error("Browser session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Browser session expired: {0}")]
    SessionExpired(Uuid),

    #[error("Browser crashed: {0}")]
    BrowserCrashed(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Status of a browser session in the pool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserSessionStatus {
    /// Browser is starting up
    Starting,
    /// Browser is ready for use
    Ready,
    /// Browser is currently in use
    InUse,
    /// Browser is being recycled
    Recycling,
    /// Browser has been terminated
    Terminated,
}

/// A browser session in the cloud pool
#[derive(Debug, Clone)]
pub struct CloudBrowserSession {
    /// Unique session identifier
    pub id: Uuid,
    /// User who owns this session
    pub user_id: Uuid,
    /// Current status
    pub status: BrowserSessionStatus,
    /// Browser type/version
    pub browser_type: String,
    /// CDP (Chrome DevTools Protocol) endpoint URL
    pub cdp_endpoint: Option<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    pub last_activity_at: chrono::DateTime<chrono::Utc>,
}

impl CloudBrowserSession {
    /// Creates a new browser session
    #[must_use]
    pub fn new(user_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            status: BrowserSessionStatus::Starting,
            browser_type: "chromium".to_string(),
            cdp_endpoint: None,
            created_at: now,
            last_activity_at: now,
        }
    }

    /// Checks if the session has timed out (no activity for 30 minutes)
    #[must_use]
    pub fn is_timed_out(&self) -> bool {
        let timeout = chrono::Duration::minutes(30);
        chrono::Utc::now() - self.last_activity_at > timeout
    }
}

/// Trait for browser pool operations
#[async_trait::async_trait]
pub trait BrowserPoolServiceTrait: Send + Sync {
    /// Acquires a browser session from the pool
    async fn acquire(&self, user_id: Uuid, tier: Tier) -> Result<CloudBrowserSession, BrowserPoolError>;

    /// Releases a browser session back to the pool
    async fn release(&self, session_id: Uuid) -> Result<(), BrowserPoolError>;

    /// Gets the status of a browser session
    async fn get_status(&self, session_id: Uuid) -> Result<CloudBrowserSession, BrowserPoolError>;

    /// Lists all active sessions for a user
    async fn list_user_sessions(&self, user_id: Uuid) -> Result<Vec<CloudBrowserSession>, BrowserPoolError>;

    /// Terminates a browser session
    async fn terminate(&self, session_id: Uuid) -> Result<(), BrowserPoolError>;

    /// Gets pool statistics
    async fn get_stats(&self) -> BrowserPoolStats;
}

/// Statistics about the browser pool
#[derive(Debug, Clone, Default)]
pub struct BrowserPoolStats {
    /// Total browsers in the pool
    pub total: u32,
    /// Browsers currently in use
    pub in_use: u32,
    /// Browsers ready for use
    pub available: u32,
    /// Browsers being recycled
    pub recycling: u32,
}

/// Default implementation of the browser pool service
///
/// # Note
/// This is a placeholder implementation. The actual implementation will need:
/// - Container orchestration client (Docker/K8s)
/// - Session state storage
/// - Health monitoring
///
/// TODO: Implement `BrowserPoolServiceTrait` when infrastructure is ready
#[derive(Default)]
pub struct BrowserPoolService {
    _private: (),
}

impl BrowserPoolService {
    /// Creates a new browser pool service
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { _private: () })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_session_creation() {
        let user_id = Uuid::new_v4();
        let session = CloudBrowserSession::new(user_id);

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.status, BrowserSessionStatus::Starting);
        assert!(!session.is_timed_out());
    }

    #[test]
    fn test_pool_error_display() {
        let err = BrowserPoolError::CapacityReached(Tier::Free);
        assert!(err.to_string().contains("Free"));
    }

    #[test]
    fn test_pool_stats_default() {
        let stats = BrowserPoolStats::default();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.available, 0);
    }
}
