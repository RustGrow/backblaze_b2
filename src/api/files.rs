use crate::client::B2Client;
use crate::errors::B2Error;
use crate::errors::Result;
use crate::models::File;
use serde_json::json;
use sha1::{Digest, Sha1};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

impl B2Client {
    /// Lists files in a bucket.
    pub async fn list_files(
        &self,
        bucket_id: &str,
        start_file_name: Option<&str>,
    ) -> Result<Vec<File>> {
        let body = json!({
            "bucketId": bucket_id,
            "startFileName": start_file_name,
        });

        let response: serde_json::Value = self.post("b2_list_file_names", body).await?;

        let files: Vec<File> = serde_json::from_value(response["files"].clone())
            .map_err(|e| B2Error::InvalidResponse(e.to_string()))?;
        Ok(files)
    }

    /// Uploads a file to a bucket.
    pub async fn upload_file(
        &self,
        bucket_id: &str,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<File> {
        let upload_url = self.get_upload_url(bucket_id).await?;

        let mut hasher = Sha1::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);

        let response = self
            .get_client()
            .post(&upload_url.url)
            .header("Authorization", upload_url.authorization_token)
            .header("X-Bz-File-Name", file_name)
            .header("Content-Type", "b2/x-auto")
            .header("X-Bz-Content-Sha1", hash_hex)
            .body(content)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Uploads a file from a stream.
    pub async fn upload_file_stream(
        &self,
        bucket_id: &str,
        file_name: &str,
        mut file: TokioFile,
    ) -> Result<File> {
        let upload_url = self.get_upload_url(bucket_id).await?;

        let mut hasher = Sha1::new();
        let mut buffer = [0u8; 8192];
        let mut content = Vec::new();

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            content.extend_from_slice(&buffer[..n]);
        }

        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);

        let response = self
            .get_client()
            .post(&upload_url.url)
            .header("Authorization", upload_url.authorization_token)
            .header("X-Bz-File-Name", file_name)
            .header("Content-Type", "b2/x-auto")
            .header("X-Bz-Content-Sha1", hash_hex)
            .body(content)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Gets an upload URL for a bucket.
    async fn get_upload_url(&self, bucket_id: &str) -> Result<UploadUrl> {
        let body = json!({
            "bucketId": bucket_id,
        });

        self.post("b2_get_upload_url", body).await
    }
}

#[derive(serde::Deserialize)]
struct UploadUrl {
    url: String,
    authorization_token: String,
}
