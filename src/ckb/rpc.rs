use crate::error::ApiError;

pub async fn dry_run_transaction(
    _rpc_url: &str,
    _transaction: &serde_json::Value,
) -> Result<serde_json::Value, ApiError> {
    Err(ApiError::not_implemented(
        "CKB RPC dry-run integration is not implemented yet.",
    ))
}
