use std::error::Error;

use crate::client::client::{Client, RetryConfig};

#[derive(Debug)]
pub struct ClientBuilder {
    retry_config: RetryConfig,
}

impl ClientBuilder {
    /// Create a new ClientBuilder with default retry configuration.
    pub fn new() -> Self {
        Self {
            retry_config: RetryConfig::default(),
        }
    }

    /// Set the maximum number of retry attempts.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.retry_config.max_retries = max_retries;
        self
    }

    /// Enable or disable retrying on 401 Unauthorized responses.
    pub fn with_retry_on_401(mut self, retry_on_401: bool) -> Self {
        self.retry_config.retry_on_401 = retry_on_401;
        self
    }

    /// Build the Client with the configured settings.
    pub async fn build(self) -> Result<Client, Box<dyn Error>> {
        Client::with_retry_config(self.retry_config).await
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

