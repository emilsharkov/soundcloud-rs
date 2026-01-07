use std::error::Error;

use soundcloud_rs::{ClientBuilder, SoundcloudIdentifier, query::TracksQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ClientBuilder::new()
        .with_retry_on_401(true)
        .with_max_retries(3)
        .build()
        .await?;

    let query = TracksQuery {
        q: Some("no era amor".to_string()),
        limit: Some(5),
        ..Default::default()
    };
    
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("No tracks found");
    let first_track_id = first_track.id.expect("No track id found");
    client
        .download_track(
            &SoundcloudIdentifier::Id(first_track_id),
            None,
            Some("./downloads"),
            None,
        )
        .await?;
    Ok(())
}
