use serde::{Deserialize, Serialize};

use crate::models::response::{users::UserSummary, PagingCollection};

pub type Tracks = PagingCollection<Track>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Track {
    pub access: Option<String>,
    pub artwork_url: Option<String>,
    pub bpm: Option<f64>,
    pub comment_count: Option<u64>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub download_url: Option<String>,
    pub downloadable: Option<bool>,
    pub duration: Option<u64>,
    pub embeddable_by: Option<String>,
    pub favoritings_count: Option<u64>,
    pub genre: Option<String>,
    pub id: Option<u64>,
    pub isrc: Option<String>,
    pub kind: Option<String>,
    pub label_name: Option<String>,
    pub license: Option<String>,
    pub media: Option<Media>,
    pub permalink_url: Option<String>,
    pub playback_count: Option<u64>,
    pub publisher_metadata: Option<PublisherMetadata>,
    pub purchase_title: Option<String>,
    pub purchase_url: Option<String>,
    pub release: Option<String>,
    pub release_day: Option<u32>,
    pub release_month: Option<u32>,
    pub release_year: Option<u32>,
    pub reposts_count: Option<u64>,
    pub sharing: Option<String>,
    pub stream_url: Option<String>,
    pub streamable: Option<bool>,
    pub tag_list: Option<String>,
    pub title: Option<String>,
    pub urn: Option<String>,
    pub user: Option<UserSummary>,
    pub user_favorite: Option<bool>,
    pub user_playback_count: Option<u64>,
    pub waveform_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PublisherMetadata {
    pub id: Option<u64>,
    pub urn: Option<String>,
    pub contains_music: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Media {
    pub transcodings: Option<Vec<Transcoding>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Transcoding {
    pub url: Option<String>,
    pub preset: Option<String>,
    pub duration: Option<u64>,
    pub snipped: Option<bool>,
    pub format: Option<TranscodingFormat>,
    pub quality: Option<String>,
    pub is_legacy_transcoding: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TranscodingFormat {
    pub protocol: Option<StreamType>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    Hls,
    Progressive,
    #[serde(other)]
    None,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Stream {
    pub url: Option<String>,
}