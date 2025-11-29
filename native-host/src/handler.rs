//! Message handler for processing extension requests

use serde_json::json;
use tracing::{debug, warn};

use crate::error::Result;
use crate::messages::{Request, Response};

/// Handle a request from the browser extension
///
/// This function routes requests to the appropriate handler based on the action.
/// Currently supported actions:
/// - `ping`: Health check, returns "pong"
/// - `version`: Returns the native host version
/// - `echo`: Echoes back the payload
///
/// Future actions (to be implemented when synmem-core is available):
/// - `scrape`: Scrape page content
/// - `search`: Search memory
/// - `navigate`: Browser navigation
pub fn handle_request(request: Request) -> Response {
    debug!(action = %request.action, id = ?request.id, "Handling request");

    match request.action.as_str() {
        "ping" => Response::success(request.id, json!("pong")),

        "version" => Response::success(
            request.id,
            json!({
                "name": env!("CARGO_PKG_NAME"),
                "version": env!("CARGO_PKG_VERSION"),
            }),
        ),

        "echo" => Response::success(request.id, request.payload),

        // Placeholder for future synmem-core integration
        "scrape" | "search" | "navigate" | "store" => {
            warn!(action = %request.action, "Action not yet implemented");
            Response::error(
                request.id,
                format!("Action '{}' is not yet implemented", request.action),
            )
        }

        unknown => {
            warn!(action = %unknown, "Unknown action");
            Response::error(request.id, format!("Unknown action: {}", unknown))
        }
    }
}

/// Process a raw message bytes and return response bytes
pub fn process_message(message_bytes: &[u8]) -> Result<Vec<u8>> {
    // Parse the request
    let request: Request = serde_json::from_slice(message_bytes)?;

    // Handle the request
    let response = handle_request(request);

    // Serialize the response
    let response_bytes = serde_json::to_vec(&response)?;

    Ok(response_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_ping() {
        let request = Request {
            id: Some("123".to_string()),
            action: "ping".to_string(),
            payload: serde_json::Value::Null,
        };

        let response = handle_request(request);

        assert!(response.success);
        assert_eq!(response.id, Some("123".to_string()));
        assert_eq!(response.data, Some(json!("pong")));
    }

    #[test]
    fn test_handle_version() {
        let request = Request {
            id: None,
            action: "version".to_string(),
            payload: serde_json::Value::Null,
        };

        let response = handle_request(request);

        assert!(response.success);
        let data = response.data.unwrap();
        assert_eq!(data["name"], "synmem-native-host");
        assert!(data["version"].is_string());
    }

    #[test]
    fn test_handle_echo() {
        let payload = json!({"test": "data", "number": 42});
        let request = Request {
            id: Some("456".to_string()),
            action: "echo".to_string(),
            payload: payload.clone(),
        };

        let response = handle_request(request);

        assert!(response.success);
        assert_eq!(response.data, Some(payload));
    }

    #[test]
    fn test_handle_unknown_action() {
        let request = Request {
            id: Some("789".to_string()),
            action: "unknown_action".to_string(),
            payload: serde_json::Value::Null,
        };

        let response = handle_request(request);

        assert!(!response.success);
        assert!(response.error.unwrap().contains("Unknown action"));
    }

    #[test]
    fn test_handle_not_implemented() {
        let request = Request {
            id: None,
            action: "scrape".to_string(),
            payload: serde_json::Value::Null,
        };

        let response = handle_request(request);

        assert!(!response.success);
        assert!(response.error.unwrap().contains("not yet implemented"));
    }

    #[test]
    fn test_process_message() {
        let message = br#"{"action": "ping", "id": "test-1"}"#;
        let response_bytes = process_message(message).unwrap();
        let response: Response = serde_json::from_slice(&response_bytes).unwrap();

        assert!(response.success);
        assert_eq!(response.id, Some("test-1".to_string()));
    }

    #[test]
    fn test_process_invalid_json() {
        let invalid_message = b"not valid json";
        let result = process_message(invalid_message);

        assert!(result.is_err());
    }
}
