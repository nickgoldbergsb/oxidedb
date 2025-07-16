use std::collections::HashMap;

use crate::data_model::item::MetadataValue;

#[derive(Debug, Clone)]
pub enum FilterCondition {
    Match {
        key: String,
        value: MetadataValue,
    },
    NotMatch {
        key: String,
        value: MetadataValue,
    },
    In {
        key: String,
        values: Vec<MetadataValue>,
    },
    Exists {
        key: String,
        exists: bool,
    },
    Contains {
        key: String,
        value: String,
    },
    Range {
        key: String,
        lte: Option<f64>,
        gte: Option<f64>,
        lt: Option<f64>,
        gt: Option<f64>,
    },
    Must(Vec<FilterCondition>),
    Should(Vec<FilterCondition>),
    MustNot(Vec<FilterCondition>),
}

impl FilterCondition {
    pub fn matches(&self, metadata: Option<&HashMap<String, MetadataValue>>) -> bool {
        match self {
            FilterCondition::Match { key, value } => {
                metadata.as_ref().and_then(|data| data.get(key)) == Some(value)
            }
            FilterCondition::NotMatch { key, value } => {
                metadata.as_ref().and_then(|data| data.get(key)) != Some(value)
            }
            FilterCondition::In { key, values } => metadata
                .as_ref()
                .and_then(|data| data.get(key))
                .is_some_and(|v| values.contains(v)),
            FilterCondition::Exists { key, exists } => metadata
                .as_ref()
                .is_some_and(|data| data.contains_key(key) == *exists),
            FilterCondition::Contains { key, value } => metadata
                .and_then(|data| data.get(key))
                .is_some_and(|v| match v {
                    MetadataValue::StringArray(arr) => arr.contains(value),
                    _ => false,
                }),
            FilterCondition::Range {
                key,
                lte,
                gte,
                lt,
                gt,
            } => match metadata.and_then(|data| data.get(key)) {
                Some(MetadataValue::FloatValue(float)) => {
                    let lte_ok = lte.is_none_or(|limit| *float <= limit);
                    let gte_ok = gte.is_none_or(|limit| *float >= limit);
                    let lt_ok = lt.is_none_or(|limit| *float < limit);
                    let gt_ok = gt.is_none_or(|limit| *float > limit);
                    lte_ok && gte_ok && lt_ok && gt_ok
                }
                _ => false,
            },
            FilterCondition::Must(filters) => filters.iter().all(|f| f.matches(metadata)),
            FilterCondition::Should(filters) => filters.iter().any(|f| f.matches(metadata)),
            FilterCondition::MustNot(filters) => filters.iter().all(|f| !f.matches(metadata)),
        }
    }
}
