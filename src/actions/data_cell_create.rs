use async_trait::async_trait;

use crate::{
    actions::{live_selection_not_implemented, ActionBuilder},
    config::AppConfig,
    error::ApiError,
    models::requests::DataCellCreateRequest,
    utils::{
        amount::ckb_to_shannons,
        hex::{normalize_hex, utf8_to_hex},
        validation,
    },
};

pub struct DataCellCreateBuilder;

#[async_trait]
impl ActionBuilder<DataCellCreateRequest> for DataCellCreateBuilder {
    fn action_id(&self) -> &'static str {
        "data_cell.create"
    }

    fn validate_request(&self, request: &DataCellCreateRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        validation::validate_testnet_address("fromAddress", &request.from_address)?;
        if request.data.content.trim().is_empty() {
            return Err(ApiError::bad_request("data.content cannot be empty"));
        }
        encoded_data_hex(request)?;
        ckb_to_shannons(&request.capacity_ckb).map_err(ApiError::bad_request)?;
        if let Some(fee_rate) = &request.fee_rate {
            validation::validate_positive_integer("feeRate", fee_rate)?;
        }
        Ok(())
    }

    async fn build(
        &self,
        request: DataCellCreateRequest,
        config: &AppConfig,
    ) -> Result<crate::models::responses::BuildActionResponse, ApiError> {
        self.validate_request(&request)?;
        config.require_indexer_url()?;
        Err(live_selection_not_implemented())
    }

    fn response_summary(&self, request: &DataCellCreateRequest) -> serde_json::Value {
        let capacity_shannons = ckb_to_shannons(&request.capacity_ckb).unwrap_or_default();
        serde_json::json!({
            "fromAddress": request.from_address,
            "encoding": request.data.encoding,
            "dataHex": encoded_data_hex(request).ok(),
            "capacityCkb": request.capacity_ckb,
            "capacityShannons": capacity_shannons.to_string(),
            "estimatedFeeShannons": null,
        })
    }
}

pub fn encoded_data_hex(request: &DataCellCreateRequest) -> Result<String, ApiError> {
    match request.data.encoding.as_str() {
        "utf8" => Ok(utf8_to_hex(&request.data.content)),
        "hex" => normalize_hex(&request.data.content).map_err(ApiError::bad_request),
        _ => Err(ApiError::bad_request(
            "data.encoding must be either utf8 or hex",
        )),
    }
}
