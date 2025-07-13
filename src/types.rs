use serde::{Serialize, Deserialize};

// OAuthToken and related enums
#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthToken {
    pub grant_type: GrantType,
    pub client_id: String,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
    ClientCredentials,
}

// TrackMetadataRequest and nested TrackMetadata
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMetadataRequest {
    pub track: TrackMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(default, rename = "sharing")]
    pub sharing: Sharing,
    #[serde(default, rename = "embeddable_by")]
    pub embeddable_by: EmbeddableBy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub streamable: bool,
    #[serde(default)]
    pub downloadable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(default)]
    pub commentable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Sharing {
    Public,
    Private,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddableBy {
    All,
    Me,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum License {
    NoRightsReserved,
    AllRightsReserved,
    CcBy,
    CcByNc,
    CcByNd,
    CcBySa,
    CcByNcNd,
    CcByNcSa,
}

// CreateUpdatePlaylistRequest and nested types
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUpdatePlaylistRequest {
    pub playlist: PlaylistRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub sharing: Sharing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<PlaylistTrack>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artwork_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_type: Option<SetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Album,
    Playlist,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistTrack {
    pub urn: String,
}

// CreateUpdatePlaylistFormRequest: form-style keys
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUpdatePlaylistFormRequest {
    #[serde(rename = "playlist[title]")]
    pub title: Option<String>,
    #[serde(rename = "playlist[sharing]")]
    pub sharing: Option<Sharing>,
    #[serde(rename = "playlist[description]")]
    pub description: Option<String>,
    #[serde(rename = "playlist[tracks][][urn]")]
    pub tracks: Option<String>,
    #[serde(rename = "playlist[artwork_data]")]
    pub artwork_data: Option<String>,
    #[serde(rename = "playlist[ean]")]
    pub ean: Option<String>,
    #[serde(rename = "playlist[genre]")]
    pub genre: Option<String>,
    #[serde(rename = "playlist[label_name]")]
    pub label_name: Option<String>,
    #[serde(rename = "playlist[license]")]
    pub license: Option<String>,
    #[serde(rename = "playlist[permalink]")]
    pub permalink: Option<String>,
    #[serde(rename = "playlist[permalink_url]")]
    pub permalink_url: Option<String>,
    #[serde(rename = "playlist[purchase_title]")]
    pub purchase_title: Option<String>,
    #[serde(rename = "playlist[purchase_url]")]
    pub purchase_url: Option<String>,
    #[serde(rename = "playlist[release]")]
    pub release: Option<String>,
    #[serde(rename = "playlist[release_date]")]
    pub release_date: Option<String>,
    #[serde(rename = "playlist[set_type]")]
    pub set_type: Option<SetType>,
    #[serde(rename = "playlist[tag_list]")]
    pub tag_list: Option<String>,
}

// TrackDataRequest and nested TrackFormData
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackDataRequest {
    #[serde(rename = "track[title]")]
    pub title: Option<String>,
    #[serde(rename = "track[asset_data]")]
    pub asset_data: Option<String>,
    #[serde(rename = "track[permalink]")]
    pub permalink: Option<String>,
    #[serde(rename = "track[sharing]")]
    pub sharing: Option<Sharing>,
    #[serde(rename = "track[embeddable_by]")]
    pub embeddable_by: Option<EmbeddableBy>,
    #[serde(rename = "track[purchase_url]")]
    pub purchase_url: Option<String>,
    #[serde(rename = "track[description]")]
    pub description: Option<String>,
    #[serde(rename = "track[genre]")]
    pub genre: Option<String>,
    #[serde(rename = "track[tag_list]")]
    pub tag_list: Option<String>,
    #[serde(rename = "track[label_name]")]
    pub label_name: Option<String>,
    #[serde(rename = "track[release]")]
    pub release: Option<String>,
    #[serde(rename = "track[release_date]")]
    pub release_date: Option<String>,
    #[serde(rename = "track[streamable]")]
    pub streamable: Option<bool>,
    #[serde(rename = "track[downloadable]")]
    pub downloadable: Option<bool>,
    #[serde(rename = "track[license]")]
    pub license: Option<License>,
    #[serde(rename = "track[commentable]")]
    pub commentable: Option<bool>,
    #[serde(rename = "track[isrc]")]
    pub isrc: Option<String>,
    #[serde(rename = "track[artwork_data]")]
    pub artwork_data: Option<String>,
}

// Found
#[derive(Serialize, Deserialize, Debug)]
pub struct Found {
    pub status: String,
    pub location: String,
}

// Error
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: Option<i32>,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, // deprecated
}

// TooManyRequests
#[derive(Serialize, Deserialize, Debug)]
pub struct TooManyRequests {
    #[serde(flatten)]
    pub error: Error,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_warning_urn: Option<String>,
}

// User
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discogs_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followings_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myspace_name: Option<String>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_favorites_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reposts_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SubscriptionItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionItem {
    pub product: Option<SubscriptionProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// Me
#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_count: Option<i32>, // deprecated always 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discogs_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followings_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myspace_name: Option<String>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email_confirmed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_playlists_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_tracks_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_favorites_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<Quota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reposts_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SubscriptionItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_seconds_left: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_upload_quota: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_seconds_used: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_seconds_left: Option<i32>,
}

// Users
#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub collection: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
}

// Track
#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artwork_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpm: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable_by: Option<String>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favoritings_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_favorite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_playback_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_country_codes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reposts_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_uri: Option<String>,
}

// Tracks and alias for TracksList
#[derive(Serialize, Deserialize, Debug)]
pub struct Tracks {
    pub collection: Vec<Track>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
}
pub type TracksList = Vec<Track>; // deprecated

// Playlist
#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>, //deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artwork_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddable_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<Track>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

// Playlists
#[derive(Serialize, Deserialize, Debug)]
pub struct Playlists {
    pub collection: Vec<Playlist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
}
pub type PlaylistsArray = Vec<Playlist>; // deprecated

// Activity items and collections
#[derive(Serialize, Deserialize, Debug)]
pub struct Activities {
    pub collection: Vec<ActivityItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityItem {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub origin: ActivityOrigin,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActivityOrigin {
    Track(Track),
    Playlist(Playlist),
}

// WebProfiles
#[derive(Serialize, Deserialize, Debug)]
pub struct WebProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
pub type WebProfiles = Vec<WebProfile>;

// Comment and nested user
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub urn: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<CommentUser>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followings_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reposts_count: Option<i32>,
}

// Comments
#[derive(Serialize, Deserialize, Debug)]
pub struct Comments {
    pub collection: Vec<Comment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_href: Option<String>,
}

// Streams
#[derive(Serialize, Deserialize, Debug)]
pub struct Streams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_mp3_128_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_mp3_128_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_opus_64_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_mp3_128_url: Option<String>,
}
