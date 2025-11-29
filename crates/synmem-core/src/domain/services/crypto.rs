//! Cryptographic services for secure session management
//!
//! This module provides:
//! - AES-256-GCM encryption/decryption for cookies
//! - Argon2 key derivation from master password
//! - Secure memory handling with zeroize

use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use argon2::{Argon2, password_hash::SaltString, PasswordHasher};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Errors that can occur during cryptographic operations
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Failed to generate random bytes: {0}")]
    RandomGenerationError(String),
    
    #[error("Failed to derive key from password: {0}")]
    KeyDerivationError(String),
    
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    
    #[error("Invalid key length")]
    InvalidKeyLength,
    
    #[error("Invalid nonce")]
    InvalidNonce,
    
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(String),
}

/// Master key derived from password using Argon2
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey {
    /// The derived 256-bit key
    key: [u8; 32],
}

impl MasterKey {
    /// Derive a master key from a password and salt using Argon2id
    pub fn derive(password: &str, salt: &[u8]) -> Result<Self, CryptoError> {
        // Use Argon2id for resistance against both side-channel and GPU attacks
        let argon2 = Argon2::default();
        
        // Convert salt to the format expected by argon2
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| CryptoError::KeyDerivationError(e.to_string()))?;
        
        // Hash the password (this gives us a PHC string)
        let hash = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| CryptoError::KeyDerivationError(e.to_string()))?;
        
        // Extract the hash output
        let hash_output = hash.hash
            .ok_or_else(|| CryptoError::KeyDerivationError("No hash output".to_string()))?;
        
        let hash_bytes = hash_output.as_bytes();
        
        if hash_bytes.len() < 32 {
            return Err(CryptoError::InvalidKeyLength);
        }
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash_bytes[..32]);
        
        Ok(Self { key })
    }

    /// Get a reference to the raw key bytes (for internal use only)
    pub(crate) fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

impl std::fmt::Debug for MasterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never reveal key material in debug output
        f.debug_struct("MasterKey")
            .field("key", &"[REDACTED]")
            .finish()
    }
}

/// Cryptographic service for encrypting and decrypting session data
pub struct CryptoService {
    rng: SystemRandom,
}

impl CryptoService {
    /// Create a new CryptoService instance
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// Generate a cryptographically secure random salt (16 bytes)
    pub fn generate_salt(&self) -> Result<Vec<u8>, CryptoError> {
        let mut salt = vec![0u8; 16];
        self.rng.fill(&mut salt)
            .map_err(|e| CryptoError::RandomGenerationError(e.to_string()))?;
        Ok(salt)
    }

    /// Generate a cryptographically secure random nonce (12 bytes for AES-256-GCM)
    pub fn generate_nonce(&self) -> Result<Vec<u8>, CryptoError> {
        let mut nonce = vec![0u8; 12];
        self.rng.fill(&mut nonce)
            .map_err(|e| CryptoError::RandomGenerationError(e.to_string()))?;
        Ok(nonce)
    }

    /// Encrypt data using AES-256-GCM
    ///
    /// Returns (ciphertext, nonce) where nonce is needed for decryption
    pub fn encrypt(&self, plaintext: &[u8], key: &MasterKey) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        let nonce_bytes = self.generate_nonce()?;
        
        let unbound_key = UnboundKey::new(&AES_256_GCM, key.as_bytes())
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
        
        let sealing_key = LessSafeKey::new(unbound_key);
        
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| CryptoError::InvalidNonce)?;
        
        let mut in_out = plaintext.to_vec();
        
        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
        
        Ok((in_out, nonce_bytes))
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8], key: &MasterKey) -> Result<Vec<u8>, CryptoError> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, key.as_bytes())
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;
        
        let opening_key = LessSafeKey::new(unbound_key);
        
        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|_| CryptoError::InvalidNonce)?;
        
        let mut in_out = ciphertext.to_vec();
        
        let plaintext = opening_key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;
        
        Ok(plaintext.to_vec())
    }

    /// Encrypt and encode as base64 (convenience method)
    pub fn encrypt_to_base64(&self, plaintext: &[u8], key: &MasterKey) -> Result<(String, String), CryptoError> {
        let (ciphertext, nonce) = self.encrypt(plaintext, key)?;
        Ok((BASE64.encode(&ciphertext), BASE64.encode(&nonce)))
    }

    /// Decrypt from base64-encoded data (convenience method)
    pub fn decrypt_from_base64(&self, ciphertext_b64: &str, nonce_b64: &str, key: &MasterKey) -> Result<Vec<u8>, CryptoError> {
        let ciphertext = BASE64.decode(ciphertext_b64)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))?;
        let nonce = BASE64.decode(nonce_b64)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))?;
        
        self.decrypt(&ciphertext, &nonce, key)
    }

    /// Generate a salt and encode as base64
    pub fn generate_salt_base64(&self) -> Result<String, CryptoError> {
        let salt = self.generate_salt()?;
        Ok(BASE64.encode(&salt))
    }

    /// Decode a base64-encoded salt
    pub fn decode_salt(&self, salt_b64: &str) -> Result<Vec<u8>, CryptoError> {
        BASE64.decode(salt_b64)
            .map_err(|e| CryptoError::Base64DecodeError(e.to_string()))
    }
}

