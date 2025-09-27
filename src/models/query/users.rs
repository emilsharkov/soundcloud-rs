use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UsersQuery {
    pub q: Option<String>,
    pub ids: Option<String>,
    pub urns: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub linked_partitioning: Option<bool>,
}
