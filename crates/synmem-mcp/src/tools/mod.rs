//! MCP Navigation Tools
//!
//! Tools for browser navigation and interaction.

mod navigate;
mod click;
mod type_text;
mod scroll;
mod screenshot;
mod wait_for;

pub use navigate::*;
pub use click::*;
pub use type_text::*;
pub use scroll::*;
pub use screenshot::*;
pub use wait_for::*;

use rmcp::model::Tool;
use serde_json::json;
use std::sync::Arc;

/// Get all navigation tools.
pub fn all_navigation_tools() -> Vec<Tool> {
    vec![
        navigate_to_tool(),
        click_tool(),
        type_text_tool(),
        scroll_tool(),
        screenshot_tool(),
        wait_for_tool(),
    ]
}

/// Helper to convert JSON value to Arc<JsonObject>
fn json_to_schema(value: serde_json::Value) -> Arc<serde_json::Map<String, serde_json::Value>> {
    Arc::new(value.as_object().cloned().unwrap_or_default())
}

/// Create the navigate_to tool definition.
pub fn navigate_to_tool() -> Tool {
    Tool::new(
        "navigate_to",
        "Navigate browser to URL",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to navigate to"
                },
                "wait_for": {
                    "type": "string",
                    "enum": ["load", "domcontentloaded", "networkidle"],
                    "description": "Wait condition before returning (default: load)"
                }
            },
            "required": ["url"]
        })),
    )
}

/// Create the click tool definition.
pub fn click_tool() -> Tool {
    Tool::new(
        "click",
        "Click element by selector or text",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector of the element to click"
                },
                "text": {
                    "type": "string",
                    "description": "Text content of the element to click"
                }
            }
        })),
    )
}

/// Create the type_text tool definition.
pub fn type_text_tool() -> Tool {
    Tool::new(
        "type_text",
        "Type text into an input element",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector of the input element"
                },
                "text": {
                    "type": "string",
                    "description": "Text to type"
                }
            },
            "required": ["selector", "text"]
        })),
    )
}

/// Create the scroll tool definition.
pub fn scroll_tool() -> Tool {
    Tool::new(
        "scroll",
        "Scroll the page",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "direction": {
                    "type": "string",
                    "enum": ["up", "down", "left", "right"],
                    "description": "Direction to scroll (default: down)"
                },
                "amount": {
                    "type": "integer",
                    "description": "Amount to scroll in pixels"
                }
            }
        })),
    )
}

/// Create the screenshot tool definition.
pub fn screenshot_tool() -> Tool {
    Tool::new(
        "screenshot",
        "Take a screenshot of the current page",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "full_page": {
                    "type": "boolean",
                    "description": "Capture the full scrollable page (default: false)"
                },
                "path": {
                    "type": "string",
                    "description": "File path to save the screenshot"
                }
            }
        })),
    )
}

/// Create the wait_for tool definition.
pub fn wait_for_tool() -> Tool {
    Tool::new(
        "wait_for",
        "Wait for an element to appear",
        json_to_schema(json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector of the element to wait for"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in milliseconds (default: 30000)"
                }
            },
            "required": ["selector"]
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_navigation_tools() {
        let tools = all_navigation_tools();
        assert_eq!(tools.len(), 6);

        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(tool_names.contains(&"navigate_to"));
        assert!(tool_names.contains(&"click"));
        assert!(tool_names.contains(&"type_text"));
        assert!(tool_names.contains(&"scroll"));
        assert!(tool_names.contains(&"screenshot"));
        assert!(tool_names.contains(&"wait_for"));
    }

    #[test]
    fn test_navigate_to_tool_schema() {
        let tool = navigate_to_tool();
        assert_eq!(tool.name.as_ref(), "navigate_to");
        assert!(tool.description.contains("Navigate"));

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
    }

    #[test]
    fn test_click_tool_schema() {
        let tool = click_tool();
        assert_eq!(tool.name.as_ref(), "click");

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
    }

    #[test]
    fn test_type_text_tool_schema() {
        let tool = type_text_tool();
        assert_eq!(tool.name.as_ref(), "type_text");

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
        assert!(schema.contains_key("required"));
    }

    #[test]
    fn test_scroll_tool_schema() {
        let tool = scroll_tool();
        assert_eq!(tool.name.as_ref(), "scroll");

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
    }

    #[test]
    fn test_screenshot_tool_schema() {
        let tool = screenshot_tool();
        assert_eq!(tool.name.as_ref(), "screenshot");

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
    }

    #[test]
    fn test_wait_for_tool_schema() {
        let tool = wait_for_tool();
        assert_eq!(tool.name.as_ref(), "wait_for");

        let schema = &tool.input_schema;
        assert!(schema.contains_key("properties"));
        assert!(schema.contains_key("required"));
    }
}
