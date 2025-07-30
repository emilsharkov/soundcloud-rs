# SoundCloud Rust Client

A Rust client for interacting with the SoundCloud API. This library provides a comprehensive interface for searching, retrieving, and downloading content from SoundCloud.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
soundcloud = "0.1.0"
```

## Examples

### Search, get, and download a track
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;
    
    // Search for tracks
    let query = TracksQuery {
        q: Some("electronic".to_string()),
        limit: Some(5),
        ..Default::default()
    };
    let tracks = client.search_tracks(Some(&query)).await?;
    let first_track = tracks.collection.first().unwrap();
    
    // Get a specific track
    let track_id = first_track.id.as_ref().unwrap();
    let track = client.get_track_by_id(track_id.to_string().as_str()).await?;

    // Download the track
    client.download_track(&track, Some(StreamType::Hls), Some("./downloads"), None).await?;
    
    Ok(())
}
```

### Search, fetch, and download a playlist
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;
    
    // Search for playlists
    let query = PlaylistsQuery {
        q: Some("brazilian funk".to_string()),
        limit: Some(3),
        ..Default::default()
    };
    let playlists = client.search_playlists(Some(&query)).await?;
    let first_playlist = playlists.collection.first().unwrap();
    
    // Get a specific playlist
    let playlist_id = first_playlist.id.as_ref().unwrap();
    let playlist = client.get_playlist_by_id(playlist_id.to_string().as_str()).await?;

    // Download the playlist
    client.download_playlist(&playlist, Some("./downloads"), None).await?;
    
    Ok(())
}
```

### Get user information, followers, tracks, and playlists
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;
    
    // Get a specific user
    let user_id = "123456789";
    let user = client.get_user_by_id(user_id).await?;
    println!("User: {}", user.username);
    
    // Get user's followers
    let followers = client.get_user_followers_by_id(user_id, None).await?;
    println!("User has {} followers", followers.collection.len());
    
    // Get user's tracks
    let user_tracks = client.get_user_tracks_by_id(user_id, None).await?;
    println!("User has {} tracks", user_tracks.collection.len());
    
    // Get user's playlists
    let user_playlists = client.get_user_playlists_by_id(user_id, None).await?;
    println!("User has {} playlists", user_playlists.collection.len());
    
    Ok(())
}
```

## Available Methods

### Core Client Methods

#### `new() -> Result<Self, Box<dyn Error>>`
Creates a new SoundCloud client instance. Automatically fetches the required client ID from SoundCloud's website.

#### `get<Q: Serialize, R: DeserializeOwned>(&self, path: &str, query: Option<&Q>) -> Result<R, Box<dyn Error>>`
Generic method for making GET requests to the SoundCloud API.

#### `get_json<R: DeserializeOwned, Q: Serialize>(base_url: &str, path: Option<&str>, query: Option<&Q>, client_id: &str) -> Result<R, Box<dyn Error>>`
Static method for making JSON GET requests with custom base URL and client ID.

### Search Methods

#### `get_search_results(query: Option<&SearchResultsQuery>) -> Result<SearchResultsResponse, Box<dyn Error>>`
Get search results using the search queries endpoint.

#### `search_all(query: Option<&SearchAllQuery>) -> Result<SearchAllResponse, Box<dyn Error>>`
Perform a comprehensive search across all content types.

### Track Methods

#### `search_tracks(query: Option<&TracksQuery>) -> Result<Tracks, Box<dyn Error>>`
Search for tracks using optional query parameters.

#### `get_track_by_id(id: &str) -> Result<Track, Box<dyn Error>>`
Retrieve a specific track by its ID.

#### `get_track_by_urn(urn: &str) -> Result<Track, Box<dyn Error>>`
Retrieve a specific track by its URN.

#### `get_track_related_by_id(id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`
Get related tracks for a track by ID with optional pagination.

#### `get_track_related_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`
Get related tracks for a track by URN with optional pagination.

#### `download_track(track: &Track, stream_type: Option<StreamType>, destination: Option<&str>, filename: Option<&str>) -> Result<(), Box<dyn Error>>`
Download a track to a local file. Supports both HLS and Progressive stream types.

### Playlist Methods

#### `search_playlists(query: Option<&PlaylistsQuery>) -> Result<Playlists, Box<dyn Error>>`
Search for playlists using optional query parameters.

#### `get_playlist_by_id(id: &str) -> Result<Playlist, Box<dyn Error>>`
Retrieve a specific playlist by its ID.

#### `get_playlist_by_urn(urn: &str) -> Result<Playlist, Box<dyn Error>>`
Retrieve a specific playlist by its URN.

#### `get_playlist_reposters_by_id(id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get users who reposted a playlist by ID with optional pagination.

#### `get_playlist_reposters_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get users who reposted a playlist by URN with optional pagination.

#### `download_playlist(playlist: &Playlist, destination: Option<&str>, playlist_name: Option<&str>) -> Result<(), Box<dyn Error>>`
Download all tracks in a playlist to a local directory.

### Album Methods

#### `search_albums(query: Option<&AlbumQuery>) -> Result<Playlists, Box<dyn Error>>`
Search for albums using optional query parameters.

### User Methods

#### `search_users(query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>>`
Search for users using optional query parameters.

#### `get_user_by_id(id: &str) -> Result<User, Box<dyn Error>>`
Retrieve a specific user by their ID.

#### `get_user_by_urn(urn: &str) -> Result<User, Box<dyn Error>>`
Retrieve a specific user by their URN.

#### `get_user_followers_by_id(id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get followers of a user by ID with optional pagination.

#### `get_user_followers_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get followers of a user by URN with optional pagination.

#### `get_user_followings_by_id(id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get users that a user follows by ID with optional pagination.

#### `get_user_followings_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>>`
Get users that a user follows by URN with optional pagination.

#### `get_user_playlists_by_id(id: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>>`
Get playlists created by a user by ID with optional pagination.

#### `get_user_playlists_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>>`
Get playlists created by a user by URN with optional pagination.

#### `get_user_tracks_by_id(id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`
Get tracks created by a user by ID with optional pagination.

#### `get_user_tracks_by_urn(urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>>`
Get tracks created by a user by URN with optional pagination.

## Dependencies

This library depends on:

- **`reqwest`** - HTTP client for making API requests to SoundCloud's servers. Used for fetching client IDs, track data, and downloading audio files.
- **`serde`** - Serialization/deserialization framework for converting JSON responses from the SoundCloud API into Rust structs. Enables type-safe data handling.
- **`serde_json`** - JSON parsing library that works with serde to handle the JSON format used by SoundCloud's API responses.
- **`tokio`** - Async runtime that provides the foundation for non-blocking I/O operations. Enables concurrent API requests and file downloads.
- **`tokio-util`** - Additional utilities for tokio, providing enhanced async I/O capabilities for file operations and stream handling.
- **`futures-util`** - Utilities for working with futures and async streams, supporting the async/await patterns used throughout the library.
- **`ffmpeg-sidecar`** - Audio processing library that handles HLS (HTTP Live Streaming) format downloads. Automatically downloads and manages FFmpeg binaries for audio conversion.
- **`regex`** - Regular expression library used for extracting client IDs from SoundCloud's JavaScript files. Essential for the authentication process.

## License

This project is licensed under the MIT License.
