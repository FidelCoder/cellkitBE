pub mod capacity_lock;
pub mod cell_consolidation;
pub mod ckb_transfer;
pub mod data_cell_create;
pub mod estimate_fee;
pub mod registry;
pub mod transactions;
pub mod validate;
pub mod xudt_transfer;

use async_trait::async_trait;

use crate::{config::AppConfig, error::ApiError, models::responses::BuildActionResponse};

#[async_trait]
pub trait ActionBuilder<R>: Send + Sync {
    fn action_id(&self) -> &'static str;
    fn validate_request(&self, request: &R) -> Result<(), ApiError>;
    async fn build(&self, request: R, config: &AppConfig) -> Result<BuildActionResponse, ApiError>;
    fn response_summary(&self, request: &R) -> serde_json::Value;
}

pub fn live_selection_not_implemented() -> ApiError {
    ApiError::not_implemented(
        "Live CKB cell selection is not implemented yet. This backend will not fake chain state.",
    )
}
