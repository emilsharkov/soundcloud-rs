use serde::Deserialize;

pub mod tracks;
pub mod users;
pub mod playlists;

#[derive(Debug, Deserialize)]
pub struct PagingCollection<T> {
    pub collection: Vec<T>,
    pub next_href: Option<String>,
}