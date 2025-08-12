mod playlists;
mod search;
mod tracks;
mod users;
pub use playlists::*;
pub use search::*;
pub use tracks::*;
pub use users::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PagingCollection<T> {
    pub collection: Vec<T>,
}
