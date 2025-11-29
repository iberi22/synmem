//! License key system for offline validation.
//!
//! Uses Ed25519 signatures for cryptographic verification.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use super::tier::Tier;
use crate::error::{Result, SubscriptionError};

/// A license key for offline validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// Unique license key identifier.
    pub key: String,
    /// Subscription tier this license grants.
    pub tier: Tier,
    /// When this license expires.
    pub expires_at: DateTime<Utc>,
    /// Ed25519 signature for verification.
    pub signature: String,
}

/// Data that gets signed to create/verify a license.
#[derive(Debug, Serialize)]
struct LicensePayload {
    key: String,
    tier: Tier,
    expires_at: DateTime<Utc>,
}

impl License {
    /// Creates a new license with the given parameters and signs it.
    ///
    /// # Arguments
    /// * `key` - Unique license key identifier
    /// * `tier` - Subscription tier to grant
    /// * `expires_at` - Expiration date/time
    /// * `signing_key` - Ed25519 signing key
    ///
    /// # Returns
    /// A new signed `License`.
    pub fn new(
        key: String,
        tier: Tier,
        expires_at: DateTime<Utc>,
        signing_key: &SigningKey,
    ) -> Self {
        let payload = LicensePayload {
            key: key.clone(),
            tier,
            expires_at,
        };
        let message = serde_json::to_string(&payload).expect("Failed to serialize payload");
        let signature = signing_key.sign(message.as_bytes());
        let signature_b64 = BASE64.encode(signature.to_bytes());

        Self {
            key,
            tier,
            expires_at,
            signature: signature_b64,
        }
    }

    /// Verifies this license against the provided public key.
    ///
    /// # Arguments
    /// * `verifying_key` - Ed25519 public key to verify against
    ///
    /// # Returns
    /// `Ok(())` if the signature is valid, `Err` otherwise.
    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<()> {
        let payload = LicensePayload {
            key: self.key.clone(),
            tier: self.tier,
            expires_at: self.expires_at,
        };
        let message = serde_json::to_string(&payload).map_err(|e| {
            SubscriptionError::LicenseValidation(format!("Failed to serialize payload: {e}"))
        })?;

        let signature_bytes = BASE64.decode(&self.signature).map_err(|e| {
            SubscriptionError::LicenseValidation(format!("Invalid signature encoding: {e}"))
        })?;

        let signature_array: [u8; 64] =
            signature_bytes.try_into().map_err(|_| {
                SubscriptionError::LicenseValidation("Invalid signature length".to_string())
            })?;

        let signature = Signature::from_bytes(&signature_array);

        verifying_key
            .verify(message.as_bytes(), &signature)
            .map_err(|e| {
                SubscriptionError::LicenseValidation(format!("Signature verification failed: {e}"))
            })
    }

    /// Checks if the license is currently valid (not expired and signature valid).
    ///
    /// # Arguments
    /// * `verifying_key` - Ed25519 public key to verify against
    ///
    /// # Returns
    /// `Ok(())` if the license is valid, `Err` otherwise.
    pub fn validate(&self, verifying_key: &VerifyingKey) -> Result<()> {
        // Check expiration
        if self.expires_at < Utc::now() {
            return Err(SubscriptionError::LicenseExpired(self.expires_at));
        }

        // Verify signature
        self.verify(verifying_key)
    }

