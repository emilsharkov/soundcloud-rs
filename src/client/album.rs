use std::error::Error;

use crate::{models::query::AlbumQuery, response::Playlists, Client};

impl Client {
    pub async fn search_albums(&self, query: Option<&AlbumQuery>) -> Result<Playlists, Box<dyn Error>> {
        let resp: Playlists = self.get("search/albums", query).await?;
        Ok(resp)
    }
}