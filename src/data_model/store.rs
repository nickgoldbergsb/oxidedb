use super::item::Item;

pub struct VectorStore {
    pub items: HashMap<String, Item>
}

impl VectorStore {
    pub fn new(items: HashMap<String, Item>) -> Self {
        VectorStore { items }
    }

    pub fn upsert(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item)
    }

    pub fn delete(&mut self, id: &str) {
        self.items.remove(id)
    }

    pub fn get(&mut self, &item: Item) -> Option<&item> {
        self.items.get(id)
    }
}