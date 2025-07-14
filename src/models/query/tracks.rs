use serde::Serialize;

#[derive(Debug, Serialize)]
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


#[derive(Debug, Serialize)]
pub struct BpmRange {
    pub from: Option<u32>,
    pub to: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DurationRange {
    pub from: Option<u32>,
    pub to: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DateRange {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Paging {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}
