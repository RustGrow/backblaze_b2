use dotenv::dotenv;
use std::env;

/// Configuration for Backblaze B2 API.
#[derive(Debug, Clone)]
pub struct Config {
    pub application_key_id: String,
    pub application_key: String,
    pub api_base_url: String,
}

impl Config {
    /// Loads configuration from environment variables or .env file.
    pub fn load() -> Result<Self, env::VarError> {
        dotenv().ok(); // Load .env file if present

        Ok(Config {
            application_key_id: env::var("B2_APPLICATION_KEY_ID")?,
            application_key: env::var("B2_APPLICATION_KEY")?,
            api_base_url: env::var("B2_API_BASE_URL")
                .unwrap_or_else(|_| "https://api.backblazeb2.com".to_string()),
        })
    }
}
