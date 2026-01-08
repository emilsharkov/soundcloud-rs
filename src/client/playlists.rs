use crate::models::client::Client;
use crate::models::client::Identifier;
use crate::models::error::Error;
use crate::models::query::{Paging, PlaylistsQuery};
use crate::models::response::{Playlist, Playlists, Users};
use std::path::PathBuf;

impl Client {
    pub async fn search_playlists(
        &self,
        query: Option<&PlaylistsQuery>,
    ) -> Result<Playlists, Error> {
        let resp: Playlists = self.get("search/playlists", query).await?;
        Ok(resp)
    }

    pub async fn get_playlist(
        &self,
        identifier: &Identifier,
    ) -> Result<Playlist, Error> {
        let url = format!("playlists/{identifier}");
        let resp: Playlist = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_playlist_reposters(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Users, Error> {
        let url = format!("playlists/{identifier}/reposters");
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn download_playlist(
        &self,
        identifier: &Identifier,
        destination: Option<&str>,
        playlist_name: Option<&str>,
    ) -> Result<(), Error> {
        let playlist = self.get_playlist(identifier).await?;

        let playlist_title = match playlist_name {
            Some(playlist_name) => playlist_name,
            None => playlist.title.as_ref().expect("Missing playlist title"),
        };

        let output_path = match destination {
            Some(destination) => PathBuf::from(destination).join(playlist_title),
            None => PathBuf::from(playlist_title),
        };
        if !output_path.exists() {
            std::fs::create_dir_all(&output_path)?;
        }

        let output_path_str = output_path
            .to_str()
            .expect("Failed to convert output path to string");
        let tracks = playlist.tracks.as_ref().expect("Missing tracks");
        for track in tracks {
            let identifier = track.id.as_ref().expect("Missing track id");
            if let Err(e) = self
                .download_track(
                    &Identifier::Id(*identifier),
                    None,
                    Some(output_path_str),
                    None,
                )
                .await
            {
                println!("Error downloading track: {e}")
            }
        }

        Ok(())
    }
}
