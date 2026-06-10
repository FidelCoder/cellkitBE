use async_trait::async_trait;

use crate::{
    actions::{live_selection_not_implemented, ActionBuilder},
    config::AppConfig,
    error::ApiError,
    models::requests::CellConsolidationRequest,
    utils::validation,
};

pub struct CellConsolidationBuilder;

#[async_trait]
impl ActionBuilder<CellConsolidationRequest> for CellConsolidationBuilder {
    fn action_id(&self) -> &'static str {
        "cell.consolidate"
    }

    fn validate_request(&self, request: &CellConsolidationRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        validation::validate_testnet_address("ownerAddress", &request.owner_address)?;
        let max_cells = request.max_cells.unwrap_or(20);
        if max_cells == 0 || max_cells > 100 {
            return Err(ApiError::bad_request("maxCells must be between 1 and 100"));
        }
        if let Some(fee_rate) = &request.fee_rate {
            validation::validate_positive_integer("feeRate", fee_rate)?;
        }
        Ok(())
    }

    async fn build(
        &self,
        request: CellConsolidationRequest,
        config: &AppConfig,
    ) -> Result<crate::models::responses::BuildActionResponse, ApiError> {
        self.validate_request(&request)?;
        config.require_indexer_url()?;
        Err(live_selection_not_implemented())
    }

    fn response_summary(&self, request: &CellConsolidationRequest) -> serde_json::Value {
        serde_json::json!({
            "ownerAddress": request.owner_address,
            "maxCells": request.max_cells.unwrap_or(20),
            "selectedCellCount": null,
            "estimatedFeeShannons": null,
        })
    }
}
