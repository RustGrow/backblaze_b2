# Backblaze B2 Rust Library

A Rust library for interacting with the Backblaze B2 Native API. This library provides an easy-to-use, asynchronous interface for managing buckets, uploading and downloading files, and other B2 operations.

## Features

- Asynchronous API using `tokio` and `reqwest`.
- Support for bucket management (create, list, delete).
- File operations (upload, download, list, delete).
- Comprehensive error handling.
- Configuration via `.env` file.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
backblaze_b2 = "0.1.0"
Or install via Cargo:


cargo add backblaze_b2
Usage
Configuration
Create a .env file in your project root:


B2_APPLICATION_KEY_ID=your_application_key_id
B2_APPLICATION_KEY=your_application_key
B2_API_BASE_URL=https://api.backblazeb2.com
Example

use backblaze_b2::{config::Config, client::B2Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load()?;
    let mut client = B2Client::new(config);

    // Authorize account
    client.authorize_account().await?;

    // List buckets
    let buckets = client.list_buckets().await?;
    println!("Buckets: {:?}", buckets);

    // Upload a file
    let content = b"Hello, B2!".to_vec();
    let file = client.upload_file("bucket_id", "example.txt", content).await?;
    println!("Uploaded file: {:?}", file);

    Ok(())
}
Documentation
Full API documentation is available at GitHub Pages.

Contributing
See CONTRIBUTING.md for details on how to contribute.

License
This project is licensed under the MIT License - see the LICENSE file for details.