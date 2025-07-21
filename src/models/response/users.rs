// src/response/users.rs
use serde::Deserialize;

use crate::models::response::PagingCollection;

pub type Users = PagingCollection<User>;

#[derive(Debug, Deserialize)]
pub struct User {
    pub urn: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: Option<u32>,
    pub followings_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UserSummary {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub permalink_url: Option<String>,
    pub avatar_url: Option<String>,
}