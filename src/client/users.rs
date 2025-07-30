use std::error::Error;
use crate::client::client::Client;
use crate::models::query::{UsersQuery, Paging};
use crate::models::response::{User, Users, Tracks, Playlists};

impl Client {
    pub async fn search_users(&self, query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>> {
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
}