    /// Returns whether the license has expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

/// Key pair for license signing and verification.
#[derive(Debug)]
pub struct LicenseKeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl LicenseKeyPair {
    /// Generates a new random key pair.
    #[must_use]
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Creates a key pair from existing bytes.
    ///
    /// # Arguments
    /// * `secret_key_bytes` - 32-byte secret key
    ///
    /// # Returns
    /// A `LicenseKeyPair` or an error.
    pub fn from_bytes(secret_key_bytes: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(secret_key_bytes);
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Returns a reference to the signing key.
    #[must_use]
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    /// Returns a reference to the verifying key.
    #[must_use]
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Exports the signing key as base64-encoded bytes.
    #[must_use]
    pub fn export_secret(&self) -> String {
        BASE64.encode(self.signing_key.to_bytes())
    }

    /// Exports the verifying key as base64-encoded bytes.
    #[must_use]
    pub fn export_public(&self) -> String {
        BASE64.encode(self.verifying_key.to_bytes())
    }

    /// Creates a `VerifyingKey` from base64-encoded bytes.
    ///
    /// # Arguments
    /// * `public_key_b64` - Base64-encoded public key
    ///
    /// # Returns
    /// A `VerifyingKey` or an error.
    pub fn verifying_key_from_base64(public_key_b64: &str) -> Result<VerifyingKey> {
        let bytes = BASE64.decode(public_key_b64).map_err(|e| {
            SubscriptionError::LicenseValidation(format!("Invalid public key encoding: {e}"))
        })?;

        let key_bytes: [u8; 32] = bytes.try_into().map_err(|_| {
            SubscriptionError::LicenseValidation("Invalid public key length".to_string())
        })?;

        VerifyingKey::from_bytes(&key_bytes).map_err(|e| {
            SubscriptionError::LicenseValidation(format!("Invalid public key: {e}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_license_creation_and_verification() {
        let key_pair = LicenseKeyPair::generate();
        let expires_at = Utc::now() + Duration::days(30);

        let license = License::new(
            "TEST-LICENSE-001".to_string(),
            Tier::Pro,
            expires_at,
            key_pair.signing_key(),
        );

        assert_eq!(license.key, "TEST-LICENSE-001");
        assert_eq!(license.tier, Tier::Pro);
        assert!(!license.is_expired());
        assert!(license.verify(key_pair.verifying_key()).is_ok());
        assert!(license.validate(key_pair.verifying_key()).is_ok());
    }

    #[test]
    fn test_expired_license() {
        let key_pair = LicenseKeyPair::generate();
        let expires_at = Utc::now() - Duration::days(1);

        let license = License::new(
            "TEST-LICENSE-002".to_string(),
            Tier::Pro,
            expires_at,
            key_pair.signing_key(),
        );

        assert!(license.is_expired());
        assert!(license.verify(key_pair.verifying_key()).is_ok()); // Signature still valid
        assert!(license.validate(key_pair.verifying_key()).is_err()); // But license expired
    }

    #[test]
    fn test_invalid_signature() {
        let key_pair1 = LicenseKeyPair::generate();
        let key_pair2 = LicenseKeyPair::generate();
        let expires_at = Utc::now() + Duration::days(30);

        let license = License::new(
            "TEST-LICENSE-003".to_string(),
            Tier::Pro,
            expires_at,
            key_pair1.signing_key(),
        );

        // Verify with wrong key should fail
        assert!(license.verify(key_pair2.verifying_key()).is_err());
    }

    #[test]
    fn test_key_pair_export_import() {
        let key_pair = LicenseKeyPair::generate();
        let public_b64 = key_pair.export_public();

        let imported_verifying_key = LicenseKeyPair::verifying_key_from_base64(&public_b64).unwrap();

        let expires_at = Utc::now() + Duration::days(30);
        let license = License::new(
            "TEST-LICENSE-004".to_string(),
            Tier::Enterprise,
            expires_at,
            key_pair.signing_key(),
        );

        assert!(license.verify(&imported_verifying_key).is_ok());
    }

    #[test]
    fn test_license_serialization() {
        let key_pair = LicenseKeyPair::generate();
        let expires_at = Utc::now() + Duration::days(30);

        let license = License::new(
            "TEST-LICENSE-005".to_string(),
            Tier::Pro,
            expires_at,
            key_pair.signing_key(),
        );

        let json = serde_json::to_string(&license).unwrap();
        let parsed: License = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.key, license.key);
        assert_eq!(parsed.tier, license.tier);
        assert!(parsed.verify(key_pair.verifying_key()).is_ok());
    }
}
