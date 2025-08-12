use super::vector::Vector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Item {
    id: String,
    vector: Vector,
    metadata: Option<HashMap<String, MetadataValue>>,
}

impl Item {
    pub fn new(
        id: String,
        vector: Vector,
        metadata: Option<HashMap<String, MetadataValue>>,
    ) -> Self {
        Self {
            id,
            vector,
            metadata,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn vector(&self) -> &Vector {
        &self.vector
    }

    pub fn metadata(&self) -> Option<&HashMap<String, MetadataValue>> {
        self.metadata.as_ref()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MetadataValue {
    StringValue(String),
    FloatValue(f64),
    BoolValue(bool),
    StringArray(Vec<String>),
}
