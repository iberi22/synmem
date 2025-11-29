//! JSON schema definitions and validation for scraper packages.

use crate::domain::entities::{PricingModel, SchemaDefinition, ScraperPackage, ScraperStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Errors that can occur during schema validation.
#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid field value for '{0}': {1}")]
    InvalidFieldValue(String, String),
    #[error("Schema validation failed: {0}")]
    ValidationFailed(String),
}

/// Result type for schema operations.
pub type SchemaResult<T> = Result<T, SchemaValidationError>;

/// JSON representation of a scraper package for submission.
///
/// This is the format users use when submitting scrapers to the marketplace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperPackageSubmission {
    /// Package name (lowercase, hyphen-separated)
    pub name: String,
    /// Semantic version
    pub version: String,
    /// Author username
    pub author: String,
    /// Price in USD (None or 0 for free)
    #[serde(default)]
    pub price: Option<f64>,
    /// List of supported sites/domains
    pub sites: Vec<String>,
    /// Human-readable description
    pub description: String,
    /// Output schema definition
    pub schema: SchemaDefinitionJson,
    /// Optional repository URL
    #[serde(default)]
    pub repository_url: Option<String>,
    /// Optional documentation URL
    #[serde(default)]
    pub documentation_url: Option<String>,
    /// Optional tags for discovery
    #[serde(default)]
    pub tags: Vec<String>,
}

/// JSON representation of a schema definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinitionJson {
    /// Output schema as a JSON object
    pub output: Value,
}

impl ScraperPackageSubmission {
    /// Validates the submission and converts it to a domain entity.
    pub fn validate_and_convert(self) -> SchemaResult<ScraperPackage> {
        // Validate name format
        if self.name.is_empty() {
            return Err(SchemaValidationError::MissingField("name".to_string()));
        }

        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        {
            return Err(SchemaValidationError::InvalidFieldValue(
                "name".to_string(),
                "must be lowercase with hyphens only".to_string(),
            ));
        }

        // Validate version format using semver crate
        if self.version.is_empty() {
            return Err(SchemaValidationError::MissingField("version".to_string()));
        }

        if semver::Version::parse(&self.version).is_err() {
            return Err(SchemaValidationError::InvalidFieldValue(
                "version".to_string(),
                "must be in valid semver format (e.g., 1.0.0)".to_string(),
            ));
        }

        // Validate author
        if self.author.is_empty() {
            return Err(SchemaValidationError::MissingField("author".to_string()));
        }

        // Validate sites
        if self.sites.is_empty() {
            return Err(SchemaValidationError::MissingField("sites".to_string()));
        }

        // Validate description
        if self.description.is_empty() {
            return Err(SchemaValidationError::MissingField(
                "description".to_string(),
            ));
        }

        // Convert schema
        let schema = convert_schema_json(&self.schema)?;

        // Create the package
        let pricing = match self.price {
            Some(p) if p > 0.0 => PricingModel::Paid { price: p },
            _ => PricingModel::Free,
        };

        let mut package = ScraperPackage::new(
            self.name,
            self.version,
            self.author,
            self.sites,
            self.description,
            schema,
        );

        package.pricing = pricing;
        package.status = ScraperStatus::PendingReview;
        package.repository_url = self.repository_url;
        package.documentation_url = self.documentation_url;
        package.tags = self.tags;

        Ok(package)
    }
}

/// Converts a JSON schema definition to a domain schema.
fn convert_schema_json(json: &SchemaDefinitionJson) -> SchemaResult<SchemaDefinition> {
    // For now, we store the output schema as-is
    // In a full implementation, we would parse and validate each field
    let output_str = serde_json::to_string(&json.output).map_err(|e| {
        SchemaValidationError::InvalidJson(format!("Failed to serialize output schema: {}", e))
    })?;

    let output_map: std::collections::HashMap<String, crate::domain::entities::SchemaField> =
        serde_json::from_str(&output_str).map_err(|e| {
            SchemaValidationError::InvalidJson(format!(
                "Failed to parse output schema: {}",
                e
            ))
        })?;

    Ok(SchemaDefinition { output: output_map })
}

