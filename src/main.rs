use std::error::Error;

use tokio;
use soundcloud::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await.unwrap();

    // match client.get_tracks(Some(&TracksQuery {
    //     q: Some("fluxxwave".to_string()),
    //     ids: None,
    //     urns: None,
    //     genres: None,
    //     tags: None,
    //     bpm: None,
    //     duration: None,
    //     created_at: None,
    //     access: None,
    //     limit: None,
    //     offset: None,
    //     linked_partitioning: None,
    // })).await {
    //     Ok(tracks) => println!("Tracks: {:?}", tracks.collection.get(0).unwrap()),
    //     Err(e) => eprintln!("Error fetching tracks: {}", e),
    // }

    match client.get_track_by_id("1508792845").await {
        Ok(track) => println!("Track: {:?}", track),
        Err(e) => eprintln!("Error fetching track: {}", e),
    }

    Ok(())
} 