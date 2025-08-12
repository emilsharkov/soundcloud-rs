use regex::Regex;
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;

use crate::constants::{SOUNDCLOUD_API_URL, SOUNDCLOUD_URL};

#[derive(Debug)]
pub struct Client {
    pub client_id: String,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let script_urls = Self::get_script_urls().await?;
        for url in script_urls {
            let client_id = Self::find_client_id(url).await?;
            if let Some(client_id) = client_id {
                return Ok(Self { client_id });
            }
        }
        Err("Client ID not found".into())
    }

    pub async fn get_json<R: DeserializeOwned, Q: Serialize>(
        base_url: &str,
        path: Option<&str>,
        query: Option<&Q>,
        client_id: &str,
    ) -> Result<R, Box<dyn Error>> {
        let url = match path {
            Some(path) => format!(
                "{}/{}",
                base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            ),
            None => base_url.to_string(),
        };

        let client = reqwest::Client::new();
        let mut request = client.get(&url);

        if let Some(q) = query {
            request = request.query(q);
        }
        request = request.query(&[("client_id", client_id)]);

        let response = request.send().await.map_err(|e| {
            println!("Error sending request: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;

        // let text = response.text().await?;
        // println!("{:?}", text);
        // let body = serde_json::from_str::<R>(&text)?;
        let body = response.json::<R>().await.map_err(|e| {
            println!("Error parsing response: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;

        Ok(body)
    }

    pub async fn get<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<R, Box<dyn Error>> {
        Self::get_json(SOUNDCLOUD_API_URL, Some(path), query, &self.client_id).await
    }

    async fn get_script_urls() -> Result<Vec<String>, Box<dyn Error>> {
        let response = reqwest::get(SOUNDCLOUD_URL).await?;
        let text = response.text().await?;
        let re = Regex::new(r#"https?://[^\s"]+\.js"#).expect("Failed to find script URLs");
        let urls: Vec<String> = re
            .find_iter(&text)
            .map(|mat| mat.as_str().to_string())
            .collect();
        Ok(urls)
    }

    async fn find_client_id(url: String) -> Result<Option<String>, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        let re = Regex::new(r#"client_id[:=]"?(\w{32})"#).expect("Failed to find client ID");
        for cap in re.captures_iter(&text) {
            return Ok(Some(cap[1].to_string()));
        }
        Ok(None)
    }
}
