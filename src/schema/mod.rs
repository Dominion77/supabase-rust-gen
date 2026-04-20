mod fetch;
mod parse;

pub use fetch::SchemaFetcher;
pub use parse::{ParsedSchema, TableDefinition, ColumnDefinition};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub definitions: std::collections::HashMap<String, Definition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    #[serde(rename = "type")]
    pub def_type: String,
    pub properties: Option<std::collections::HashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub prop_type: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub items: Option<Box<Property>>,
}