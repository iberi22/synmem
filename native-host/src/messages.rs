//! Message types for extension communication

use serde::{Deserialize, Serialize};

/// Request message from the browser extension
#[derive(Debug, Deserialize)]
pub struct Request {
    /// Unique request ID for correlation
    #[serde(default)]
    pub id: Option<String>,

    /// The action to perform
    pub action: String,

    /// Optional payload data
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// Response message to the browser extension
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// Request ID for correlation (echoed from request)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Whether the operation was successful
    pub success: bool,

    /// Response data (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Response {
    /// Create a success response
    pub fn success(id: Option<String>, data: serde_json::Value) -> Self {
        Self {
            id,
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: Option<String>, error: impl Into<String>) -> Self {
        Self {
            id,
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_request() {
        let json = r#"{"id": "123", "action": "ping", "payload": {"test": true}}"#;
        let request: Request = serde_json::from_str(json).unwrap();

        assert_eq!(request.id, Some("123".to_string()));
        assert_eq!(request.action, "ping");
        assert_eq!(request.payload["test"], true);
    }

    #[test]
    fn test_deserialize_request_minimal() {
        let json = r#"{"action": "ping"}"#;
        let request: Request = serde_json::from_str(json).unwrap();

        assert_eq!(request.id, None);
        assert_eq!(request.action, "ping");
        assert!(request.payload.is_null());
    }

    #[test]
    fn test_serialize_success_response() {
        let response =
            Response::success(Some("123".to_string()), serde_json::json!({"result": "ok"}));
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains(r#""success":true"#));
        assert!(json.contains(r#""id":"123""#));
        assert!(json.contains(r#""result":"ok""#));
        assert!(!json.contains("error"));
    }

    #[test]
    fn test_serialize_error_response() {
        let response = Response::error(Some("123".to_string()), "Something went wrong");
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains(r#""success":false"#));
        assert!(json.contains(r#""error":"Something went wrong""#));
        assert!(!json.contains("data"));
    }
}
