use async_trait::async_trait;

use crate::{
    actions::{live_selection_not_implemented, ActionBuilder},
    config::AppConfig,
    error::ApiError,
    models::{
        requests::CkbTransferRequest,
        responses::{standard_next_steps, BuildActionResponse, SigningInfo},
        transaction::TransactionSkeleton,
    },
    utils::{amount::ckb_to_shannons, validation},
};

pub struct CkbTransferBuilder;

#[async_trait]
impl ActionBuilder<CkbTransferRequest> for CkbTransferBuilder {
    fn action_id(&self) -> &'static str {
        "ckb.transfer"
    }

    fn validate_request(&self, request: &CkbTransferRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        validation::validate_testnet_address("fromAddress", &request.from_address)?;
        validation::validate_testnet_address("toAddress", &request.to_address)?;
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
        config.require_indexer_url()?;
        Err(live_selection_not_implemented())
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
        next_steps: standard_next_steps(),
    }
}
