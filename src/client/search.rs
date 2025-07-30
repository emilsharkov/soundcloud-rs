use std::error::Error;

use crate::{query::{SearchAllQuery, SearchResultsQuery}, response::{SearchAllResponse, SearchResultsResponse}, Client};

impl Client {
    pub async fn get_search_results(&self, query: Option<&SearchResultsQuery>) -> Result<SearchResultsResponse, Box<dyn Error>> {
        let resp: SearchResultsResponse = self.get("search/queries", query).await?;
        Ok(resp)
    }

    pub async fn search_all(&self, query: Option<&SearchAllQuery>) -> Result<SearchAllResponse, Box<dyn Error>> {
        let resp: SearchAllResponse = self.get("search", query).await?;
        Ok(resp)
    }
}