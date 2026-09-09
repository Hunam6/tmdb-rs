#![cfg(feature = "v4")]

use tmdb_rs::{AccessToken, Client, MediaType};
use wiremock::matchers::{bearer_token, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn v4(server: &MockServer) -> tmdb_rs::V4 {
    let client = Client::with_read_token("read-token").with_base_url(server.uri());
    client.v4(&AccessToken::new("user-token"))
}

#[tokio::test]
async fn v4_uses_its_own_base_and_credential() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/access_token"))
        .and(bearer_token("user-token"))
        .and(body_json(serde_json::json!({"request_token": "req-1"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "status_code": 1,
            "status_message": "Success.",
            "account_id": "abc123",
            "access_token": "user-token-2",
        })))
        .mount(&server)
        .await;

    let answer = v4(&server).v4_access_token("req-1").send().await.unwrap();
    assert_eq!(answer.account_id, "abc123");
    assert_eq!(answer.token().as_str(), "user-token-2");
}

#[tokio::test]
async fn v4_list_items_roundtrip() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/list/1/items"))
        .and(bearer_token("user-token"))
        .and(body_json(serde_json::json!({
            "items": [{"media_type": "movie", "media_id": 550}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "status_code": 1,
            "status_message": "Success.",
            "results": [{"media_id": 550, "media_type": "movie", "success": true}],
        })))
        .mount(&server)
        .await;

    let answer = v4(&server)
        .v4_list_add_items(
            1,
            vec![tmdb_rs::ListItem {
                media_type: MediaType::Movie,
                media_id: 550,
            }],
        )
        .send()
        .await
        .unwrap();
    assert!(answer.results[0].success);
}
