use crate::models::client::Client;
use crate::models::client::Identifier;
use crate::models::error::Error;
use crate::models::query::{Paging, UsersQuery};
use crate::models::response::{Playlists, Reposts, Tracks, User, Users};

impl Client {
    pub async fn search_users(&self, query: Option<&UsersQuery>) -> Result<Users, Error> {
        let resp: Users = self.get("search/users", query).await?;
        Ok(resp)
    }

    pub async fn get_user(
        &self,
        identifier: &Identifier,
    ) -> Result<User, Error> {
        let url = format!("users/{identifier}");
        let resp: User = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_user_followers(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Users, Error> {
        let url = format!("users/{identifier}/followers");
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_followings(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Users, Error> {
        let url = format!("users/{identifier}/followings");
        let resp: Users = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_playlists(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Playlists, Error> {
        let url = format!("users/{identifier}/playlists");
        let resp: Playlists = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_tracks(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Tracks, Error> {
        let url = format!("users/{identifier}/tracks");
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_user_reposts(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Reposts, Error> {
        let id = match identifier {
            Identifier::Id(id) => id.to_string(),
            Identifier::Urn(urn) => urn
                .split(':')
                .last()
                .expect("Could not extract ID from URN")
                .to_owned(),
        };
        let url = format!("stream/users/{}/reposts", id);
        let resp: Reposts = self.get(&url, pagination).await?;
        Ok(resp)
    }
}
