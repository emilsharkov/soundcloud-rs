use std::error::Error;
use crate::client::client::Client;
use crate::models::query::users::UsersQuery;
use crate::models::query::tracks::Paging;
use crate::models::response::users::{User, Users, WebProfiles};
use crate::models::response::tracks::Tracks;
use crate::models::response::playlists::Playlists;

impl Client {
    pub async fn get_users(&self, query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>> {
        let resp: Users = self.get("search/users", query).await?;
        Ok(resp)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<User, Box<dyn Error>> {
        let url = format!("users/{}", id);
        let resp: User = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_user_by_urn(&self, urn: &str) -> Result<User, Box<dyn Error>> {
        let url = format!("users/{}", urn);
        let resp: User = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_user_favorites_tracks_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/favorites", id);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_favorites_tracks_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/favorites", urn);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followers_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followers", id);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followers_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followers", urn);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followings_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followings", id);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followings_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followings", urn);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_following_detail_by_id(&self, user_id: &str, following_id: &str) -> Result<User, Box<dyn Error>> {
        let url = format!("users/{}/followings/{}", user_id, following_id);
        let u = self.get(&url, None::<&()>).await?;
        Ok(u)
    }

    pub async fn get_user_following_detail_by_urn(&self, user_urn: &str, following_urn: &str) -> Result<User, Box<dyn Error>> {
        let url = format!("users/{}/followings/{}", user_urn, following_urn);
        let u = self.get(&url, None::<&()>).await?;
        Ok(u)
    }

    pub async fn get_user_playlists_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>> {
        let url = format!("users/{}/playlists", id);
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_playlists_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>> {
        let url = format!("users/{}/playlists", urn);
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_tracks_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/tracks", id);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_tracks_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/tracks", urn);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_web_profiles_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<WebProfiles, Box<dyn Error>> {
        let url = format!("users/{}/web-profiles", id);
        let resp: WebProfiles = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_web_profiles_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<WebProfiles, Box<dyn Error>> {
        let url = format!("users/{}/web-profiles", urn);
        let resp: WebProfiles = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_likes_tracks_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/likes/tracks", id);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_likes_tracks_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/likes/tracks", urn);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_likes_playlists_by_id(&self, id: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>> {
        let url = format!("users/{}/likes/playlists", id);
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_likes_playlists_by_urn(&self, urn: &str, pagination: Option<&Paging>) -> Result<Playlists, Box<dyn Error>> {
        let url = format!("users/{}/likes/playlists", urn);
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }
}