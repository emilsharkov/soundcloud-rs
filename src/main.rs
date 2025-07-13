mod lib;

use std::error::Error;
use reqwest;
use regex::Regex;
use lib::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let re = Regex::new(r#"https?://[^\s"]+\.js"#).unwrap();
    let urls: Vec<&str> = re.find_iter(&text).map(|mat| mat.as_str()).collect();
    println!("{:?}", urls);
    let mut client_id = String::new();
    for url in urls {
        if let Ok(id) = get_client_id(url).await {
            if !id.is_empty() {
                println!("{}", id);
                client_id = id;
                break;
            }
        }
    }
    println!("{}", client_id);
    Ok(())
}

async fn get_client_id(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let re = Regex::new(r#"client_id[:=]"?(\w{32})"#).unwrap();
    let mut client_id = String::new();
    println!("{}", url);
    for cap in re.captures_iter(&text) {
        client_id = cap[1].to_string();
    }
    println!("{}", client_id);
    Ok(client_id)
}