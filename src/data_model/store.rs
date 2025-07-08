use super::item::Item;

use std::collections::HashMap;

#[derive(Debug)]
pub struct VectorStore {
    items: HashMap<String, Item>
}

impl VectorStore {
    pub fn new(items: HashMap<String, Item>) -> Self {
        VectorStore { items }
    }

    pub fn upsert(&mut self, item: Item) {
        self.items.insert(item.get_id().clone(), item);
    }

    pub fn delete(&mut self, id: &str) {
        self.items.remove(id);
    }

    pub fn get(&self, id: &str) -> Option<&Item> {
        self.items.get(id)
    }
}