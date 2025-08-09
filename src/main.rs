use std::error::Error;

use tokio;
use soundcloud_rs::{query::{TracksQuery}, response::StreamType, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await?;
    let query = TracksQuery {
        q: Some("fluxxwave".to_string()),
        limit: Some(5),
        ..Default::default()
    };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().unwrap();
    client.download_track(&first_track, Some(StreamType::Progressive), Some("./downloads"), None).await?;
    Ok(())
} 