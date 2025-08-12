use std::error::Error;

use soundcloud_rs::{Client, query::TracksQuery, response::StreamType};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await?;
    let query = TracksQuery {
        q: Some("fluxxwave".to_string()),
        limit: Some(5),
        ..Default::default()
    };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("No tracks found");
    let waveform = client.get_track_waveform(&first_track).await?;
    println!("{:?}", waveform);
    // client.download_track(&first_track, Some(&StreamType::Progressive), Some("./downloads"), None).await?;
    Ok(())
}
