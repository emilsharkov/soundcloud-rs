use std::error::Error;

use tokio;
use soundcloud_rs::{query::{SearchResultsQuery, SearchAllQuery, AlbumQuery}, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await?;
    let search_query = AlbumQuery {
        q: Some("lil uzi vert".to_string()),
        limit: Some(10),
        offset: Some(0),
        linked_partitioning: Some(true),
    };
    let search_response = client.search_albums(Some(&search_query)).await?;
    println!("{:#?}", search_response);
    Ok(())
} 