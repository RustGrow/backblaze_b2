use crate::client::B2Client;
use crate::errors::Result;

impl B2Client {
    /// Authorizes the account (wrapper for client method).
    pub async fn authorize_account(&mut self) -> Result<()> {
        self.authorize().await
    }
}
