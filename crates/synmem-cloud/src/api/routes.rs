//! API routes for SynMem Cloud

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::error::ApiError;
use crate::domain::Tier;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    // In a real implementation, this would contain:
    // - Database connection pool
    // - Service instances
    // - Configuration
    pub version: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Creates the main API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/v1/scrape", post(create_scrape))
        .route("/v1/sessions", get(list_sessions))
        .route("/v1/sessions", post(create_session))
        .route("/v1/browser/acquire", post(acquire_browser))
        .route("/v1/browser/release", post(release_browser))
        .route("/v1/usage", get(get_usage))
        .with_state(Arc::new(state))
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Health check endpoint
async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: state.version.clone(),
    })
}

/// Scrape request body
#[derive(Debug, Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
    pub selector: Option<String>,
    pub wait_for: Option<String>,
}

/// Scrape response body
#[derive(Debug, Serialize)]
pub struct ScrapeResponse {
    pub job_id: String,
    pub status: String,
}

/// Create a scrape job
async fn create_scrape(
    Json(request): Json<ScrapeRequest>,
) -> Result<Json<ScrapeResponse>, ApiError> {
    // In a real implementation, this would:
    // 1. Validate the request
    // 2. Check user's tier and rate limits
    // 3. Queue the scrape job
    // 4. Return the job ID

    if request.url.is_empty() {
        return Err(ApiError::bad_request("URL is required"));
    }

    Ok(Json(ScrapeResponse {
        job_id: uuid::Uuid::new_v4().to_string(),
        status: "queued".to_string(),
    }))
}

/// Session list response
#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionInfo>,
}

/// Session info
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub device_id: String,
    pub synced_at: String,
}

/// List user's sessions
async fn list_sessions() -> Json<SessionListResponse> {
    // In a real implementation, this would fetch from database
    Json(SessionListResponse { sessions: vec![] })
}

/// Create session request
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
    pub device_id: String,
}

/// Create a new session
async fn create_session(
    Json(request): Json<CreateSessionRequest>,
) -> Result<Json<SessionInfo>, ApiError> {
    if request.name.is_empty() {
        return Err(ApiError::bad_request("Session name is required"));
    }

    Ok(Json(SessionInfo {
        id: uuid::Uuid::new_v4().to_string(),
        name: request.name,
        device_id: request.device_id,
        synced_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// Browser acquire request
#[derive(Debug, Deserialize)]
pub struct AcquireBrowserRequest {
    pub tier: Option<Tier>,
}

/// Browser session response
#[derive(Debug, Serialize)]
pub struct BrowserSessionResponse {
    pub session_id: String,
    pub cdp_endpoint: String,
    pub expires_at: String,
}

/// Acquire a browser session
async fn acquire_browser(
    Json(_request): Json<AcquireBrowserRequest>,
) -> Result<Json<BrowserSessionResponse>, ApiError> {
    // In a real implementation, this would:
    // 1. Check user's tier for cloud session access
    // 2. Check pool capacity
    // 3. Spawn or acquire a browser instance
    // 4. Return the CDP endpoint

    Ok(Json(BrowserSessionResponse {
        session_id: uuid::Uuid::new_v4().to_string(),
        cdp_endpoint: "ws://localhost:9222/devtools/browser/...".to_string(),
        expires_at: (chrono::Utc::now() + chrono::Duration::minutes(30)).to_rfc3339(),
    }))
}

/// Release browser request
#[derive(Debug, Deserialize)]
pub struct ReleaseBrowserRequest {
    pub session_id: String,
}

/// Release a browser session
async fn release_browser(
    Json(request): Json<ReleaseBrowserRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if request.session_id.is_empty() {
        return Err(ApiError::bad_request("Session ID is required"));
    }

    Ok(Json(serde_json::json!({ "status": "released" })))
}

/// Usage response
#[derive(Debug, Serialize)]
pub struct UsageResponse {
    pub tier: Tier,
    pub scrapes_this_month: u32,
    pub scrape_limit: Option<u32>,
    pub storage_bytes_used: u64,
    pub browser_sessions_active: u32,
}

/// Get usage statistics
async fn get_usage() -> Json<UsageResponse> {
    // In a real implementation, this would fetch from database
    Json(UsageResponse {
        tier: Tier::Free,
        scrapes_this_month: 0,
        scrape_limit: Some(100),
        storage_bytes_used: 0,
        browser_sessions_active: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(!state.version.is_empty());
    }

    #[tokio::test]
    async fn test_health_check() {
        let state = Arc::new(AppState::default());
        let response = health_check(State(state)).await;
        assert_eq!(response.status, "ok");
    }
}
