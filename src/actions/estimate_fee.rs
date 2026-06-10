use crate::{
    error::ApiError,
    models::{requests::EstimateFeeRequest, responses::EstimateFeeResponse},
    utils::{amount::shannons_to_ckb, fee::estimate_fee_shannons, validation},
};

pub fn estimate_fee(request: &EstimateFeeRequest) -> Result<EstimateFeeResponse, ApiError> {
    validation::validate_network(&request.network)?;
    let fee_rate = validation::validate_positive_integer("feeRate", &request.fee_rate)?;
    let serialized = serde_json::to_vec(&request.transaction)
        .map_err(|error| ApiError::Internal(format!("failed to serialize transaction: {error}")))?;
    let estimated_size_bytes = serialized.len();
    let estimated_fee = estimate_fee_shannons(estimated_size_bytes, fee_rate);

    Ok(EstimateFeeResponse {
        fee_rate: request.fee_rate.clone(),
        estimated_size_bytes,
        estimated_fee_shannons: estimated_fee.to_string(),
        estimated_fee_ckb: shannons_to_ckb(estimated_fee),
    })
}
