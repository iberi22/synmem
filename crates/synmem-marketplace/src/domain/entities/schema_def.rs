//! Schema definition for scraper output.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of a schema field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaFieldType {
    /// String value
    String,
    /// Numeric value (integer or float)
    Number,
    /// Boolean value
    Boolean,
    /// Array of values
    Array,
    /// Nested object
    Object,
    /// Any type
    Any,
}

impl Default for SchemaFieldType {
    fn default() -> Self {
        Self::String
    }
}

/// A field in the schema definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    /// Field type
    #[serde(rename = "type")]
    pub field_type: SchemaFieldType,
    /// Optional description
    pub description: Option<String>,
    /// Whether the field is required
    #[serde(default)]
    pub required: bool,
    /// For array types, the type of items
    pub items: Option<Box<SchemaField>>,
    /// For object types, nested properties
    pub properties: Option<HashMap<String, SchemaField>>,
}

impl SchemaField {
    /// Creates a new string field.
    pub fn string() -> Self {
        Self {
            field_type: SchemaFieldType::String,
            description: None,
            required: false,
            items: None,
            properties: None,
        }
    }

    /// Creates a new required string field.
    pub fn required_string() -> Self {
        Self {
            field_type: SchemaFieldType::String,
            description: None,
            required: true,
            items: None,
            properties: None,
        }
    }

    /// Creates a new array field.
    pub fn array(item_type: SchemaField) -> Self {
        Self {
            field_type: SchemaFieldType::Array,
            description: None,
            required: false,
            items: Some(Box::new(item_type)),
            properties: None,
        }
    }

    /// Creates a new object field.
    pub fn object(properties: HashMap<String, SchemaField>) -> Self {
        Self {
            field_type: SchemaFieldType::Object,
            description: None,
            required: false,
            items: None,
            properties: Some(properties),
        }
    }

    /// Adds a description to the field.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Makes the field required.
    pub fn as_required(mut self) -> Self {
        self.required = true;
        self
    }
}

/// Schema definition for scraper output.
///
/// Defines the expected structure of data extracted by a scraper.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDefinition {
    /// Output schema fields
    pub output: HashMap<String, SchemaField>,
}

impl SchemaDefinition {
    /// Creates a new empty schema definition.
    pub fn new() -> Self {
        Self {
            output: HashMap::new(),
        }
    }

    /// Adds a field to the output schema.
    pub fn with_field(mut self, name: &str, field: SchemaField) -> Self {
        self.output.insert(name.to_string(), field);
        self
    }

    /// Creates a schema for a LinkedIn profile scraper.
    pub fn linkedin_profile() -> Self {
        let mut schema = Self::new();
        schema.output.insert(
            "name".to_string(),
            SchemaField::required_string().with_description("Full name"),
        );
        schema.output.insert(
            "headline".to_string(),
            SchemaField::string().with_description("Professional headline"),
        );
        schema.output.insert(
            "experience".to_string(),
            SchemaField::array(SchemaField::string()).with_description("Work experience"),
        );
        schema.output.insert(
            "education".to_string(),
            SchemaField::array(SchemaField::string()).with_description("Education history"),
        );
        schema.output.insert(
            "skills".to_string(),
            SchemaField::array(SchemaField::string()).with_description("Skills list"),
        );
        schema
    }

    /// Creates a schema for an Amazon product scraper.
    pub fn amazon_product() -> Self {
        let mut schema = Self::new();
        schema.output.insert(
            "title".to_string(),
            SchemaField::required_string().with_description("Product title"),
        );
        schema.output.insert(
            "price".to_string(),
            SchemaField {
                field_type: SchemaFieldType::Number,
                description: Some("Product price".to_string()),
                required: true,
                items: None,
                properties: None,
            },
        );
        schema.output.insert(
            "rating".to_string(),
            SchemaField {
                field_type: SchemaFieldType::Number,
                description: Some("Average rating".to_string()),
                required: false,
                items: None,
                properties: None,
            },
        );
        schema.output.insert(
            "reviews_count".to_string(),
            SchemaField {
                field_type: SchemaFieldType::Number,
                description: Some("Number of reviews".to_string()),
                required: false,
                items: None,
                properties: None,
            },
        );
        schema.output.insert(
            "description".to_string(),
            SchemaField::string().with_description("Product description"),
        );
        schema.output.insert(
            "images".to_string(),
            SchemaField::array(SchemaField::string()).with_description("Product images URLs"),
        );
        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_definition() {
        let schema = SchemaDefinition::new()
            .with_field("name", SchemaField::required_string())
            .with_field("tags", SchemaField::array(SchemaField::string()));

        assert!(schema.output.contains_key("name"));
        assert!(schema.output.contains_key("tags"));
        assert!(schema.output.get("name").unwrap().required);
    }

    #[test]
    fn test_linkedin_schema() {
        let schema = SchemaDefinition::linkedin_profile();
        assert!(schema.output.contains_key("name"));
        assert!(schema.output.contains_key("experience"));
        assert_eq!(
            schema.output.get("experience").unwrap().field_type,
            SchemaFieldType::Array
        );
    }

    #[test]
    fn test_serialization() {
        let schema = SchemaDefinition::linkedin_profile();
        let json = serde_json::to_string_pretty(&schema).unwrap();
        assert!(json.contains("name"));
        assert!(json.contains("experience"));

        // Deserialize back
        let deserialized: SchemaDefinition = serde_json::from_str(&json).unwrap();
        assert!(deserialized.output.contains_key("name"));
    }
}
