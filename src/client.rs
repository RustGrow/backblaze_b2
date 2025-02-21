use crate::config::Config;
use crate::errors::B2ErrorResponse;
use crate::errors::{B2Error, Result};
use crate::models::Authorization;
use reqwest::Client;
use serde::de::DeserializeOwned;

/// Client for interacting with Backblaze B2 API.
///
/// # Example
/// ```rust
/// use backblaze_b2::{config::Config, client::B2Client};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::load()?;
///     let mut client = B2Client::new(config);
///     client.authorize_account().await?;
///     let buckets = client.list_buckets().await?;
///     println!("Buckets: {:?}", buckets);
///     Ok(())
/// }
/// ```
pub struct B2Client {
    config: Config,
    client: Client,
    auth: Option<Authorization>,
}

impl B2Client {
    /// Creates a new B2Client instance.
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        Self {
            config,
            client,
            auth: None,
        }
    }

    /// Returns a reference to the HTTP client.
    pub fn get_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Authorizes the account using application key ID and key.
    pub async fn authorize(&mut self) -> Result<()> {
        let url = format!("{}/b2api/v3/b2_authorize_account", self.config.api_base_url);
        let response = self
            .client
            .get(&url)
            .basic_auth(
                &self.config.application_key_id,
                Some(&self.config.application_key),
            )
            .send()
            .await?;

        self.handle_response(response).await.map(|auth| {
            self.auth = Some(auth);
        })
    }

    /// Returns the authorization data if available.
    pub fn get_auth(&self) -> Result<&Authorization> {
        self.auth
            .as_ref()
            .ok_or_else(|| B2Error::InvalidResponse("Client not authorized".to_string()))
    }

    /// Sends a GET request to the API.
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| B2Error::InvalidResponse("Client not authorized".to_string()))?;

        let url = format!("{}/b2api/v3/{}", auth.api_url, endpoint);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&auth.authorization_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Sends a POST request to the API.
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<T> {
        let auth = self.get_auth()?;

        let url = format!("{}/b2api/v3/{}", auth.api_url, endpoint);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&auth.authorization_token)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handles API response and errors.
    pub async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        if status.is_success() {
            response.json().await.map_err(|e| B2Error::HttpError(e))
        } else {
            let error: B2ErrorResponse =
                response.json().await.map_err(|e| B2Error::HttpError(e))?;
            Err(B2Error::ApiError(error))
        }
    }
}
