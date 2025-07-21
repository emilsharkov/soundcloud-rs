// src/tracks.rs
use std::error::Error;
use crate::client::client::Client;
use crate::models::query::tracks::{TracksQuery, Paging};
use crate::models::response::tracks::{Track, Tracks};

impl Client {
    pub async fn get_tracks(&self, query: Option<&TracksQuery>) -> Result<Tracks, Box<dyn Error>> {
        let tracks: Tracks = self.get("search/tracks", query).await?;
        Ok(tracks)
    }

    pub async fn get_track_by_id(&self, id: &str) -> Result<Track, Box<dyn Error>> {
        let url = format!("tracks/{}", id);
        let resp: Track = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_track_by_urn(&self, urn: &str) -> Result<Track, Box<dyn Error>> {
        let url = format!("tracks/{}", urn);
        let resp: Track = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_track_related_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("tracks/{}/related", id);
        let resp: Tracks  = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_track_related_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("tracks/{}/related", urn);
        let resp: Tracks  = self.get(&url, pagination).await?;
        Ok(resp)
    }
}