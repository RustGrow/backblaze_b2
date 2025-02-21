use crate::client::B2Client;
use crate::errors::{B2Error, Result};
use crate::models::Bucket;
use serde_json::json;

impl B2Client {
    /// Lists all buckets in the account.
    ///
    /// # Errors
    /// Returns an error if the client is not authorized or if the API request fails.
    pub async fn list_buckets(&self) -> Result<Vec<Bucket>> {
        let auth = self.get_auth()?;
        let body = json!({
            "accountId": auth.account_id,
        });

        let response: serde_json::Value = self.post("b2_list_buckets", body).await?;

        let buckets: Vec<Bucket> = serde_json::from_value(response["buckets"].clone())
            .map_err(|e| B2Error::InvalidResponse(e.to_string()))?;
        Ok(buckets)
    }

    /// Creates a new bucket.
    ///
    /// # Arguments
    /// * `bucket_name` - The name of the bucket to create.
    /// * `bucket_type` - The type of the bucket (e.g., "allPrivate", "allPublic").
    ///
    /// # Errors
    /// Returns an error if the client is not authorized or if the API request fails.
    pub async fn create_bucket(&self, bucket_name: &str, bucket_type: &str) -> Result<Bucket> {
        let body = json!({
            "accountId": self.get_auth()?.account_id,
            "bucketName": bucket_name,
            "bucketType": bucket_type,
        });

        self.post("b2_create_bucket", body).await
    }
}
