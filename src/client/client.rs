use regex::Regex;
use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;
use tokio::sync::RwLock;

use crate::constants::{SOUNDCLOUD_API_URL, SOUNDCLOUD_URL};

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_on_401: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 1,
            retry_on_401: true,
        }
    }
}

#[derive(Debug)]
pub struct Client {
    client_id: RwLock<String>,
    retry_config: RetryConfig,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Self::with_retry_config(RetryConfig::default()).await
    }

    pub async fn with_retry_config(retry_config: RetryConfig) -> Result<Self, Box<dyn Error>> {
        let client_id = Self::get_client_id().await?;
        Ok(Self { client_id: RwLock::new(client_id), retry_config })
    }

    pub async fn refresh_client_id(&self) -> Result<(), Box<dyn Error>> {
        let new_client_id = Self::get_client_id().await?;
        *self.client_id.write().await = new_client_id;
        Ok(())
    }

    pub async fn get_client_id_value(&self) -> String {
        self.client_id.read().await.clone()
    }

    pub async fn get_json<R: DeserializeOwned, Q: Serialize>(
        base_url: &str,
        path: Option<&str>,
        query: Option<&Q>,
        client_id: &str,
    ) -> Result<(R, u16), Box<dyn Error>> {
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
            println!("Error sending request: {e}");
            Box::new(e) as Box<dyn Error>
        })?;

        let status = response.status().as_u16();

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(format!("HTTP {}: {}", status, text).into());
        }

        // Parse JSON body for successful responses
        let body = response.json::<R>().await.map_err(|e| {
            println!("Error parsing response: {e}");
            Box::new(e) as Box<dyn Error>
        })?;

        Ok((body, status))
    }

    pub async fn get<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<R, Box<dyn Error>> {
        let mut retries = 0;
        let max_retries = self.retry_config.max_retries;

        loop {
            let client_id = self.client_id.read().await.clone();
            let result = Self::get_json(SOUNDCLOUD_API_URL, Some(path), query, &client_id).await;

            match result {
                Ok((body, _status)) => {
                    return Ok(body);
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    // Check if we got a 401 and should retry
                    if error_msg.contains("401") 
                        && self.retry_config.retry_on_401 
                        && retries < max_retries {
                        retries += 1;
                        println!("Received 401, refreshing client_id and retrying (attempt {retries}/{max_retries})");
                        self.refresh_client_id().await?;
                        continue;
                    }
                    // For non-401 errors or if we've exhausted retries, return the error
                    return Err(e);
                }
            }
        }
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
        if let Some(cap) = re.captures_iter(&text).next() {
            return Ok(Some(cap[1].to_string()));
        }
        Ok(None)
    }

    async fn get_client_id() -> Result<String, Box<dyn Error>> {
        let script_urls = Self::get_script_urls().await?;
        for url in script_urls {
            let client_id = Self::find_client_id(url).await?;
            if let Some(client_id) = client_id {
                return Ok(client_id);
            }
        }
        Err("Client ID not found".into())
    }
}
