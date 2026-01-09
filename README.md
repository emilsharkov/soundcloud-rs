# SoundCloud Rust Client

A Rust client for interacting with the SoundCloud API. This library provides an async interface for searching, retrieving, and downloading content from SoundCloud. It automatically discovers a valid `client_id` from SoundCloud, so you don't need to provide one.

Add the crate to your project:

```bash
cargo add soundcloud-rs
```

## Quickstart

```rust
use soundcloud_rs::{Client, Identifier, query::TracksQuery, response::StreamType};

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
        .download_track(&Identifier::Id(first_track_id), Some(&StreamType::Hls), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

### Advanced: Using ClientBuilder for Custom Retry Configuration

```rust
use soundcloud_rs::{ClientBuilder, Identifier, query::TracksQuery};

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
use soundcloud_rs::{Client, Identifier, query::TracksQuery, response::StreamType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for tracks
    let query = TracksQuery { q: Some("electronic".to_string()), limit: Some(5), ..Default::default() };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().expect("no tracks found").clone();

    // Get a specific track
    let track_id = first_track.id.expect("missing track id");
    let track = client.get_track(&Identifier::Id(track_id)).await?;

    // Download the track (Progressive example)
    client
        .download_track(&Identifier::Id(track_id), Some(&StreamType::Progressive), Some("./downloads"), None)
        .await?;

    Ok(())
}
```

### Search, fetch, and download a playlist
```rust
use soundcloud_rs::{Client, Identifier, query::PlaylistsQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Search for playlists
    let query = PlaylistsQuery { q: Some("brazilian funk".to_string()), limit: Some(3), ..Default::default() };
    let playlists = client.search_playlists(Some(&query)).await?;
    let first_playlist = playlists.collection.first().expect("no playlists found").clone();

    // Get a specific playlist
    let playlist_id = first_playlist.id.expect("missing playlist id");
    let playlist = client.get_playlist(&Identifier::Id(playlist_id as i64)).await?;

    // Download the playlist
    client.download_playlist(&Identifier::Id(playlist_id as i64), Some("./downloads"), None).await?;

    Ok(())
}
```

### Get user information, followers, tracks, and playlists
```rust
use soundcloud_rs::{Client, Identifier, query::Paging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;

    // Get a specific user
    let user_id = 123456789;
    let user = client.get_user(&Identifier::Id(user_id)).await?;
    println!("User: {}", user.username.unwrap_or_default());

    // Get user's followers
    let followers = client.get_user_followers(&Identifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} followers", followers.collection.len());

    // Get user's tracks
    let user_tracks = client.get_user_tracks(&Identifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} tracks", user_tracks.collection.len());

    // Get user's playlists
    let user_playlists = client.get_user_playlists(&Identifier::Id(user_id), None::<&Paging>).await?;
    println!("User has {} playlists", user_playlists.collection.len());

    Ok(())
}
```

## Identifier

The library uses an `Identifier` enum to handle different types of SoundCloud resource identifiers:

```rust
use soundcloud_rs::Identifier;

// Use numeric ID
let track_id = Identifier::Id(123456789);

// Use URN (useful for some API endpoints)
let track_urn = Identifier::Urn("soundcloud:tracks:123456789".to_string());
```

This provides better type safety and flexibility when working with SoundCloud resources.

## API Overview

### Core Client Methods

#### Creating a Client
- **`Client::new() -> Result<Self, Error>`**: Initialize the client with default retry configuration by discovering a `client_id`.
- **`Client::with_retry_config(retry_config: RetryConfig) -> Result<Self, Error>`**: Initialize the client with custom retry configuration.

#### ClientBuilder (Recommended for Custom Configuration)
- **`ClientBuilder::new() -> Self`**: Create a new builder with default retry configuration.
- **`with_max_retries(max_retries: u32) -> Self`**: Set the maximum number of retry attempts (default: 1).
- **`with_retry_on_401(retry_on_401: bool) -> Self`**: Enable or disable retrying on 401 Unauthorized responses (default: true).
- **`build() -> Result<Client, Error>`**: Build the client with the configured settings.

#### Client Management
- **`refresh_client_id(&self) -> Result<(), Error>`**: Refresh the client ID by re-discovering it from SoundCloud. Useful if you encounter 401 errors.
- **`get_client_id_value(&self) -> String`**: Get the current client ID value.
- **`health_check(&self) -> bool`**: Health check endpoint that calls `/me` on the API. Returns `true` if the API responds successfully (2xx), `false` otherwise.

#### Low-Level API Methods
- **`get<Q: Serialize, R: DeserializeOwned>(&self, path: &str, query: Option<&Q>) -> Result<R, Error>`**: Perform a GET request against the SoundCloud API.
- **`get_json<R: DeserializeOwned, Q: Serialize>(base_url: &str, path: Option<&str>, query: Option<&Q>, client_id: &str) -> Result<(R, u16), Error>`**: Static helper to GET JSON from any base URL. Returns both the response body and HTTP status code.

### Search
- **`get_search_results(query: Option<&SearchResultsQuery>) -> Result<SearchResultsResponse, Error>`**
- **`search_all(query: Option<&SearchAllQuery>) -> Result<SearchAllResponse, Error>`**

### Tracks
- **`search_tracks(query: Option<&TracksQuery>) -> Result<Tracks, Error>`**
- **`get_track(identifier: &Identifier) -> Result<Track, Error>`**
- **`get_track_related(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Tracks, Error>`**
- **`download_track(identifier: &Identifier, stream_type: Option<&StreamType>, destination: Option<&str>, filename: Option<&str>) -> Result<(), Error>`**
- **`get_stream_url(identifier: &Identifier, stream_type: Option<&StreamType>) -> Result<String, Error>`**
- **`get_track_waveform(identifier: &Identifier) -> Result<Waveform, Error>`**

### Playlists
- **`search_playlists(query: Option<&PlaylistsQuery>) -> Result<Playlists, Error>`**
- **`get_playlist(identifier: &Identifier) -> Result<Playlist, Error>`**
- **`get_playlist_reposters(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Users, Error>`**
- **`download_playlist(identifier: &Identifier, destination: Option<&str>, playlist_name: Option<&str>) -> Result<(), Error>`**

### Albums
- **`search_albums(query: Option<&AlbumQuery>) -> Result<Playlists, Error>`**

### Users
- **`search_users(query: Option<&UsersQuery>) -> Result<Users, Error>`**
- **`get_user(identifier: &Identifier) -> Result<User, Error>`**
- **`get_user_followers(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Users, Error>`**
- **`get_user_followings(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Users, Error>`**
- **`get_user_playlists(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Playlists, Error>`**
- **`get_user_tracks(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Tracks, Error>`**
- **`get_user_reposts(identifier: &Identifier, pagination: Option<&Paging>) -> Result<Reposts, Error>`**

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

## Error Handling

The library uses a custom `Error` type that implements `std::error::Error + Send + Sync` for async compatibility. All API methods return `Result<T, Error>`.

```rust
use soundcloud_rs::{Client, Error};

match client.search_tracks(None).await {
    Ok(tracks) => println!("Found {} tracks", tracks.collection.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```


## License

MIT
