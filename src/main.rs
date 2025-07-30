use std::error::Error;

use tokio;
use soundcloud_rs::{query::{SearchResultsQuery, SearchAllQuery, AlbumQuery}, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new().await?;
    Ok(())
} 