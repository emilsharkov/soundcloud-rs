use std::error::Error;
use crate::client::client::Client;
use crate::models::query::playlists::PlaylistsQuery;
use crate::models::query::tracks::Paging;
use crate::models::response::playlists::{Playlist, Playlists};
use crate::models::response::tracks::Tracks;
use crate::models::response::users::Users;

impl Client {
    pub async fn get_playlists(&self, query: Option<&PlaylistsQuery>) -> Result<Playlists, Box<dyn Error>> {
        let resp: Playlists = self.get("search/playlists", query).await?;
        Ok(resp)
    }

    pub async fn get_playlist_by_id(&self, id: &str) -> Result<Playlist, Box<dyn Error>> {
        let url = format!("playlists/{}", id);
        let resp: Playlist = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_playlist_by_urn(&self, urn: &str) -> Result<Playlist, Box<dyn Error>> {
        let url = format!("playlists/{}", urn);
        let resp: Playlist = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_playlist_tracks_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("playlists/{}/tracks", id);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_playlist_tracks_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("playlists/{}/tracks", urn);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_playlist_reposters_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("playlists/{}/reposters", id);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_playlist_reposters_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("playlists/{}/reposters", urn);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }
}