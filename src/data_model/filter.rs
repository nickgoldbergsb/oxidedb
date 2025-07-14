use std::collections::HashMap;

use crate::data_model::item::MetadataValue;

#[derive(Debug, Clone)]
pub enum FilterCondition {
    Match { key: String, value: MetadataValue },
    Exists { key: String },
    Contains { key: String, value: String },
    Range { key: String, lte: Option<f64>, gte: Option<f64> },
    Must(Vec<FilterCondition>),
    Should(Vec<FilterCondition>),
    MustNot(Vec<FilterCondition>),
}

impl FilterCondition {
    pub fn matches(&self, metadata: Option<&HashMap<String, MetadataValue>>) -> bool {
        match self {
            FilterCondition::Match { key, value } => {
                metadata.as_ref()
                    .and_then(|data| data.get(key))
                    .map_or(false, |v| v == value)
            }
            FilterCondition::Exists { key } => {
                metadata.as_ref()
                .map_or(false, |data| data.contains_key(key))
            }
            FilterCondition::Contains { key, value } => {
                metadata
                .and_then(|data| data.get(key))
                .map_or(false, |v| match v {
                    MetadataValue::StringArray(arr) => arr.contains(value),
                    _ => false,
                })
            }
            FilterCondition::Range { key, lte, gte } => {
                match metadata.and_then(|data| data.get(key)) {
                    Some(MetadataValue::FloatValue(float)) => {
                        let lte_ok = lte.map_or(true, |limit| *float <= limit);
                        let gte_ok = gte.map_or(true, |limit| *float >= limit);
                        lte_ok && gte_ok
                    }
                    _ => false,
                }
            }
            FilterCondition::Must(filters) => {
                filters.iter().all(|f| f.matches(metadata))
            }
            FilterCondition::Should(filters) => {
                filters.iter().any(|f| f.matches(metadata))

            }
            FilterCondition::MustNot(filters) => {
                !filters.iter().all(|f| f.matches(metadata))
            }
        }
    }
}