impl Default for CryptoService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_key_derivation() {
        let crypto = CryptoService::new();
        let salt = crypto.generate_salt().unwrap();
        let key = MasterKey::derive("test_password_123", &salt).unwrap();
        
        // Key should be 32 bytes
        assert_eq!(key.as_bytes().len(), 32);
    }

    #[test]
    fn test_master_key_consistency() {
        let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key1 = MasterKey::derive("same_password", &salt).unwrap();
        let key2 = MasterKey::derive("same_password", &salt).unwrap();
        
        // Same password + same salt should produce same key
        assert_eq!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key1 = MasterKey::derive("password1", &salt).unwrap();
        let key2 = MasterKey::derive("password2", &salt).unwrap();
        
        assert_ne!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_different_salts_different_keys() {
        let salt1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let salt2 = vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let key1 = MasterKey::derive("same_password", &salt1).unwrap();
        let key2 = MasterKey::derive("same_password", &salt2).unwrap();
        
        assert_ne!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let crypto = CryptoService::new();
        let salt = crypto.generate_salt().unwrap();
        let key = MasterKey::derive("my_secure_password", &salt).unwrap();
        
        let plaintext = b"Hello, this is sensitive cookie data!";
        let (ciphertext, nonce) = crypto.encrypt(plaintext, &key).unwrap();
        
        // Ciphertext should be different from plaintext
        assert_ne!(&ciphertext[..plaintext.len()], plaintext);
        
        // Decryption should recover original plaintext
        let decrypted = crypto.decrypt(&ciphertext, &nonce, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_base64_roundtrip() {
        let crypto = CryptoService::new();
        let salt = crypto.generate_salt().unwrap();
        let key = MasterKey::derive("my_secure_password", &salt).unwrap();
        
        let plaintext = b"Sensitive session data with JSON: {\"user\": \"test\"}";
        let (ciphertext_b64, nonce_b64) = crypto.encrypt_to_base64(plaintext, &key).unwrap();
        
        let decrypted = crypto.decrypt_from_base64(&ciphertext_b64, &nonce_b64, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_password_fails_decryption() {
        let crypto = CryptoService::new();
        let salt = crypto.generate_salt().unwrap();
        let key1 = MasterKey::derive("correct_password", &salt).unwrap();
        let key2 = MasterKey::derive("wrong_password", &salt).unwrap();
        
        let plaintext = b"Secret data";
        let (ciphertext, nonce) = crypto.encrypt(plaintext, &key1).unwrap();
        
        // Decryption with wrong key should fail
        let result = crypto.decrypt(&ciphertext, &nonce, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_master_key_debug_redacted() {
        let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let key = MasterKey::derive("password", &salt).unwrap();
        
        let debug_output = format!("{:?}", key);
        assert!(debug_output.contains("REDACTED"));
        assert!(!debug_output.contains(&format!("{:?}", key.as_bytes())));
    }

    #[test]
    fn test_nonce_uniqueness() {
        let crypto = CryptoService::new();
        let nonce1 = crypto.generate_nonce().unwrap();
        let nonce2 = crypto.generate_nonce().unwrap();
        
        // Nonces should be unique
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_salt_generation() {
        let crypto = CryptoService::new();
        let salt1 = crypto.generate_salt().unwrap();
        let salt2 = crypto.generate_salt().unwrap();
        
        // Salts should be 16 bytes
        assert_eq!(salt1.len(), 16);
        // Salts should be unique
        assert_ne!(salt1, salt2);
    }
}
