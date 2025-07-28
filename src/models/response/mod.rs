
mod tracks;
mod users;
mod playlists;

pub use tracks::*;
pub use users::*;
pub use playlists::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PagingCollection<T> {
    pub collection: Vec<T>,
}