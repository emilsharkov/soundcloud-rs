use crate::{
    Client,
    models::error::Error,
    query::{SearchAllQuery, SearchResultsQuery},
    response::{SearchAllResponse, SearchResultsResponse},
};

impl Client {
    pub async fn get_search_results(
        &self,
        query: Option<&SearchResultsQuery>,
    ) -> Result<SearchResultsResponse, Error> {
        let resp: SearchResultsResponse = self.get("search/queries", query).await?;
        Ok(resp)
    }

    pub async fn search_all(
        &self,
        query: Option<&SearchAllQuery>,
    ) -> Result<SearchAllResponse, Error> {
        let resp: SearchAllResponse = self.get("search", query).await?;
        Ok(resp)
    }
}
