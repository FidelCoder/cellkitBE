use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use cellkit_actions_api::{
    app,
    config::{AppConfig, Network, Secp256k1Config, XudtConfig},
};
use ckb_sdk::{Address, AddressPayload, NetworkType};
use ckb_types::{h160, H160};
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
        secp256k1: Secp256k1Config::default(),
    }
}

fn testnet_address(suffix: u8) -> String {
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64").as_bytes());
    bytes[19] = suffix;
    let hash = H160::from_slice(&bytes).unwrap();
    let payload = AddressPayload::from_pubkey_hash(hash);
    Address::new(NetworkType::Testnet, payload, false).to_string()
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
            "toAddress": testnet_address(2),
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
            "fromAddress": testnet_address(1),
            "toAddress": testnet_address(2),
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
            "fromAddress": testnet_address(1),
            "toAddress": testnet_address(2),
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
            "fromAddress": testnet_address(1),
            "toAddress": testnet_address(2),
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

fn signed_transaction_with_witnesses() -> Value {
    json!({
        "version": "0x0",
        "cellDeps": [{"out_point": {"tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000", "index": "0x0"}, "dep_type": "dep_group"}],
        "headerDeps": [],
        "inputs": [{
            "previous_output": {
                "tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "index": "0x0"
            },
            "since": "0x0"
        }],
        "outputs": [{
            "capacity": "0x174876e800",
            "lock": {
                "code_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "hash_type": "type",
                "args": "0x"
            },
            "type": null
        }],
        "outputsData": ["0x"],
        "witnesses": ["0x1234"]
    })
}

#[tokio::test]
async fn validate_signed_transaction_endpoint_reports_unsigned_payload() {
    let (status, value) = json_request(
        "POST",
        "/api/transactions/validate-signed",
        json!({
            "network": "testnet",
            "transaction": {
                "version": "0x0",
                "cellDeps": [{"out_point": {"tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000", "index": "0x0"}, "dep_type": "dep_group"}],
                "headerDeps": [],
                "inputs": [],
                "outputs": [],
                "outputsData": [],
                "witnesses": []
            }
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(value["valid"], false);
    assert!(value["errors"].as_array().unwrap().iter().any(|error| error
        .as_str()
        .unwrap()
        .contains("signed transaction is required")));
}

#[tokio::test]
async fn validate_signed_transaction_endpoint_accepts_witnesses() {
    let (status, value) = json_request(
        "POST",
        "/api/transactions/validate-signed",
        json!({
            "network": "testnet",
            "transaction": signed_transaction_with_witnesses()
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(value["valid"], true);
}

#[tokio::test]
async fn dry_run_requires_rpc_config() {
    let (status, value) = json_request(
        "POST",
        "/api/transactions/dry-run",
        json!({
            "network": "testnet",
            "transaction": signed_transaction_with_witnesses()
        }),
    )
    .await;

    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(value["error"], "missing_config");
    assert!(value["message"]
        .as_str()
        .unwrap()
        .contains("CKB RPC is not configured"));
}
