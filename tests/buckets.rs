use backblaze_b2::client::B2Client;
use backblaze_b2::config::Config;
use backblaze_b2::errors::B2Error;
use backblaze_b2::models::Bucket;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Общая настройка для тестов
mod common {
    use super::*;

    pub async fn setup_mock_server() -> MockServer {
        MockServer::start().await
    }

    pub fn default_config(mock_server: &MockServer) -> Config {
        Config {
            application_key_id: "test_id".to_string(),
            application_key: "test_key".to_string(),
            api_base_url: mock_server.uri(),
        }
    }

    pub fn auth_response(mock_server: &MockServer) -> serde_json::Value {
        serde_json::json!({
            "accountId": "test_account",
            "authorizationToken": "test_token",
            "apiUrl": mock_server.uri(),
            "downloadUrl": mock_server.uri(),
            "recommendedPartSize": 100000000,
            "absoluteMinimumPartSize": 5000000
        })
    }

    pub fn success_buckets_response() -> serde_json::Value {
        serde_json::json!({
            "buckets": [
                {
                    "bucketId": "bucket1",
                    "bucketName": "test-bucket",
                    "bucketType": "allPrivate"
                }
            ]
        })
    }
}

// --- Успешные сценарии ---

#[tokio::test]
async fn test_list_buckets_success() {
    let mock_server = common::setup_mock_server().await;
    let config = common::default_config(&mock_server);

    // Настройка mock-ответа для авторизации
    Mock::given(method("GET"))
        .and(path("/b2api/v3/b2_authorize_account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::auth_response(&mock_server)))
        .mount(&mock_server)
        .await;

    // Настройка mock-ответа для списка бакетов
    Mock::given(method("POST"))
        .and(path("/b2api/v3/b2_list_buckets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::success_buckets_response()))
        .mount(&mock_server)
        .await;

    let mut client = B2Client::new(config);
    let auth_result = client.authorize_account().await;
    assert!(
        auth_result.is_ok(),
        "Authorization failed: {:?}",
        auth_result.err()
    );

    let buckets_result = client.list_buckets().await;
    assert!(
        buckets_result.is_ok(),
        "List buckets failed: {:?}",
        buckets_result
    );

    let buckets = buckets_result.unwrap();
    assert_eq!(buckets.len(), 1);
    assert_eq!(buckets[0].bucket_id, "bucket1");
    assert_eq!(buckets[0].bucket_name, "test-bucket");
}

// --- Ошибки ---

#[tokio::test]
async fn test_list_buckets_unauthorized() {
    let mock_server = common::setup_mock_server().await;
    let config = common::default_config(&mock_server);

    // Настройка mock-ответа для авторизации
    Mock::given(method("GET"))
        .and(path("/b2api/v3/b2_authorize_account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::auth_response(&mock_server)))
        .mount(&mock_server)
        .await;

    // Настройка mock-ответа для ошибки 401
    let error_response = serde_json::json!({
        "status": 401,
        "code": "unauthorized",
        "message": "Unauthorized"
    });

    Mock::given(method("POST"))
        .and(path("/b2api/v3/b2_list_buckets"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    let mut client = B2Client::new(config);
    let auth_result = client.authorize_account().await;
    assert!(
        auth_result.is_ok(),
        "Authorization failed: {:?}",
        auth_result.err()
    );

    let buckets_result = client.list_buckets().await;
    assert!(
        buckets_result.is_err(),
        "Expected error, but got: {:?}",
        buckets_result
    );

    if let Err(B2Error::ApiError(error)) = buckets_result {
        assert_eq!(error.status, 401);
        assert_eq!(error.code, "unauthorized");
        assert_eq!(error.message, "Unauthorized");
    } else {
        panic!("Expected ApiError, but got: {:?}", buckets_result);
    }
}

#[tokio::test]
async fn test_list_buckets_retry_429() {
    let mock_server = common::setup_mock_server().await;
    let config = common::default_config(&mock_server);

    // Настройка mock-ответа для авторизации
    Mock::given(method("GET"))
        .and(path("/b2api/v3/b2_authorize_account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(common::auth_response(&mock_server)))
        .mount(&mock_server)
        .await;

    // Настройка mock-ответа для последовательных попыток
    let error_response = serde_json::json!({
        "status": 429,
        "code": "too_many_requests",
        "message": "Too many requests"
    });

    let success_response = common::success_buckets_response();

    let mut attempt = 0;
    Mock::given(method("POST"))
        .and(path("/b2api/v3/b2_list_buckets"))
        .respond_with_fn(move |_| {
            attempt += 1;
            if attempt <= 2 {
                ResponseTemplate::new(429).set_body_json(error_response.clone())
            } else {
                ResponseTemplate::new(200).set_body_json(success_response.clone())
            }
        })
        .expect(3) // Ожидаем 3 попытки (2 ошибки + 1 успех)
        .mount(&mock_server)
        .await;

    let mut client = B2Client::new(config);
    let auth_result = client.authorize_account().await;
    assert!(
        auth_result.is_ok(),
        "Authorization failed: {:?}",
        auth_result.err()
    );

    // Добавим логирование для отладки
    println!("Starting list_buckets request");
    let buckets_result = client.list_buckets().await;
    println!("List buckets result: {:?}", buckets_result);

    assert!(
        buckets_result.is_ok(),
        "List buckets failed: {:?}",
        buckets_result
    );

    let buckets = buckets_result.unwrap();
    assert_eq!(buckets.len(), 1);
    assert_eq!(buckets[0].bucket_id, "bucket1");
    assert_eq!(buckets[0].bucket_name, "test-bucket");

    // Проверяем, что mock-сервер обработал все ожидаемые запросы
    mock_server.verify().await;
}
