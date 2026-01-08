use crate::{Client, models::query::AlbumQuery, models::error::Error, response::Playlists};

impl Client {
    pub async fn search_albums(
        &self,
        query: Option<&AlbumQuery>,
    ) -> Result<Playlists, Error> {
        let resp: Playlists = self.get("search/albums", query).await?;
        Ok(resp)
    }
}
