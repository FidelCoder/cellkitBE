use async_trait::async_trait;

use crate::{
    actions::{live_selection_not_implemented, ActionBuilder},
    config::AppConfig,
    error::ApiError,
    models::requests::CapacityLockRequest,
    utils::{amount::ckb_to_shannons, validation},
};

pub struct CapacityLockBuilder;

#[async_trait]
impl ActionBuilder<CapacityLockRequest> for CapacityLockBuilder {
    fn action_id(&self) -> &'static str {
        "capacity.lock"
    }

    fn validate_request(&self, request: &CapacityLockRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        validation::validate_testnet_address("fromAddress", &request.from_address)?;
        validation::validate_testnet_address("lockAddress", &request.lock_address)?;
        ckb_to_shannons(&request.amount_ckb).map_err(ApiError::bad_request)?;
        if let Some(fee_rate) = &request.fee_rate {
            validation::validate_positive_integer("feeRate", fee_rate)?;
        }
        Ok(())
    }

    async fn build(
        &self,
        request: CapacityLockRequest,
        config: &AppConfig,
    ) -> Result<crate::models::responses::BuildActionResponse, ApiError> {
        self.validate_request(&request)?;
        config.require_indexer_url()?;
        Err(live_selection_not_implemented())
    }

    fn response_summary(&self, request: &CapacityLockRequest) -> serde_json::Value {
        let amount_shannons = ckb_to_shannons(&request.amount_ckb).unwrap_or_default();
        serde_json::json!({
            "fromAddress": request.from_address,
            "lockAddress": request.lock_address,
            "amountCkb": request.amount_ckb,
            "amountShannons": amount_shannons.to_string(),
            "memo": request.memo,
            "estimatedFeeShannons": null,
        })
    }
}
