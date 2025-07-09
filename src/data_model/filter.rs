use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FilterCondition {
    Match { key: String, value: String },
    Exists { key: String },
    Must(Vec<FilterCondition>),
    Should(Vec<FilterCondition>),
    MustNot(Vec<FilterCondition>),
}

impl FilterCondition {
    pub fn matches(&self, metadata: Option<&HashMap<String, String>>) -> bool {
        match self {
            FilterCondition::Match { key, value } => {
                metadata.as_ref()
                    .and_then(|data| data.get(key))
                    .map_or(false, |v| v == value)
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
            FilterCondition::Exists { key } => {
                metadata.as_ref()
                .map_or(false, |data| data.contains_key(key))
            }
        }
    }
}