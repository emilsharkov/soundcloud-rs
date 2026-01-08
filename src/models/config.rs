/// Configuration for client retry behavior
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

