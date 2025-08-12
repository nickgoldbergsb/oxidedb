use serde::{Deserialize, Serialize};

use super::filter::FilterCondition;
use super::item::Item;
use super::similarity_metrics::metric::SimilarityMetric;
use super::vector::Vector;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorStore {
    items: HashMap<String, Item>,
}

impl VectorStore {
    pub fn new(items: HashMap<String, Item>) -> Self {
        VectorStore { items }
    }

    pub fn upsert(&mut self, item: Item) {
        self.items.insert(item.id().to_string(), item);
    }

    pub fn delete(&mut self, id: &str) {
        self.items.remove(id);
    }

    pub fn get(&self, id: &str) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn search_top_k(
        &self,
        vector: &Vector,
        k: usize,
        metric: &impl SimilarityMetric,
    ) -> Vec<(Item, f32)> {
        let mut heap = BinaryHeap::new();

        for item in self.items.values() {
            if let Some(score) = metric.compute(item.vector(), vector) {
                heap.push(HeapElement {
                    item: item.clone(),
                    score,
                });
            }

            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|element| (element.item, element.score))
            .collect()
    }

    pub fn filter(&self, filters: &FilterCondition) -> Vec<&Item> {
        self.items
            .values()
            .filter(|item| filters.matches(item.metadata()))
            .collect()
    }

    pub fn search_top_k_with_filter(
        &self,
        vector: &Vector,
        k: usize,
        metric: &impl SimilarityMetric,
        filters: Option<&FilterCondition>,
    ) -> Vec<(Item, f32)> {
        let data = match filters {
            Some(f) => self.filter(f),
            None => self.items.values().collect(),
        };

        let mut heap = BinaryHeap::new();

        for item in data {
            if let Some(score) = metric.compute(item.vector(), vector) {
                heap.push(HeapElement {
                    item: item.clone(),
                    score,
                });
            }

            if heap.len() > k {
                heap.pop();
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|element| (element.item, element.score))
            .collect()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct HeapElement {
    item: Item,
    score: f32,
}

impl Eq for HeapElement {}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
