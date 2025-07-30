
mod tracks;
mod users;
mod playlists;
mod search;
pub use tracks::*;
pub use users::*;
pub use playlists::*;
pub use search::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagingCollection<T> {
    pub collection: Vec<T>,
}