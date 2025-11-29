//! Native Messaging protocol implementation
//!
//! The Native Messaging protocol uses stdin/stdout with length-prefixed messages:
//! - Each message is preceded by 4 bytes containing the message length (little-endian)
//! - Messages are JSON-encoded

use std::io::{Read, Write};

use crate::error::{NativeHostError, Result};

/// Maximum message size (1 MB as per Chrome's native messaging limit)
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// Read a message from the given reader using the native messaging protocol
///
/// # Protocol
/// 1. Read 4 bytes as little-endian u32 for message length
/// 2. Read `length` bytes as the message content
///
/// # Errors
/// - Returns error if reading fails
/// - Returns error if message exceeds MAX_MESSAGE_SIZE
pub fn read_message<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    // Read message length (4 bytes, little-endian)
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes) as usize;

    // Validate message size
    if len > MAX_MESSAGE_SIZE {
        return Err(NativeHostError::MessageTooLarge {
            size: len,
            max: MAX_MESSAGE_SIZE,
        });
    }

    // Read message
    let mut message = vec![0u8; len];
    reader.read_exact(&mut message)?;

    Ok(message)
}

/// Write a message to the given writer using the native messaging protocol
///
/// # Protocol
/// 1. Write 4 bytes as little-endian u32 for message length
/// 2. Write the message content
///
/// # Errors
/// - Returns error if writing fails
/// - Returns error if message exceeds MAX_MESSAGE_SIZE
pub fn write_message<W: Write>(writer: &mut W, message: &[u8]) -> Result<()> {
    // Validate message size
    if message.len() > MAX_MESSAGE_SIZE {
        return Err(NativeHostError::MessageTooLarge {
            size: message.len(),
            max: MAX_MESSAGE_SIZE,
        });
    }

    // Write message length (4 bytes, little-endian)
    let len_bytes = (message.len() as u32).to_le_bytes();
    writer.write_all(&len_bytes)?;

    // Write message
    writer.write_all(message)?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_message() {
        let message = b"hello";
        let len_bytes = (message.len() as u32).to_le_bytes();
        let mut data = Vec::new();
        data.extend_from_slice(&len_bytes);
        data.extend_from_slice(message);

        let mut cursor = Cursor::new(data);
        let result = read_message(&mut cursor).unwrap();

        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_write_message() {
        let message = b"hello";
        let mut buffer = Vec::new();

        write_message(&mut buffer, message).unwrap();

        // Check length prefix
        let len = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        assert_eq!(len, 5);

        // Check message content
        assert_eq!(&buffer[4..], b"hello");
    }

    #[test]
    fn test_roundtrip() {
        let original = b"test message";
        let mut buffer = Vec::new();

        // Write
        write_message(&mut buffer, original).unwrap();

        // Read
        let mut cursor = Cursor::new(buffer);
        let result = read_message(&mut cursor).unwrap();

        assert_eq!(result, original);
    }

    #[test]
    fn test_read_empty_message() {
        let len_bytes = 0u32.to_le_bytes();
        let mut cursor = Cursor::new(len_bytes.to_vec());

        let result = read_message(&mut cursor).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_message_too_large() {
        let large_size = (MAX_MESSAGE_SIZE + 1) as u32;
        let len_bytes = large_size.to_le_bytes();
        let mut cursor = Cursor::new(len_bytes.to_vec());

        let result = read_message(&mut cursor);
        assert!(matches!(
            result,
            Err(NativeHostError::MessageTooLarge { .. })
        ));
    }

    #[test]
    fn test_write_message_too_large() {
        let large_message = vec![0u8; MAX_MESSAGE_SIZE + 1];
        let mut buffer = Vec::new();

        let result = write_message(&mut buffer, &large_message);
        assert!(matches!(
            result,
            Err(NativeHostError::MessageTooLarge { .. })
        ));
    }

    #[test]
    fn test_json_message_roundtrip() {
        let json = serde_json::json!({
            "action": "test",
            "payload": {"key": "value"}
        });
        let json_bytes = serde_json::to_vec(&json).unwrap();
        let mut buffer = Vec::new();

        // Write
        write_message(&mut buffer, &json_bytes).unwrap();

        // Read
        let mut cursor = Cursor::new(buffer);
        let result = read_message(&mut cursor).unwrap();

        // Parse
        let parsed: serde_json::Value = serde_json::from_slice(&result).unwrap();
        assert_eq!(parsed["action"], "test");
        assert_eq!(parsed["payload"]["key"], "value");
    }
}
