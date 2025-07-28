use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct TracksQuery {
    pub q: Option<String>,
    pub ids: Option<String>,
    pub urns: Option<String>,
    pub genres: Option<String>,
    pub tags: Option<String>,
    pub bpm: Option<String>,
    pub duration: Option<String>,
    pub created_at: Option<String>,
    pub access: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct Paging {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}
