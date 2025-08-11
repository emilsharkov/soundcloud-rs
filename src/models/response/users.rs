use serde::{Deserialize, Serialize};

use crate::models::response::PagingCollection;

pub type Users = PagingCollection<User>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub avatar_url: Option<String>,
    pub city: Option<String>,
    pub comments_count: Option<u32>,
    pub country_code: Option<String>,
    pub created_at: Option<String>,
    pub creator_subscriptions: Option<Vec<CreatorSubscriptionWrapper>>,
    pub creator_subscription: Option<CreatorSubscriptionWrapper>,
    pub description: Option<String>,
    pub followers_count: Option<u32>,
    pub followings_count: Option<u32>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub groups_count: Option<u32>,
    pub id: Option<u64>,
    pub kind: Option<String>,
    pub last_modified: Option<String>,
    pub last_name: Option<String>,
    pub likes_count: Option<u32>,
    pub playlist_likes_count: Option<u32>,
    pub permalink: Option<String>,
    pub permalink_url: Option<String>,
    pub playlist_count: Option<u32>,
    pub reposts_count: Option<u32>,
    pub track_count: Option<u32>,
    pub uri: Option<String>,
    pub urn: Option<String>,
    pub username: Option<String>,
    pub verified: Option<bool>,
    pub visuals: Option<Visuals>,
    pub badges: Option<Badges>,
    pub station_urn: Option<String>,
    pub station_permalink: Option<String>,
    pub date_of_birth: Option<DateOfBirth>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreatorSubscriptionWrapper {
    pub product: Product,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Product {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Visuals {
    pub urn: Option<String>,
    pub enabled: Option<bool>,
    pub visuals: Option<Vec<VisualEntry>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VisualEntry {
    pub urn: Option<String>,
    pub entry_time: Option<u32>,
    pub visual_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Badges {
    pub pro: Option<bool>,
    pub creator_mid_tier: Option<bool>,
    pub pro_unlimited: Option<bool>,
    pub verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DateOfBirth {
    pub month: Option<u8>,
    pub year: Option<u16>,
    pub day: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserSummary {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub permalink_url: Option<String>,
    pub avatar_url: Option<String>,
}