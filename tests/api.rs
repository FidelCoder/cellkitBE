use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use cellkit_actions_api::{
    app,
    config::{AppConfig, Network, XudtConfig},
};
use serde_json::{json, Value};
use tower::ServiceExt;

fn test_config() -> AppConfig {
    AppConfig {
        server_host: "127.0.0.1".to_string(),
        server_port: 8080,
        network: Network::Testnet,
        ckb_rpc_url: None,
        ckb_indexer_url: None,
        default_fee_rate: 1000,
        xudt: XudtConfig::default(),
    }
}

async fn json_request(method: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    let response = app(test_config())
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap_or_else(|_| json!({}));
    (status, value)
}

#[tokio::test]
async fn health_route_returns_ok() {
    let response = app(test_config())
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["status"], "ok");
    assert_eq!(value["service"], "cellkit-actions-api");
}

#[tokio::test]
async fn actions_route_returns_mvp_actions() {
    let response = app(test_config())
        .oneshot(
            Request::builder()
                .uri("/api/actions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["actions"].as_array().unwrap().len(), 5);
    assert!(value["actions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["id"] == "ckb.transfer"));
}

#[tokio::test]
async fn invalid_ckb_transfer_request_returns_400() {
    let (status, value) = json_request(
        "POST",
        "/api/actions/ckb-transfer/build",
        json!({
            "network": "testnet",
            "fromAddress": "not-a-testnet-address",
            "toAddress": "ckt1qqqqqqqqqqqqqqqqq",
            "amountCkb": "0",
            "feeRate": "1000"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(value["error"], "bad_request");
}

#[tokio::test]
async fn invalid_xudt_script_returns_400() {
    let (status, value) = json_request(
        "POST",
        "/api/actions/xudt-transfer/build",
        json!({
            "network": "testnet",
            "fromAddress": "ckt1qqqqqqqqqqqqqqqqq",
            "toAddress": "ckt1qqqqqqqqqqqqqqqq2",
            "xudtTypeScript": {
                "codeHash": "bad",
                "hashType": "type",
                "args": "0x01"
            },
            "amount": "1000",
            "feeRate": "1000"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(value["message"].as_str().unwrap().contains("codeHash"));
}

#[tokio::test]
async fn missing_indexer_config_returns_clear_error() {
    let (status, value) = json_request(
        "POST",
        "/api/actions/ckb-transfer/build",
        json!({
            "network": "testnet",
            "fromAddress": "ckt1qqqqqqqqqqqqqqqqq",
            "toAddress": "ckt1qqqqqqqqqqqqqqqq2",
            "amountCkb": "100",
            "feeRate": "1000"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(value["error"], "missing_config");
    assert!(value["message"]
        .as_str()
        .unwrap()
        .contains("CKB indexer is not configured"));
}

#[tokio::test]
async fn missing_xudt_config_returns_clear_error() {
    let (status, value) = json_request(
        "POST",
        "/api/actions/xudt-transfer/build",
        json!({
            "network": "testnet",
            "fromAddress": "ckt1qqqqqqqqqqqqqqqqq",
            "toAddress": "ckt1qqqqqqqqqqqqqqqq2",
            "xudtTypeScript": {
                "codeHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "hashType": "type",
                "args": "0x01"
            },
            "amount": "1000",
            "feeRate": "1000"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(value["message"]
        .as_str()
        .unwrap()
        .contains("xUDT cell dep is not configured"));
}
