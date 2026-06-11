use crate::{
    actions::validate,
    ckb::rpc,
    config::AppConfig,
    error::ApiError,
    models::{
        requests::{SignedTransactionRequest, ValidateActionRequest},
        responses::{
            BroadcastTransactionResponse, DryRunTransactionResponse,
            ValidateSignedTransactionResponse,
        },
    },
    utils::validation,
};

pub fn validate_signed_transaction(
    request: &SignedTransactionRequest,
) -> Result<ValidateSignedTransactionResponse, ApiError> {
    validation::validate_network(&request.network)?;

    let mut response = validate::validate_transaction_shape(&ValidateActionRequest {
        network: request.network.clone(),
        action: None,
        transaction: request.transaction.clone(),
    })?;

    if witnesses_are_empty(&request.transaction) {
        response
            .errors
            .push("signed transaction is required; witnesses are empty".to_string());
    }

    Ok(ValidateSignedTransactionResponse {
        valid: response.errors.is_empty(),
        errors: response.errors,
        warnings: response.warnings,
    })
}

pub async fn dry_run_signed_transaction(
    request: SignedTransactionRequest,
    config: &AppConfig,
) -> Result<DryRunTransactionResponse, ApiError> {
    let validation = validate_signed_transaction(&request)?;
    if !validation.valid {
        return Err(ApiError::bad_request(first_error(validation.errors)));
    }

    let rpc_url = config.require_rpc_url()?;
    let dry_run = rpc::dry_run_transaction(rpc_url, &request.transaction).await?;

    Ok(DryRunTransactionResponse {
        status: "dry_run_ok".to_string(),
        network: "testnet".to_string(),
        dry_run: Some(dry_run),
        warnings: validation.warnings,
        next_steps: vec![
            "Review dry-run cycles and RPC response".to_string(),
            "Broadcast only after confirming the signed payload".to_string(),
        ],
    })
}

pub async fn broadcast_signed_transaction(
    request: SignedTransactionRequest,
    config: &AppConfig,
) -> Result<BroadcastTransactionResponse, ApiError> {
    let validation = validate_signed_transaction(&request)?;
    if !validation.valid {
        return Err(ApiError::bad_request(first_error(validation.errors)));
    }

    let rpc_url = config.require_rpc_url()?;
    let skip_dry_run = request.skip_dry_run.unwrap_or(false);
    let dry_run = if skip_dry_run {
        Some(serde_json::json!({ "performed": false }))
    } else {
        let dry_run = rpc::dry_run_transaction(rpc_url, &request.transaction)
            .await
            .map_err(|error| {
                ApiError::bad_request(format!(
                    "broadcast rejected because dry-run failed: {error}"
                ))
            })?;
        Some(serde_json::json!({
            "performed": true,
            "cycles": dry_run.get("cycles").and_then(|value| value.as_str()).unwrap_or_default()
        }))
    };

    let tx_hash = rpc::broadcast_transaction(rpc_url, &request.transaction).await?;

    Ok(BroadcastTransactionResponse {
        status: "broadcasted".to_string(),
        network: "testnet".to_string(),
        explorer_url: Some(format!(
            "https://testnet.explorer.nervos.org/transaction/{tx_hash}"
        )),
        tx_hash,
        dry_run,
        next_steps: vec![
            "Track confirmation on the CKB testnet explorer".to_string(),
            "Keep the signed transaction and tx hash for your records".to_string(),
        ],
    })
}

fn first_error(errors: Vec<String>) -> String {
    errors
        .into_iter()
        .next()
        .unwrap_or_else(|| "signed transaction validation failed".to_string())
}

fn witnesses_are_empty(transaction: &serde_json::Value) -> bool {
    let Some(witnesses) = transaction
        .get("witnesses")
        .and_then(|value| value.as_array())
    else {
        return true;
    };

    witnesses.is_empty()
        || witnesses.iter().all(|witness| match witness {
            serde_json::Value::String(value) => value == "0x" || value.is_empty(),
            serde_json::Value::Null => true,
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use super::validate_signed_transaction;
    use crate::models::requests::SignedTransactionRequest;
    use serde_json::json;

    fn request(transaction: serde_json::Value) -> SignedTransactionRequest {
        SignedTransactionRequest {
            network: "testnet".to_string(),
            transaction,
            skip_dry_run: None,
        }
    }

    #[test]
    fn validate_signed_transaction_rejects_empty_witnesses() {
        let response = validate_signed_transaction(&request(json!({
            "version": "0x0",
            "cellDeps": [{"out_point": {"tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000", "index": "0x0"}, "dep_type": "dep_group"}],
            "headerDeps": [],
            "inputs": [{"previous_output": {"tx_hash": "0x0", "index": "0x0"}, "since": "0x0"}],
            "outputs": [{"capacity": "0x1", "lock": {"code_hash": "0x00", "hash_type": "type", "args": "0x"}, "type": null}],
            "outputsData": ["0x"],
            "witnesses": []
        })))
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error == "signed transaction is required; witnesses are empty"));
    }

    #[test]
    fn validate_signed_transaction_accepts_non_empty_witnesses() {
        let response = validate_signed_transaction(&request(json!({
            "version": "0x0",
            "cellDeps": [{"out_point": {"tx_hash": "0x0000000000000000000000000000000000000000000000000000000000000000", "index": "0x0"}, "dep_type": "dep_group"}],
            "headerDeps": [],
            "inputs": [{"previous_output": {"tx_hash": "0x0", "index": "0x0"}, "since": "0x0"}],
            "outputs": [{"capacity": "0x1", "lock": {"code_hash": "0x00", "hash_type": "type", "args": "0x"}, "type": null}],
            "outputsData": ["0x"],
            "witnesses": ["0x1234"]
        })))
        .unwrap();

        assert!(response.valid);
    }
}
