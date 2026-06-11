use async_trait::async_trait;

use crate::{
    actions::ActionBuilder,
    ckb::{
        address::{parse_address_to_script, script_to_json},
        indexer::get_ordinary_live_cells_by_lock,
        scripts::get_secp256k1_cell_dep,
        tx::{build_ckb_transfer_skeleton, estimate_transaction_size_bytes},
    },
    config::{AppConfig, Network},
    error::ApiError,
    models::{
        requests::CkbTransferRequest,
        responses::{BuildActionResponse, SigningInfo},
        transaction::TransactionSkeleton,
    },
    utils::{amount::ckb_to_shannons, fee::estimate_fee_shannons, validation},
};

const DEFAULT_CELL_QUERY_LIMIT: u32 = 100;
const INITIAL_ESTIMATED_TX_SIZE_BYTES: usize = 1_000;

pub struct CkbTransferBuilder;

#[async_trait]
impl ActionBuilder<CkbTransferRequest> for CkbTransferBuilder {
    fn action_id(&self) -> &'static str {
        "ckb.transfer"
    }

    fn validate_request(&self, request: &CkbTransferRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        parse_address_to_script(&request.from_address, Network::Testnet)?;
        parse_address_to_script(&request.to_address, Network::Testnet)?;
        ckb_to_shannons(&request.amount_ckb).map_err(ApiError::bad_request)?;
        if let Some(fee_rate) = &request.fee_rate {
            validation::validate_positive_integer("feeRate", fee_rate)?;
        }
        Ok(())
    }

    async fn build(
        &self,
        request: CkbTransferRequest,
        config: &AppConfig,
    ) -> Result<BuildActionResponse, ApiError> {
        self.validate_request(&request)?;

        let indexer_url = config.require_indexer_url()?;
        let secp256k1_cell_dep = get_secp256k1_cell_dep(config, Network::Testnet)?;
        let from_lock_script = parse_address_to_script(&request.from_address, Network::Testnet)?;
        let to_lock_script = parse_address_to_script(&request.to_address, Network::Testnet)?;
        let amount_shannons = ckb_to_shannons_u64(&request.amount_ckb)?;
        let fee_rate = fee_rate_or_default(&request, config)?;

        let live_cells = get_ordinary_live_cells_by_lock(
            indexer_url,
            from_lock_script.clone(),
            DEFAULT_CELL_QUERY_LIMIT,
        )
        .await?;

        let from_lock_json = script_to_json(&from_lock_script);
        let to_lock_json = script_to_json(&to_lock_script);

        let initial_fee = estimate_fee_shannons(INITIAL_ESTIMATED_TX_SIZE_BYTES, fee_rate) as u64;
        let first_selection = crate::ckb::cells::select_cells_for_capacity(
            &live_cells,
            amount_shannons,
            initial_fee,
        )?;
        let first_transaction = build_ckb_transfer_skeleton(
            &first_selection,
            from_lock_json.clone(),
            to_lock_json.clone(),
            secp256k1_cell_dep.clone(),
        );
        let first_size = estimate_transaction_size_bytes(&first_transaction)?;
        let refined_fee = estimate_fee_shannons(first_size, fee_rate) as u64;

        let mut final_selection = crate::ckb::cells::select_cells_for_capacity(
            &live_cells,
            amount_shannons,
            refined_fee,
        )?;
        let mut transaction = build_ckb_transfer_skeleton(
            &final_selection,
            from_lock_json,
            to_lock_json,
            secp256k1_cell_dep,
        );
        let mut estimated_size_bytes = estimate_transaction_size_bytes(&transaction)?;
        let final_fee_from_size = estimate_fee_shannons(estimated_size_bytes, fee_rate) as u64;

        if final_fee_from_size > final_selection.estimated_fee_shannons {
            final_selection = crate::ckb::cells::select_cells_for_capacity(
                &live_cells,
                amount_shannons,
                final_fee_from_size,
            )?;
            transaction = build_ckb_transfer_skeleton(
                &final_selection,
                script_to_json(&from_lock_script),
                script_to_json(&to_lock_script),
                get_secp256k1_cell_dep(config, Network::Testnet)?,
            );
            estimated_size_bytes = estimate_transaction_size_bytes(&transaction)?;
        }

        let warnings = final_selection.warnings();

        Ok(BuildActionResponse {
            action: self.action_id().to_string(),
            network: request.network.clone(),
            status: "built".to_string(),
            summary: serde_json::json!({
                "fromAddress": request.from_address,
                "toAddress": request.to_address,
                "amountCkb": request.amount_ckb,
                "amountShannons": amount_shannons.to_string(),
                "selectedCellCount": final_selection.selected_cells.len(),
                "totalInputShannons": final_selection.total_input_shannons.to_string(),
                "changeShannons": final_selection.change_shannons.to_string(),
                "estimatedFeeShannons": final_selection.estimated_fee_shannons.to_string(),
                "estimatedSizeBytes": estimated_size_bytes,
                "feeRate": fee_rate.to_string(),
            }),
            transaction,
            signing: SigningInfo {
                required: true,
                signer_address: Some(request.from_address),
                witness_placeholders: vec![serde_json::json!({
                    "lock": "65-byte zeroed secp256k1 signature placeholder"
                })],
            },
            warnings,
            next_steps: vec![
                "Review the unsigned transaction payload".to_string(),
                "Sign the transaction with a compatible CKB wallet".to_string(),
                "Broadcast the signed transaction using CKB RPC or wallet tooling".to_string(),
            ],
        })
    }

    fn response_summary(&self, request: &CkbTransferRequest) -> serde_json::Value {
        let amount_shannons = ckb_to_shannons(&request.amount_ckb).unwrap_or_default();
        serde_json::json!({
            "fromAddress": request.from_address,
            "toAddress": request.to_address,
            "amountCkb": request.amount_ckb,
            "amountShannons": amount_shannons.to_string(),
            "estimatedFeeShannons": null,
        })
    }
}

