use crate::{error::ApiError, utils::validation};

pub fn validate_testnet_address(field: &str, value: &str) -> Result<(), ApiError> {
    validation::validate_testnet_address(field, value)
}
