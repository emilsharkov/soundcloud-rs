use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AlbumQuery {
    pub q: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub linked_partitioning: Option<bool>,
}
