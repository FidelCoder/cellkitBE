use serde::Serialize;

use super::transaction::TransactionSkeleton;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionItem {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub endpoint: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActionsResponse {
    pub actions: Vec<ActionItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildActionResponse {
    pub action: String,
    pub network: String,
    pub status: String,
    pub summary: serde_json::Value,
    pub transaction: TransactionSkeleton,
    pub signing: SigningInfo,
    pub warnings: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningInfo {
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_address: Option<String>,
    #[serde(default)]
    pub witness_placeholders: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateActionResponse {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimateFeeResponse {
    pub fee_rate: String,
    pub estimated_size_bytes: usize,
    pub estimated_fee_shannons: String,
    pub estimated_fee_ckb: String,
}

pub fn standard_next_steps() -> Vec<String> {
    vec![
        "Review transaction payload".to_string(),
        "Sign with compatible CKB wallet".to_string(),
        "Broadcast signed transaction".to_string(),
    ]
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSignedTransactionResponse {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunTransactionResponse {
    pub status: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<serde_json::Value>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTransactionResponse {
    pub status: String,
    pub network: String,
    pub tx_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_url: Option<String>,
    #[serde(default)]
    pub next_steps: Vec<String>,
}
