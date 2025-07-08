use super::vector::Vector;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Item {
    id: String,
    vector: Vector,
    metadata: Option<HashMap<String, String>>
}

impl Item {
    pub fn new(id: String, vector: Vector, metadata: Option<HashMap<String, String>>) -> Self {
        Self { id, vector, metadata }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }

    pub fn get_metadata(&self) -> Option<&HashMap<String, String>> {
        self.metadata.as_ref()
    }
}