use std::error::Error;

use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};

use crate::constants::{SOUNDCLOUD_URL, SOUNDCLOUD_API_URL};

#[derive(Debug)]
pub struct Client {
    client_id: String,
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

    pub async fn get<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<R, Box<dyn Error>> {
        let url = format!(
            "{}/{}",
            SOUNDCLOUD_API_URL,
            path.trim_start_matches('/')
        );

        let client = reqwest::Client::new();
        let mut request = client.get(&url);
        if let Some(query) = query {
            request = request.query(query);
        }
        request = request.query(&[("client_id", &self.client_id)]);

        let response = request.send().await.map_err(|e| {
            println!("Error sending request: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;

        let body = response.text().await?;
        println!("Body: {}", body);
        println!("Bello");
        let body: R = serde_json::from_str(&body).unwrap();
        // let body = response.json::<R>().await.map_err(|e| {
        //     println!("Error parsing SoundCloud API response: {}", e);
        //     Box::new(e) as Box<dyn Error>
        // })?;
        Ok(body)
    }

    async fn get_script_urls() -> Result<Vec<String>, Box<dyn Error>> {
        let response = reqwest::get(SOUNDCLOUD_URL).await?;
        let text = response.text().await?;
        let re = Regex::new(r#"https?://[^\s"]+\.js"#).unwrap();
        let urls: Vec<String> = re.find_iter(&text).map(|mat| mat.as_str().to_string()).collect();
        Ok(urls)
    }

    async fn find_client_id(url: String) -> Result<Option<String>, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        let re = Regex::new(r#"client_id[:=]"?(\w{32})"#).unwrap();
        for cap in re.captures_iter(&text) {
            return Ok(Some(cap[1].to_string()));
        }
        Ok(None)
    }
}
