# SoundCloud Rust Client

A Rust client for interacting with the SoundCloud API. This library provides an async interface for searching, retrieving, and downloading content from SoundCloud. It automatically discovers a valid `client_id` from SoundCloud, so you don't need to provide one.

Add the crate to your project:

```bash
cargo add soundcloud-rs
```

## Quickstart

```rust
use soundcloud_rs::{Client, SoundcloudIdentifier, query::TracksQuery, response::StreamType};

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
        .download_track(&SoundcloudIdentifier::Id(first_track_id), Some(&StreamType::Hls), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

### Advanced: Using ClientBuilder for Custom Retry Configuration

```rust
use soundcloud_rs::{ClientBuilder, SoundcloudIdentifier, query::TracksQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with custom retry configuration
    let client = ClientBuilder::new()
        .with_max_retries(3)           // Retry up to 3 times on 401 errors
        .with_retry_on_401(true)        // Enable automatic retry on 401 Unauthorized
        .build()
        .await?;

    // Use the client as normal
    let query = TracksQuery { q: Some("electronic".into()), limit: Some(5), ..Default::default() };
    let tracks = client.search_tracks(Some(&query)).await?;
    
    Ok(())
}
```

## More Examples

### Search, get, and download a track
```rust
use soundcloud_rs::{Client, SoundcloudIdentifier, query::TracksQuery, response::StreamType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for tracks
    let query = TracksQuery { q: Some("electronic".to_string()), limit: Some(5), ..Default::default() };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("no tracks found").clone();

    // Get a specific track
    let track_id = first_track.id.expect("missing track id");
    let track = client.get_track(&SoundcloudIdentifier::Id(track_id)).await?;

    // Download the track (Progressive example)
    client
        .download_track(&SoundcloudIdentifier::Id(track_id), Some(&StreamType::Progressive), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

### Search, fetch, and download a playlist
```rust
use soundcloud_rs::{Client, SoundcloudIdentifier, query::PlaylistsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for playlists
    let query = PlaylistsQuery { q: Some("brazilian funk".to_string()), limit: Some(3), ..Default::default() };
    let playlists = client.search_playlists(Some(&query)).await?;
    let first_playlist = playlists.collection.first().expect("no playlists found").clone();

    // Get a specific playlist
    let playlist_id = first_playlist.id.expect("missing playlist id");
    let playlist = client.get_playlist(&SoundcloudIdentifier::Id(playlist_id)).await?;

    // Download the playlist
    client.download_playlist(&SoundcloudIdentifier::Id(playlist_id), Some("./downloads"), None).await?;

    Ok(())
}
```

### Get user information, followers, tracks, and playlists
```rust
use soundcloud_rs::{Client, SoundcloudIdentifier, query::Paging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Get a specific user
    let user_id = 123456789;
    let user = client.get_user(&SoundcloudIdentifier::Id(user_id)).await?;
    println!("User: {}", user.username.unwrap_or_default());

    // Get user's followers
    let followers = client.get_user_followers(&SoundcloudIdentifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} followers", followers.collection.len());

    // Get user's tracks
    let user_tracks = client.get_user_tracks(&SoundcloudIdentifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} tracks", user_tracks.collection.len());

    // Get user's playlists
    let user_playlists = client.get_user_playlists(&SoundcloudIdentifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} playlists", user_playlists.collection.len());

    Ok(())
}
```

## SoundcloudIdentifier

The library now uses a `SoundcloudIdentifier` enum to handle different types of SoundCloud resource identifiers:

```rust
use soundcloud_rs::SoundcloudIdentifier;

// Use numeric ID
let track_id = SoundcloudIdentifier::Id(123456789);

// Use URN (useful for some API endpoints)
let track_urn = SoundcloudIdentifier::Urn("soundcloud:tracks:123456789".to_string());
```

This provides better type safety and flexibility when working with SoundCloud resources.

## API Overview

### Core Client Methods

#### Creating a Client
- **`Client::new() -> Result<Self, Box<dyn Error>>`**: Initialize the client with default retry configuration by discovering a `client_id`.
- **`Client::with_retry_config(retry_config: RetryConfig) -> Result<Self, Box<dyn Error>>`**: Initialize the client with custom retry configuration.

#### ClientBuilder (Recommended for Custom Configuration)
- **`ClientBuilder::new() -> Self`**: Create a new builder with default retry configuration.
- **`with_max_retries(max_retries: u32) -> Self`**: Set the maximum number of retry attempts (default: 1).
- **`with_retry_on_401(retry_on_401: bool) -> Self`**: Enable or disable retrying on 401 Unauthorized responses (default: true).
- **`build() -> Result<Client, Box<dyn Error>>`**: Build the client with the configured settings.

#### Client Management
- **`refresh_client_id(&self) -> Result<(), Box<dyn Error>>`**: Refresh the client ID by re-discovering it from SoundCloud. Useful if you encounter 401 errors.
- **`get_client_id_value(&self) -> String`**: Get the current client ID value.

#### Low-Level API Methods
- **`get<Q: Serialize, R: DeserializeOwned>(&self, path: &str, query: Option<&Q>) -> Result<R, Box<dyn Error>>`**: Perform a GET request against the SoundCloud API.
- **`get_json<R: DeserializeOwned, Q: Serialize>(base_url: &str, path: Option<&str>, query: Option<&Q>, client_id: &str) -> Result<(R, u16), Box<dyn Error>>`**: Static helper to GET JSON from any base URL. Returns both the response body and HTTP status code.

### Search
- **`get_search_results(query: Option<&SearchResultsQuery>) -> Result<SearchResultsResponse, Box<dyn Error>>`**
- **`search_all(query: Option<&SearchAllQuery>) -> Result<SearchAllResponse, Box<dyn Error>>`**

### Tracks
- **`search_tracks(query: Option<&TracksQuery>) -> Result<Tracks, Box<dyn Error>>`**
- **`get_track(identifier: &SoundcloudIdentifier) -> Result<Track, Box<dyn Error>>`**
- **`get_track_related(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`**
- **`download_track(identifier: &SoundcloudIdentifier, stream_type: Option<&StreamType>, destination: Option<&str>, filename: Option<&str>) -> Result<(), Box<dyn Error>>`**
- **`get_stream_url(identifier: &SoundcloudIdentifier, stream_type: Option<&StreamType>) -> Result<String, Box<dyn Error>>`**
- **`get_track_waveform(identifier: &SoundcloudIdentifier) -> Result<Waveform, Box<dyn Error>>`**

### Playlists
- **`search_playlists(query: Option<&PlaylistsQuery>) -> Result<Playlists, Box<dyn Error>>`**
- **`get_playlist(identifier: &SoundcloudIdentifier) -> Result<Playlist, Box<dyn Error>>`**
- **`get_playlist_reposters(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`download_playlist(identifier: &SoundcloudIdentifier, destination: Option<&str>, playlist_name: Option<&str>) -> Result<(), Box<dyn Error>>`**

### Albums
- **`search_albums(query: Option<&AlbumQuery>) -> Result<Playlists, Box<dyn Error>>`**

### Users
- **`search_users(query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>>`**
- **`get_user(identifier: &SoundcloudIdentifier) -> Result<User, Box<dyn Error>>`**
- **`get_user_followers(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`get_user_followings(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`**
- **`get_user_playlists(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>>`**
- **`get_user_tracks(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`**
- **`get_user_reposts(identifier: &SoundcloudIdentifier, pagination: Option<&Paging>) -> Result<Reposts, Box<dyn Error>>`**

## Retry Configuration

The client supports automatic retry on 401 Unauthorized errors, which can occur when SoundCloud rotates their client IDs. By default, the client will retry once with a refreshed client ID. You can customize this behavior:

```rust
use soundcloud_rs::ClientBuilder;

let client = ClientBuilder::new()
    .with_max_retries(3)        // Retry up to 3 times
    .with_retry_on_401(true)     // Enable retry on 401 (default: true)
    .build()
    .await?;
```

**RetryConfig defaults:**
- `max_retries`: 1
- `retry_on_401`: true

When a 401 error occurs, the client will automatically refresh the client ID and retry the request up to `max_retries` times.

## Notes on Downloads and FFmpeg
- **HLS downloads** use `ffmpeg-sidecar`. On first HLS download, the crate will automatically download an FFmpeg binary for your platform. No manual installation is required.
- **Progressive downloads** are saved directly without FFmpeg.
- If your environment blocks downloads or requires proxies, the automatic FFmpeg download may fail; in that case, configure your network accordingly before using HLS.

## Changelog

### v0.11.0
- **BREAKING**: All API methods now use `SoundcloudIdentifier` instead of raw `i64` or `&str` for resource identifiers
- Added `SoundcloudIdentifier` enum supporting both numeric IDs and URNs
- Improved type safety across all API methods
- Updated all examples and documentation to reflect the new identifier system

## License

MIT
