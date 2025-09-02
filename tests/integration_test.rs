use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use linebot_rs::{Config, create_app};
use serde_json::json;
use tower::ServiceExt;

fn create_test_config() -> Config {
    Config {
        channel_access_token: "test_channel_access_token".to_string(),
        channel_secret: "test_channel_secret".to_string(),
        port: 3000,
        host: "0.0.0.0".to_string(),
    }
}

fn create_test_signature(channel_secret: &str, body: &str) -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
    mac.update(body.as_bytes());
    let signature = mac.finalize().into_bytes();
    let encoded_signature = STANDARD.encode(signature);
    format!("sha256={}", encoded_signature)
}

#[tokio::test]
async fn test_health_check() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn test_webhook_missing_signature() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": []
    })
    .to_string();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_webhook_invalid_signature() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": []
    })
    .to_string();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .header("x-line-signature", "sha256=invalid_signature")
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_webhook_valid_signature() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": []
    })
    .to_string();

    let signature = create_test_signature(&config.channel_secret, &body);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .header("x-line-signature", signature)
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_text_message() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": [{
            "type": "message",
            "replyToken": "reply_token_123",
            "message": {
                "type": "text",
                "text": "hello"
            },
            "timestamp": 1234567890,
            "source": {
                "type": "user",
                "userId": "user_123"
            },
            "mode": "active"
        }]
    })
    .to_string();

    let signature = create_test_signature(&config.channel_secret, &body);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .header("x-line-signature", signature)
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_follow_event() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": [{
            "type": "follow",
            "replyToken": "reply_token_456",
            "timestamp": 1234567890,
            "source": {
                "type": "user",
                "userId": "user_456"
            },
            "mode": "active"
        }]
    })
    .to_string();

    let signature = create_test_signature(&config.channel_secret, &body);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .header("x-line-signature", signature)
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_sticker_message() {
    let config = create_test_config();
    let app = create_app(config.clone());

    let body = json!({
        "destination": "test",
        "events": [{
            "type": "message",
            "replyToken": "reply_token_789",
            "message": {
                "type": "sticker",
                "packageId": "1",
                "stickerId": "1"
            },
            "timestamp": 1234567890,
            "source": {
                "type": "user",
                "userId": "user_789"
            },
            "mode": "active"
        }]
    })
    .to_string();

    let signature = create_test_signature(&config.channel_secret, &body);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/webhook")
        .header("content-type", "application/json")
        .header("x-line-signature", signature)
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
