use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlaylistsQuery {
    pub q: Option<String>,
    pub access: Option<String>,
    pub show_tracks: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub linked_partitioning: Option<bool>,
}
