use crate::client::client::Client;
use crate::models::query::{Paging, UsersQuery};
use crate::models::response::{Playlists, Tracks, User, Users};
use std::error::Error;

impl Client {
    pub async fn search_users(&self, query: Option<&UsersQuery>) -> Result<Users, Box<dyn Error>> {
        let resp: Users = self.get("search/users", query).await?;
        Ok(resp)
    }

    pub async fn get_user(&self, identifier: &i64) -> Result<User, Box<dyn Error>> {
        let url = format!("users/{}", identifier);
        let resp: User = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_user_followers(
        &self,
        identifier: &i64,
        pagination: Option<&Paging>,
    ) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followers", identifier);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followings(
        &self,
        identifier: &i64,
        pagination: Option<&Paging>,
    ) -> Result<Users, Box<dyn Error>> {
        let url = format!("users/{}/followings", identifier);
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_playlists(
        &self,
        identifier: &i64,
        pagination: Option<&Paging>,
    ) -> Result<Playlists, Box<dyn Error>> {
        let url = format!("users/{}/playlists", identifier);
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_tracks(
        &self,
        identifier: &i64,
        pagination: Option<&Paging>,
    ) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("users/{}/tracks", identifier);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }
}
