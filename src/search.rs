use crate::bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SearchOptions {
    pub(crate) query: Option<String>,
    limit: u64,
    offset: u64,
    filter: SearchFilter,
    sort: SearchSort,
    order: SearchOrder,
}

impl SearchOptions {
    pub fn query(self, query: String) -> Self {
        Self {
            query: Some(query),
            ..self
        }
    }
    pub fn limit(self, limit: u64) -> Self {
        Self { limit, ..self }
    }
    pub fn offset(self, offset: u64) -> Self {
        Self { offset, ..self }
    }
    pub fn filter(self, filter: SearchFilter) -> Self {
        Self { filter, ..self }
    }
    pub fn sort(self, sort: SearchSort) -> Self {
        Self { sort, ..self }
    }
    pub fn order(self, order: SearchOrder) -> Self {
        Self { order, ..self }
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            query: None,
            limit: 21,
            offset: 0,
            filter: SearchFilter::default(),
            sort: SearchSort::default(),
            order: SearchOrder::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    tags: Vec<String>,
    features: SearchFeatures,
    premium: bool,
    filter_mode: SearchFilterMode,
}

impl SearchFilter {
    pub fn tags(self, tags: Vec<String>) -> Self {
        Self { tags, ..self }
    }
    pub fn features(self, features: SearchFeatures) -> Self {
        Self { features, ..self }
    }
    pub fn premium(self, premium: bool) -> Self {
        Self { premium, ..self }
    }
    pub fn filter_mode(self, filter_mode: SearchFilterMode) -> Self {
        Self {
            filter_mode,
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SearchSort {
    Relevancy,
    Votes,
    Age,
    Trending,
    Popularity,
    Premium,
}

impl Default for SearchSort {
    fn default() -> Self {
        Self::Trending
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SearchOrder {
    #[serde(rename = "desc")]
    Descending,
    #[serde(rename = "asc")]
    Ascending,
}

impl Default for SearchOrder {
    fn default() -> Self {
        Self::Descending
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SearchFilterMode {
    Union,
    Intersection,
}

impl Default for SearchFilterMode {
    fn default() -> Self {
        Self::Intersection
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SearchFeatures: u64 {
        const PREFIX = 0b00000001;
        const LANGUAGE = 0b00000010;
        const COMMANDS = 0b00000100;
        const DASHBOARD = 0b00001000;
        const SLASH_COMMANDS = 0b00010000;
        const PAID_FEATURES = 0b00100000;
        const DOCUMENTATION = 0b01000000;
        const INTERACTIVE_BUTTONS = 0b10000000;
    }
}

impl Default for SearchFeatures {
    fn default() -> Self {
        Self::empty()
    }
}
