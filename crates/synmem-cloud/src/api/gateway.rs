//! API Gateway for SynMem Cloud
//!
//! The API Gateway is the main entry point for all API requests.
//! It handles:
//! - Request routing
//! - Authentication
//! - Rate limiting
//! - CORS
//! - Request logging

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use super::routes::{create_router, AppState};

/// Configuration for the API Gateway
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Enable CORS for all origins (development only)
    pub cors_permissive: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_permissive: false,
        }
    }
}

/// The API Gateway server
pub struct ApiGateway {
    config: GatewayConfig,
    app_state: AppState,
}

impl ApiGateway {
    /// Creates a new API Gateway with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: GatewayConfig::default(),
            app_state: AppState::default(),
        }
    }

    /// Creates a new API Gateway with custom configuration
    #[must_use]
    pub fn with_config(config: GatewayConfig) -> Self {
        Self {
            config,
            app_state: AppState::default(),
        }
    }

    /// Sets the application state
    #[must_use]
    pub fn with_state(mut self, state: AppState) -> Self {
        self.app_state = state;
        self
    }

    /// Builds the router with all middleware
    pub fn build_router(&self) -> Router {
        let cors = if self.config.cors_permissive {
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        } else {
            CorsLayer::new()
        };

        create_router(self.app_state.clone())
            .layer(TraceLayer::new_for_http())
            .layer(cors)
    }

    /// Starts the API Gateway server
    ///
    /// # Errors
    /// Returns an error if the server fails to bind or start
    pub async fn serve(self) -> Result<(), std::io::Error> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .expect("Invalid address");

        let router = self.build_router();

        info!("Starting API Gateway on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await
    }
}

impl Default for ApiGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_config_default() {
        let config = GatewayConfig::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert!(!config.cors_permissive);
    }

    #[test]
    fn test_gateway_creation() {
        let gateway = ApiGateway::new();
        assert_eq!(gateway.config.port, 8080);
    }

    #[test]
    fn test_gateway_with_config() {
        let config = GatewayConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            cors_permissive: true,
        };
        let gateway = ApiGateway::with_config(config);
        assert_eq!(gateway.config.port, 3000);
    }

    #[test]
    fn test_build_router() {
        let gateway = ApiGateway::new();
        let _router = gateway.build_router();
        // Router builds successfully
    }
}
