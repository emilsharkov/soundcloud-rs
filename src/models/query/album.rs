use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct AlbumQuery {
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}