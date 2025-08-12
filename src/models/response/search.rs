use serde::{Deserialize, Serialize};

use crate::response::{PagingCollection, Playlist, Track, User};

pub type SearchResultsResponse = PagingCollection<SearchResult>;
pub type SearchAllResponse = PagingCollection<SearchAllResult>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SearchResult {
    pub output: Option<String>,
    pub query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SearchAllResult {
    Track(Track),
    User(User),
    Playlist(Playlist),
}
