use std::error::Error;
use std::path::PathBuf;
use crate::client::client::Client;
use crate::models::query::{PlaylistsQuery, Paging};
use crate::models::response::{Playlist, Playlists, Users};

impl Client {
    pub async fn search_playlists(&self, query: Option<&PlaylistsQuery>) -> Result<Playlists, Box<dyn Error>> {
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

    pub async fn download_playlist(&self, playlist: &Playlist, destination: Option<&str>, playlist_name: Option<&str>) -> Result<(), Box<dyn Error>> {
        let playlist_title = match playlist_name {
            Some(playlist_name) => playlist_name,
            None => playlist.title.as_ref().unwrap(),
        };
        
        let output_path = match destination {
            Some(destination) => PathBuf::from(destination).join(playlist_title),
            None => PathBuf::from(playlist_title),
        };
        if !output_path.exists() {
            std::fs::create_dir_all(&output_path)?;
        }

        let output_path_str = output_path.to_str().unwrap();
        let tracks = playlist.tracks.as_ref().unwrap();
        for track in tracks {
            match self.download_track(track, None, Some(output_path_str), None).await {
                Err(e) => println!("Error downloading track: {}", e),
                _ => (),
            }
        }

        Ok(())
    }
}