use async_trait::async_trait;

use crate::{
    actions::{live_selection_not_implemented, ActionBuilder},
    config::AppConfig,
    error::ApiError,
    models::requests::XudtTransferRequest,
    utils::validation,
};

pub struct XudtTransferBuilder;

#[async_trait]
impl ActionBuilder<XudtTransferRequest> for XudtTransferBuilder {
    fn action_id(&self) -> &'static str {
        "xudt.transfer"
    }

    fn validate_request(&self, request: &XudtTransferRequest) -> Result<(), ApiError> {
        validation::validate_network(&request.network)?;
        validation::validate_testnet_address("fromAddress", &request.from_address)?;
        validation::validate_testnet_address("toAddress", &request.to_address)?;
        validation::validate_hex_field(
            "xudtTypeScript.codeHash",
            &request.xudt_type_script.code_hash,
        )?;
        validation::validate_hex_field("xudtTypeScript.args", &request.xudt_type_script.args)?;
        match request.xudt_type_script.hash_type.as_str() {
            "type" | "data" | "data1" | "data2" => {}
            _ => {
                return Err(ApiError::bad_request(
                    "xudtTypeScript.hashType must be one of type, data, data1, or data2",
                ));
            }
        }
        validation::validate_positive_integer("amount", &request.amount)?;
        if let Some(fee_rate) = &request.fee_rate {
            validation::validate_positive_integer("feeRate", fee_rate)?;
        }
        Ok(())
    }

    async fn build(
        &self,
        request: XudtTransferRequest,
        config: &AppConfig,
    ) -> Result<crate::models::responses::BuildActionResponse, ApiError> {
        self.validate_request(&request)?;
        config.testnet_xudt_cell_dep()?;
        config.require_indexer_url()?;
        Err(live_selection_not_implemented())
    }

    fn response_summary(&self, request: &XudtTransferRequest) -> serde_json::Value {
        serde_json::json!({
            "fromAddress": request.from_address,
            "toAddress": request.to_address,
            "amount": request.amount,
            "xudtTypeScript": request.xudt_type_script,
            "estimatedFeeShannons": null,
        })
    }
}
