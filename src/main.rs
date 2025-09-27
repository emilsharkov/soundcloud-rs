use std::error::Error;

use soundcloud_rs::{Client, query::TracksQuery, response::StreamType};

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
    let first_track_id = first_track.id.expect("No track id found");
    client
        .download_track(
            &first_track_id,
            Some(&StreamType::Progressive),
            Some("./downloads"),
            None,
        )
        .await?;
    Ok(())
}
