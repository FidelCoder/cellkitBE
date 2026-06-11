use crate::error::ApiError;

pub async fn dry_run_transaction(
    rpc_url: &str,
    transaction: &serde_json::Value,
) -> Result<serde_json::Value, ApiError> {
    let result = call_rpc(
        rpc_url,
        "dry_run_transaction",
        serde_json::json!([transaction]),
    )
    .await
    .map_err(|error| ApiError::bad_request(format!("CKB RPC dry-run failed: {error}")))?;

    let cycles = result
        .get("cycles")
        .and_then(|value| value.as_str())
        .ok_or_else(|| ApiError::bad_request("invalid CKB RPC dry-run response"))?;

    Ok(serde_json::json!({ "cycles": cycles }))
}

pub async fn broadcast_transaction(
    rpc_url: &str,
    transaction: &serde_json::Value,
) -> Result<String, ApiError> {
    let result = call_rpc(
        rpc_url,
        "send_transaction",
        serde_json::json!([transaction, "passthrough"]),
    )
    .await
    .map_err(|error| ApiError::bad_request(format!("CKB RPC broadcast failed: {error}")))?;

    result
        .as_str()
        .map(ToString::to_string)
        .ok_or_else(|| ApiError::bad_request("invalid CKB RPC broadcast response"))
}

async fn call_rpc(
    rpc_url: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let body = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    });

    let response = reqwest::Client::new()
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|_| "CKB RPC is unreachable".to_string())?;

    if !response.status().is_success() {
        return Err("CKB RPC is unreachable".to_string());
    }

    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|_| "invalid CKB RPC response".to_string())?;

    if let Some(error) = payload.get("error") {
        let message = error
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("CKB RPC returned an error");
        return Err(message.to_string());
    }

    payload
        .get("result")
        .cloned()
        .ok_or_else(|| "invalid CKB RPC response".to_string())
}
