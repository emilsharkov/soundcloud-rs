// src/response/users.rs
use serde::Deserialize;

use crate::models::response::PagingCollection;

pub type Users = PagingCollection<User>;
pub type WebProfiles = PagingCollection<WebProfile>;

#[derive(Debug, Deserialize)]
pub struct User {
    pub urn: String,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: Option<u32>,
    pub followings_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct WebProfile {
    pub created_at: String,
    pub urn: String,
    pub kind: String,
    pub service: String,
    pub title: String,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSummary {
    pub id: u64,
    pub username: String,
    pub permalink_url: String,
    pub avatar_url: Option<String>,
}