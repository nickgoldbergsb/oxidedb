use super::vector::Vector;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    id: String,
    vector: Vector,
    metadata: Option<HashMap<String, MetadataValue>>,
}

impl Item {
    pub fn new(id: String, vector: Vector, metadata: Option<HashMap<String, MetadataValue>>) -> Self {
        Self { id, vector, metadata }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }

    pub fn get_metadata(&self) -> Option<&HashMap<String, MetadataValue>> {
        self.metadata.as_ref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MetadataValue {
    StringValue(String),
    FloatValue(f64),
    BoolValue(bool),
    StringArray(Vec<String>),
}