fn ckb_to_shannons_u64(amount_ckb: &str) -> Result<u64, ApiError> {
    let shannons = ckb_to_shannons(amount_ckb).map_err(ApiError::bad_request)?;
    shannons
        .try_into()
        .map_err(|_| ApiError::bad_request("amountCkb is too large"))
}

fn fee_rate_or_default(request: &CkbTransferRequest, config: &AppConfig) -> Result<u64, ApiError> {
    match &request.fee_rate {
        Some(fee_rate) => validation::validate_positive_integer("feeRate", fee_rate),
        None => Ok(config.default_fee_rate),
    }
}

pub fn placeholder_response(request: &CkbTransferRequest) -> BuildActionResponse {
    BuildActionResponse {
        action: "ckb.transfer".to_string(),
        network: request.network.clone(),
        status: "built".to_string(),
        summary: CkbTransferBuilder.response_summary(request),
        transaction: TransactionSkeleton::default(),
        signing: SigningInfo {
            required: true,
            signer_address: Some(request.from_address.clone()),
            witness_placeholders: Vec::new(),
        },
        warnings: Vec::new(),
        next_steps: vec![
            "Review the unsigned transaction payload".to_string(),
            "Sign the transaction with a compatible CKB wallet".to_string(),
            "Broadcast the signed transaction using CKB RPC or wallet tooling".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::CkbTransferBuilder;
    use crate::{
        actions::ActionBuilder,
        config::{AppConfig, Network, Secp256k1Config, XudtConfig},
        models::requests::CkbTransferRequest,
    };
    use ckb_sdk::{Address, AddressPayload, NetworkType};
    use ckb_types::{h160, H160};

    fn testnet_address(suffix: u8) -> String {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64").as_bytes());
        bytes[19] = suffix;
        let hash = H160::from_slice(&bytes).unwrap();
        let payload = AddressPayload::from_pubkey_hash(hash);
        Address::new(NetworkType::Testnet, payload, false).to_string()
    }

    fn request() -> CkbTransferRequest {
        CkbTransferRequest {
            network: "testnet".to_string(),
            from_address: testnet_address(1),
            to_address: testnet_address(2),
            amount_ckb: "100".to_string(),
            fee_rate: Some("1000".to_string()),
        }
    }

    fn config() -> AppConfig {
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

    #[test]
    fn invalid_from_address_returns_error() {
        let mut request = request();
        request.from_address = "not-an-address".to_string();
        let error = CkbTransferBuilder.validate_request(&request).unwrap_err();
        assert!(error.to_string().contains("invalid CKB address"));
    }

    #[test]
    fn invalid_to_address_returns_error() {
        let mut request = request();
        request.to_address = "not-an-address".to_string();
        let error = CkbTransferBuilder.validate_request(&request).unwrap_err();
        assert!(error.to_string().contains("invalid CKB address"));
    }

    #[tokio::test]
    async fn missing_indexer_config_returns_clear_error() {
        let error = CkbTransferBuilder
            .build(request(), &config())
            .await
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "CKB indexer is not configured. Set CKB_INDEXER_URL."
        );
    }
}
