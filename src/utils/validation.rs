use crate::error::ApiError;

pub fn validate_network(network: &str) -> Result<(), ApiError> {
    if network == "testnet" {
        Ok(())
    } else {
        Err(ApiError::bad_request("network must be testnet for the MVP"))
    }
}

pub fn validate_testnet_address(field: &str, value: &str) -> Result<(), ApiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiError::bad_request(format!("{field} is required")));
    }
    if !trimmed.starts_with("ckt1") {
        return Err(ApiError::bad_request(format!(
            "{field} must be a CKB testnet address starting with ckt1"
        )));
    }
    if trimmed.len() < 12 {
        return Err(ApiError::bad_request(format!(
            "{field} is too short to be a valid CKB testnet address"
        )));
    }

    Ok(())
}

pub fn validate_positive_integer(field: &str, value: &str) -> Result<u64, ApiError> {
    let parsed = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ApiError::bad_request(format!("{field} must be a positive integer")))?;

    if parsed == 0 {
        return Err(ApiError::bad_request(format!(
            "{field} must be greater than 0"
        )));
    }

    Ok(parsed)
}

pub fn validate_hex_field(field: &str, value: &str) -> Result<(), ApiError> {
    if !value.starts_with("0x") {
        return Err(ApiError::bad_request(format!("{field} must start with 0x")));
    }
    let stripped = value.trim_start_matches("0x");
    if stripped.is_empty() {
        return Err(ApiError::bad_request(format!("{field} cannot be empty")));
    }
    if !stripped.len().is_multiple_of(2) {
        return Err(ApiError::bad_request(format!(
            "{field} must contain an even number of hex characters"
        )));
    }
    if !stripped.chars().all(|char| char.is_ascii_hexdigit()) {
        return Err(ApiError::bad_request(format!(
            "{field} contains non-hex characters"
        )));
    }

    Ok(())
}
