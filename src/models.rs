use serde::{Deserialize, Serialize};

/// Response from b2_authorize_account.
#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "accountId")] // Указываем, что в JSON используется "accountId"
    pub account_id: String,
    #[serde(rename = "authorizationToken")]
    pub authorization_token: String,
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "recommendedPartSize")]
    pub recommended_part_size: u64,
    #[serde(rename = "absoluteMinimumPartSize")]
    pub absolute_minimum_part_size: u64,
}

/// Bucket information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(rename = "bucketType")]
    pub bucket_type: String,
}

/// File information.
#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "contentLength")]
    pub content_length: u64,
    #[serde(rename = "contentSha1")]
    pub content_sha1: String,
    #[serde(rename = "uploadTimestamp")]
    pub upload_timestamp: u64,
}
