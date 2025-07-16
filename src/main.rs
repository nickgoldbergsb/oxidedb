mod vector_core;

use vector_core::filter::FilterCondition;
use vector_core::item::{Item, MetadataValue};
use vector_core::store::VectorStore;
use vector_core::vector::Vector;
use std::collections::HashMap;

use crate::vector_core::similarity_metrics::cosine::Cosine;

fn main() {
    // Dummy metadata to exercise MetadataValue variants
    let mut metadata = HashMap::new();
    metadata.insert(
        "key1".to_string(),
        MetadataValue::StringValue("value".to_string()),
    );
    metadata.insert("key2".to_string(), MetadataValue::FloatValue(2.14));
    metadata.insert("key3".to_string(), MetadataValue::BoolValue(true));
    metadata.insert(
        "key4".to_string(),
        MetadataValue::StringArray(vec!["a".into(), "b".into()]),
    );

    // Create a dummy item
    let item = Item::new(
        "item1".to_string(),
        Vector::new(vec![1.0, 2.0, 3.0]),
        Some(metadata.clone()),
    );

    // Call metadata method
    let _ = item.metadata();

    // Create a filter condition and call matches
    let filter = FilterCondition::Contains {
        key: "key1".to_string(),
        value: "value".to_string(),
    };

    let _ = FilterCondition::Match {
        key: "foo".to_string(),
        value: MetadataValue::StringValue("bar".to_string()),
    };

    // Use NotMatch variant
    let _ = FilterCondition::NotMatch {
        key: "foo".to_string(),
        value: MetadataValue::StringValue("baz".to_string()),
    };

    // Use In variant
    let _ = FilterCondition::In {
        key: "foo".to_string(),
        values: vec![
            MetadataValue::StringValue("bar".to_string()),
            MetadataValue::StringValue("baz".to_string()),
        ],
    };

    // Use Exists variant
    let _ = FilterCondition::Exists {
        key: "foo".to_string(),
        exists: true,
    };

    // Use Range variant
    let _ = FilterCondition::Range {
        key: "foo".to_string(),
        gte: Some(1.0),
        lte: Some(5.0),
        lt: None,
        gt: None,
    };

    // Use Must, Should, MustNot with nested conditions
    let nested = FilterCondition::Match {
        key: "nested".to_string(),
        value: MetadataValue::StringValue("value".to_string()),
    };

    let _ = FilterCondition::Must(vec![nested.clone()]);
    let _ = FilterCondition::Should(vec![nested.clone()]);
    let _ = FilterCondition::MustNot(vec![nested]);

    let _ = filter.matches(item.metadata());

    // Create a VectorStore and use filter methods
    let mut store = VectorStore::new(HashMap::new());
    store.upsert(item);
    let _ = store.filter(&filter);
    let metric = Cosine;
    let query: Vector = Vector::new(vec![1.0, 2.0, 3.0]);
    let _ = store.search_top_k(&query, 1, &metric);
    let _ = store.search_top_k_with_filter(&query, 1, &metric, Some(&filter));

    let _ = store.get("item1");
    store.delete("item1");
}
