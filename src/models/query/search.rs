use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SearchResultsQuery {
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SearchAllQuery {
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}
