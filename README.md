# SoundCloud Rust Client

A Rust client for interacting with the SoundCloud API. This library provides an async interface for searching, retrieving, and downloading content from SoundCloud. It automatically discovers a valid `client_id` from SoundCloud, so you don't need to provide one.

Add the crate to your project:

```bash
cargo add soundcloud-rs
```

**Current version: 0.9.0**

## Quickstart

```rust
use soundcloud_rs::{Client, query::TracksQuery, response::StreamType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for tracks
    let query = TracksQuery { q: Some("electronic".into()), limit: Some(5), ..Default::default() };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("no tracks found").clone();
    let first_track_id = first_track.id.expect("missing track id");

    // Download the track (HLS via ffmpeg, see notes below)
    client
        .download_track(&first_track_id, Some(&StreamType::Hls), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

## More Examples

### Search, get, and download a track
```rust
use soundcloud_rs::{Client, query::TracksQuery, response::StreamType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for tracks
    let query = TracksQuery { q: Some("electronic".to_string()), limit: Some(5), ..Default::default() };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("no tracks found").clone();

    // Get a specific track
    let track_id = first_track.id.expect("missing track id");
    let track = client.get_track(&track_id).await?;

    // Download the track (Progressive example)
    client
        .download_track(&track_id, Some(&StreamType::Progressive), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

### Search, fetch, and download a playlist
```rust
use soundcloud_rs::{Client, query::PlaylistsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for playlists
    let query = PlaylistsQuery { q: Some("brazilian funk".to_string()), limit: Some(3), ..Default::default() };
    let playlists = client.search_playlists(Some(&query)).await?;
    let first_playlist = playlists.collection.first().expect("no playlists found").clone();

    // Get a specific playlist
    let playlist_id = first_playlist.id.expect("missing playlist id");
    let playlist = client.get_playlist(&playlist_id).await?;

    // Download the playlist
    client.download_playlist(&playlist_id, Some("./downloads"), None).await?;

    Ok(())
}
```

### Get user information, followers, tracks, and playlists
```rust
use soundcloud_rs::{Client, query::Paging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Get a specific user
    let user_id = 123456789;
    let user = client.get_user(&user_id).await?;
    println!("User: {}", user.username.unwrap_or_default());

    // Get user's followers
    let followers = client.get_user_followers(&user_id, None::<&Paging>).await?;
    println!("User has {} followers", followers.collection.len());

    // Get user's tracks
    let user_tracks = client.get_user_tracks(&user_id, None::<&Paging>).await?;
    println!("User has {} tracks", user_tracks.collection.len());

    // Get user's playlists
    let user_playlists = client.get_user_playlists(&user_id, None::<&Paging>).await?;
    println!("User has {} playlists", user_playlists.collection.len());

    Ok(())
}
```

## API Overview

### Core
- **`Client::new() -> Result<Self, Box<dyn Error>>`**: Initialize the client by discovering a `client_id`.
- **`get<Q: Serialize, R: DeserializeOwned>(&self, path: &str, query: Option<&Q>) -> Result<R, Box<dyn Error>>`**: Perform a GET against the SoundCloud API.
- **`get_json<R: DeserializeOwned, Q: Serialize>(base_url: &str, path: Option<&str>, query: Option<&Q>, client_id: &str) -> Result<R, Box<dyn Error>>`**: Static helper to GET JSON from any base URL.

### Search
- **`get_search_results(query: Option<&SearchResultsQuery>) -> Result<SearchResultsResponse, Box<dyn Error>>`**
- **`search_all(query: Option<&SearchAllQuery>) -> Result<SearchAllResponse, Box<dyn Error>>`**

### Tracks
- **`search_tracks(query: Option<&TracksQuery>) -> Result<Tracks, Box<dyn Error>>`**
- **`get_track(id: &i64) -> Result<Track, Box<dyn Error>>`**
- **`get_track_related(id: &i64, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`**
- **`download_track(track_id: &i64, stream_type: Option<&StreamType>, destination: Option<&str>, filename: Option<&str>) -> Result<(), Box<dyn Error>>`**
- **`get_stream_url(track_id: &i64, stream_type: Option<&StreamType>) -> Result<String, Box<dyn Error>>`**
- **`get_track_waveform(track_id: &i64) -> Result<Waveform, Box<dyn Error>>`**

### Playlists
- **`search_playlists(query: Option<&PlaylistsQuery>) -> Result<Playlists, Box<dyn Error>>`**
- **`get_playlist(id: &i64) -> Result<Playlist, Box<dyn Error>>`**
- **`get_playlist_reposters(id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`download_playlist(playlist_id: &i64, destination: Option<&str>, playlist_name: Option<&str>) -> Result<(), Box<dyn Error>>`**

### Albums
- **`search_albums(query: Option<&AlbumQuery>) -> Result<Playlists, Box<dyn Error>>`**

### Users
- **`search_users(query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>>`**
- **`get_user(id: &i64) -> Result<User, Box<dyn Error>>`**
- **`get_user_followers(id: &i64, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`get_user_followings(id: &i64, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`get_user_playlists(id: &i64, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>>`**
- **`get_user_tracks(id: &i64, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`**

## Notes on Downloads and FFmpeg
- **HLS downloads** use `ffmpeg-sidecar`. On first HLS download, the crate will automatically download an FFmpeg binary for your platform. No manual installation is required.
- **Progressive downloads** are saved directly without FFmpeg.
- If your environment blocks downloads or requires proxies, the automatic FFmpeg download may fail; in that case, configure your network accordingly before using HLS.

## License

MIT
