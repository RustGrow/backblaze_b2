use backblaze_b2::client::B2Client;
use backblaze_b2::config::Config;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_authorize_account() {
    let mock_server = MockServer::start().await;
    let config = Config {
        application_key_id: "test_id".to_string(),
        application_key: "test_key".to_string(),
        api_base_url: mock_server.uri(),
    };

    let auth_response = serde_json::json!({
        "accountId": "test_account",
        "authorizationToken": "test_token",
        "apiUrl": mock_server.uri(),
        "downloadUrl": mock_server.uri(),
        "recommendedPartSize": 100000000,
        "absoluteMinimumPartSize": 5000000
    });

    Mock::given(method("GET"))
        .and(path("/b2api/v3/b2_authorize_account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&auth_response))
        .mount(&mock_server)
        .await;

    let mut client = B2Client::new(config);
    let result = client.authorize_account().await;
    assert!(result.is_ok());

    let auth = client.get_auth().unwrap();
    assert_eq!(auth.account_id, "test_account");
    assert_eq!(auth.authorization_token, "test_token");
}
