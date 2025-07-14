use std::error::Error;

use tokio;
use soundcloud::{client, models::{self, query::tracks::Paging}};
use client::client::Client;
use models::query::tracks::TracksQuery;
use models::query::playlists::PlaylistsQuery;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await.unwrap();
    
    match client.get_playlists(Some(&PlaylistsQuery {
        q: Some("test".to_string()),
        access: Some("public".to_string()),
        show_tracks: Some(true),
        limit: Some(1),
        offset: Some(0),
        linked_partitioning: Some(true),
    })).await {
        Ok(playlists) => println!("Playlists: {:?}", playlists),
        Err(e) => eprintln!("Error fetching playlists: {}", e),
    }

    Ok(())
} 