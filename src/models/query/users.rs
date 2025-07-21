use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UsersQuery {
    pub q: Option<String>,
    pub ids: Option<String>,
    pub urns: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}