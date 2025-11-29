//! SynMem Native Messaging Host
//!
//! This binary implements the Chrome/Firefox Native Messaging protocol
//! for communication between the SynMem browser extension and the core
//! Rust functionality.
//!
//! # Protocol
//! The native messaging protocol uses stdin/stdout with length-prefixed messages:
//! - Each message is preceded by 4 bytes containing the message length (little-endian)
//! - Messages are JSON-encoded
//!
//! # Usage
//! This binary is typically invoked by the browser when the extension
//! connects to the native host. It should not be run manually.

mod error;
mod handler;
mod messages;
mod protocol;

use std::io::{self, BufReader, BufWriter};

use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::error::NativeHostError;
use crate::handler::process_message;
use crate::protocol::{read_message, write_message};

fn main() {
    // Initialize logging to stderr (stdout is reserved for native messaging)
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_writer(io::stderr)
        .with_ansi(false)
        .init();

    info!(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        "Native host starting"
    );

    if let Err(e) = run() {
        error!(error = %e, "Native host error");
        std::process::exit(1);
    }

    info!("Native host shutting down");
}

fn run() -> Result<(), NativeHostError> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    // Main message loop
    loop {
        // Read message from extension
        let message = match read_message(&mut reader) {
            Ok(msg) => msg,
            Err(NativeHostError::ReadError(e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                // Extension closed the connection
                debug!("Extension disconnected (EOF)");
                break;
            }
            Err(e) => {
                error!(error = %e, "Failed to read message");
                return Err(e);
            }
        };

        debug!(size = message.len(), "Received message");

        // Process the message
        let response = match process_message(&message) {
            Ok(resp) => resp,
            Err(e) => {
                // Return error as JSON response
                let error_response = messages::Response::error(None, e.to_string());
                serde_json::to_vec(&error_response)?
            }
        };

        // Write response back to extension
        write_message(&mut writer, &response)?;
        debug!(size = response.len(), "Sent response");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{read_message, write_message};
    use std::io::Cursor;

    #[test]
    fn test_full_message_cycle() {
        // Simulate a ping request
        let request = serde_json::json!({
            "action": "ping",
            "id": "integration-test"
        });
        let request_bytes = serde_json::to_vec(&request).unwrap();

        // Write to buffer (simulating extension sending)
        let mut send_buffer = Vec::new();
        write_message(&mut send_buffer, &request_bytes).unwrap();

        // Read from buffer (simulating native host receiving)
        let mut cursor = Cursor::new(send_buffer);
        let received = read_message(&mut cursor).unwrap();

        // Process
        let response_bytes = process_message(&received).unwrap();
        let response: messages::Response = serde_json::from_slice(&response_bytes).unwrap();

        assert!(response.success);
        assert_eq!(response.id, Some("integration-test".to_string()));
        assert_eq!(response.data, Some(serde_json::json!("pong")));
    }
}
