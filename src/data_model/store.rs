use super::vector::Vector;
use super::item::Item;
use super::metrics::metric::SimilarityMetric;

// use std::collections::HashMap;
// use std::collections::BinaryHeap;
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

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

    pub fn search_top_k(&self, vector: &Vector, k: usize, metric: &impl SimilarityMetric) -> Vec<(Item, f32)> {
        let mut heap = BinaryHeap::new();

        for (_id, item) in &self.items {
            if let Some(score) =  metric.compute(item.get_vector(), vector) {
                heap.push(
                    HeapElement {
                        item: item.clone(),
                        score
                    }
                );
            }

            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|(element)| (element.item, element.score))
            .collect()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct HeapElement {
    item: Item,
    score: f32
}

impl Eq for HeapElement {}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}