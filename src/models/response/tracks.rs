use serde::Deserialize;

use crate::models::response::{users::UserSummary, PagingCollection};

pub type Tracks = PagingCollection<Track>;

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub urn: Option<String>,
    pub kind: Option<String>,
    pub duration: Option<u64>,
    pub permalink_url: Option<String>,
    pub stream_url: Option<String>,
    pub genre: Option<String>,
    pub tag_list: Option<String>,
    pub description: Option<String>,
    pub playback_count: Option<u64>,
    pub user: Option<UserSummary>,
    pub access: Option<String>,
    pub bpm: Option<f64>,
    pub comment_count: Option<u64>,
    pub downloadable: Option<bool>,
    pub download_url: Option<String>,
    pub streamable: Option<bool>,
    pub sharing: Option<String>,
    pub user_favorite: Option<bool>,
    pub user_playback_count: Option<u64>,
    pub waveform_url: Option<String>,
    pub embeddable_by: Option<String>,
    pub favoritings_count: Option<u64>,
    pub reposts_count: Option<u64>,
    pub created_at: Option<String>,
    pub license: Option<String>,
    pub label_name: Option<String>,
    pub isrc: Option<String>,
    pub release: Option<String>,
    pub release_day: Option<u32>,
    pub release_month: Option<u32>,
    pub release_year: Option<u32>,
    pub artwork_url: Option<String>,
    pub purchase_url: Option<String>,
    pub purchase_title: Option<String>,
}
