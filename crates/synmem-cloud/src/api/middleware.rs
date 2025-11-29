//! API middleware for authentication, rate limiting, and logging

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::domain::RateLimitHeaders;

/// Extension type for authenticated user information
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub tier: crate::domain::Tier,
    pub api_key_id: Option<uuid::Uuid>,
}

/// Extension type for rate limit information
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub headers: RateLimitHeaders,
}

/// Middleware to add rate limit headers to responses
pub async fn rate_limit_headers(
    request: Request<Body>,
    next: Next,
) -> Response {
    let rate_info = request.extensions().get::<RateLimitInfo>().cloned();

    let mut response = next.run(request).await;

    if let Some(info) = rate_info {
        let headers = response.headers_mut();
        headers.insert(
            "X-RateLimit-Limit",
            info.headers.limit.to_string().parse().unwrap(),
        );
        headers.insert(
            "X-RateLimit-Remaining",
            info.headers.remaining.to_string().parse().unwrap(),
        );
        headers.insert(
            "X-RateLimit-Reset",
            info.headers.reset.to_string().parse().unwrap(),
        );
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Tier;

    #[test]
    fn test_authenticated_user() {
        let user = AuthenticatedUser {
            user_id: uuid::Uuid::new_v4(),
            tier: Tier::Pro,
            api_key_id: Some(uuid::Uuid::new_v4()),
        };

        assert!(user.api_key_id.is_some());
    }

    #[test]
    fn test_rate_limit_info() {
        let info = RateLimitInfo {
            headers: RateLimitHeaders {
                limit: 100,
                remaining: 50,
                reset: 1234567890,
            },
        };

        assert_eq!(info.headers.limit, 100);
    }
}