/// Parses a JSON string into a scraper package submission.
pub fn parse_package_json(json: &str) -> SchemaResult<ScraperPackageSubmission> {
    serde_json::from_str(json)
        .map_err(|e| SchemaValidationError::InvalidJson(format!("JSON parse error: {}", e)))
}

/// Parses and validates a JSON string, returning a domain entity.
pub fn parse_and_validate_package(json: &str) -> SchemaResult<ScraperPackage> {
    let submission = parse_package_json(json)?;
    submission.validate_and_convert()
}

/// Serializes a scraper package to JSON format.
pub fn package_to_json(package: &ScraperPackage) -> SchemaResult<String> {
    let price = match &package.pricing {
        PricingModel::Free => None,
        PricingModel::Paid { price } => Some(*price),
    };

    let submission = ScraperPackageSubmission {
        name: package.name.clone(),
        version: package.version.clone(),
        author: package.author.clone(),
        price,
        sites: package.sites.clone(),
        description: package.description.clone(),
        schema: SchemaDefinitionJson {
            output: serde_json::to_value(&package.schema.output)
                .unwrap_or(Value::Object(Default::default())),
        },
        repository_url: package.repository_url.clone(),
        documentation_url: package.documentation_url.clone(),
        tags: package.tags.clone(),
    };

    serde_json::to_string_pretty(&submission)
        .map_err(|e| SchemaValidationError::InvalidJson(format!("Serialization error: {}", e)))
}

/// Example JSON format for documentation.
pub const PACKAGE_JSON_EXAMPLE: &str = r#"{
  "name": "linkedin-profile-scraper",
  "version": "1.0.0",
  "author": "username",
  "price": 5.00,
  "sites": ["linkedin.com"],
  "description": "Extract profile data from LinkedIn",
  "schema": {
    "output": {
      "name": { "type": "string", "required": true },
      "headline": { "type": "string" },
      "experience": { "type": "array", "items": { "type": "string" } }
    }
  },
  "tags": ["linkedin", "profile", "professional"]
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_json() {
        let result = parse_package_json(PACKAGE_JSON_EXAMPLE);
        assert!(result.is_ok());

        let submission = result.unwrap();
        assert_eq!(submission.name, "linkedin-profile-scraper");
        assert_eq!(submission.version, "1.0.0");
        assert_eq!(submission.price, Some(5.00));
        assert_eq!(submission.sites, vec!["linkedin.com"]);
    }

    #[test]
    fn test_validate_and_convert() {
        let result = parse_and_validate_package(PACKAGE_JSON_EXAMPLE);
        assert!(result.is_ok());

        let package = result.unwrap();
        assert_eq!(package.name, "linkedin-profile-scraper");
        assert!(!package.is_free());
        assert_eq!(package.price(), Some(5.00));
    }

    #[test]
    fn test_free_package() {
        let json = r#"{
            "name": "free-scraper",
            "version": "1.0.0",
            "author": "user",
            "sites": ["example.com"],
            "description": "A free scraper",
            "schema": { "output": {} }
        }"#;

        let package = parse_and_validate_package(json).unwrap();
        assert!(package.is_free());
        assert_eq!(package.price(), None);
    }

    #[test]
    fn test_invalid_name() {
        let json = r#"{
            "name": "Invalid Name",
            "version": "1.0.0",
            "author": "user",
            "sites": ["example.com"],
            "description": "A scraper",
            "schema": { "output": {} }
        }"#;

        let result = parse_and_validate_package(json);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchemaValidationError::InvalidFieldValue(_, _)
        ));
    }

    #[test]
    fn test_missing_sites() {
        let json = r#"{
            "name": "test-scraper",
            "version": "1.0.0",
            "author": "user",
            "sites": [],
            "description": "A scraper",
            "schema": { "output": {} }
        }"#;

        let result = parse_and_validate_package(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_package_to_json() {
        let package = parse_and_validate_package(PACKAGE_JSON_EXAMPLE).unwrap();
        let json = package_to_json(&package);
        assert!(json.is_ok());

        // Parse again to verify round-trip
        let reparsed = parse_and_validate_package(&json.unwrap());
        assert!(reparsed.is_ok());
    }
}
