use serde::{Deserialize, Serialize};

use crate::{
    models::response::PagingCollection,
    response::{Track, User},
};

pub type Reposts = PagingCollection<Repost>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Repost {
    pub created_at: Option<String>,
    pub r#type: Option<String>,
    pub user: Option<User>,
    pub uuid: Option<String>,
    pub caption: Option<String>,
    pub track: Option<Track>,
}
