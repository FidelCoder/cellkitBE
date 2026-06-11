use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CkbTransferRequest {
    pub network: String,
    pub from_address: String,
    pub to_address: String,
    pub amount_ckb: String,
    pub fee_rate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptRequest {
    pub code_hash: String,
    pub hash_type: String,
    pub args: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XudtTransferRequest {
    pub network: String,
    pub from_address: String,
    pub to_address: String,
    pub xudt_type_script: ScriptRequest,
    pub amount: String,
    pub fee_rate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellConsolidationRequest {
    pub network: String,
    pub owner_address: String,
    pub max_cells: Option<u16>,
    pub fee_rate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapacityLockRequest {
    pub network: String,
    pub from_address: String,
    pub lock_address: String,
    pub amount_ckb: String,
    pub memo: Option<String>,
    pub fee_rate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataCellCreateRequest {
    pub network: String,
    pub from_address: String,
    pub data: DataPayloadRequest,
    pub capacity_ckb: String,
    pub fee_rate: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPayloadRequest {
    pub encoding: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateActionRequest {
    pub network: String,
    pub action: Option<String>,
    pub transaction: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateFeeRequest {
    pub network: String,
    pub transaction: serde_json::Value,
    pub fee_rate: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionRequest {
    pub network: String,
    pub transaction: serde_json::Value,
    pub skip_dry_run: Option<bool>,
}
