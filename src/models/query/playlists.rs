use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlaylistsQuery {
    pub q: Option<String>,
    pub access: Option<String>,
    pub show_tracks: Option<bool>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub linked_partitioning: Option<bool>,
}
