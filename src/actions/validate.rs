use crate::{
    error::ApiError,
    models::{requests::ValidateActionRequest, responses::ValidateActionResponse},
    utils::validation,
};

pub fn validate_transaction_shape(
    request: &ValidateActionRequest,
) -> Result<ValidateActionResponse, ApiError> {
    validation::validate_network(&request.network)?;

    let tx = &request.transaction;
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for field in [
        "version",
        "cellDeps",
        "headerDeps",
        "inputs",
        "outputs",
        "outputsData",
        "witnesses",
    ] {
        if tx.get(field).is_none() {
            errors.push(format!("transaction.{field} is required"));
        }
    }

    let inputs = tx.get("inputs").and_then(|value| value.as_array());
    if matches!(inputs, Some(items) if items.is_empty()) {
        errors.push("transaction.inputs must contain at least one input".to_string());
    }

    let outputs = tx.get("outputs").and_then(|value| value.as_array());
    if matches!(outputs, Some(items) if items.is_empty()) {
        errors.push("transaction.outputs must contain at least one output".to_string());
    }

    let outputs_data = tx.get("outputsData").and_then(|value| value.as_array());
    match (outputs, outputs_data) {
        (Some(outputs), Some(outputs_data)) if outputs.len() != outputs_data.len() => {
            errors.push(
                "transaction.outputs and transaction.outputsData lengths must match".to_string(),
            );
        }
        _ => {}
    }

    if let Some(outputs) = outputs {
        for (index, output) in outputs.iter().enumerate() {
            match output
                .get("capacity")
                .and_then(|capacity| capacity.as_str())
            {
                Some(capacity) if is_hex_quantity(capacity) => {}
                _ => errors.push(format!(
                    "transaction.outputs[{index}].capacity must be a valid hex quantity"
                )),
            }
        }
    }

    if let Some(outputs_data) = outputs_data {
        for (index, output_data) in outputs_data.iter().enumerate() {
            match output_data.as_str() {
                Some(output_data) if is_hex_data(output_data) => {}
                _ => errors.push(format!(
                    "transaction.outputsData[{index}] must be 0x-prefixed hex"
                )),
            }
        }
    }

    let cell_deps = tx.get("cellDeps").and_then(|value| value.as_array());
    let has_type_scripts = contains_key_named(tx, "type") || contains_key_named(tx, "typeScript");
    let action_needs_xudt_deps = request.action.as_deref() == Some("xudt.transfer");
    if (has_type_scripts || action_needs_xudt_deps)
        && matches!(cell_deps, Some(items) if items.is_empty())
    {
        errors.push("transaction.cellDeps must include required script deps for xUDT/type-script transactions".to_string());
    }

    if request.action.as_deref() == Some("ckb.transfer") {
        if matches!(cell_deps, Some(items) if items.is_empty()) {
            errors.push(
                "transaction.cellDeps must include secp256k1 dep for CKB transfer".to_string(),
            );
        }
        if let Some(cell_deps) = cell_deps {
            let has_secp_like_dep = cell_deps.iter().any(|dep| {
                dep.get("out_point").is_some()
                    && dep
                        .get("dep_type")
                        .and_then(|value| value.as_str())
                        .is_some_and(|dep_type| dep_type == "code" || dep_type == "dep_group")
            });
            if !cell_deps.is_empty() && !has_secp_like_dep {
                errors.push(
                    "transaction.cellDeps should include secp256k1 dep for default lock"
                        .to_string(),
                );
            }
        }
    }

    let witnesses = tx.get("witnesses").and_then(|value| value.as_array());
    if matches!(witnesses, Some(items) if items.is_empty()) {
        warnings.push(
            "transaction.witnesses is empty; wallet signing usually needs witness placeholders"
                .to_string(),
        );
    }
    if request.action.as_deref() == Some("ckb.transfer")
        && matches!(witnesses, Some(items) if items.is_empty())
    {
        errors.push("transaction.witnesses must contain at least one placeholder".to_string());
    }

    Ok(ValidateActionResponse {
        valid: errors.is_empty(),
        errors,
        warnings,
    })
}

fn contains_key_named(value: &serde_json::Value, key: &str) -> bool {
    match value {
        serde_json::Value::Object(object) => object
            .iter()
            .any(|(current_key, nested)| current_key == key || contains_key_named(nested, key)),
        serde_json::Value::Array(items) => items.iter().any(|item| contains_key_named(item, key)),
        _ => false,
    }
}

fn is_hex_quantity(value: &str) -> bool {
    let Some(stripped) = value.strip_prefix("0x") else {
        return false;
    };
    !stripped.is_empty() && stripped.chars().all(|char| char.is_ascii_hexdigit())
}

fn is_hex_data(value: &str) -> bool {
    let Some(stripped) = value.strip_prefix("0x") else {
        return false;
    };
    stripped.len().is_multiple_of(2) && stripped.chars().all(|char| char.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::validate_transaction_shape;
    use crate::models::requests::ValidateActionRequest;

    #[test]
    fn catches_outputs_outputs_data_mismatch() {
        let response = validate_transaction_shape(&ValidateActionRequest {
            network: "testnet".to_string(),
            action: None,
            transaction: serde_json::json!({
                "version": "0x0",
                "cellDeps": [{}],
                "headerDeps": [],
                "inputs": [{}],
                "outputs": [{"capacity":"0x1"}, {"capacity":"0x2"}],
                "outputsData": ["0x"],
                "witnesses": [{}]
            }),
        })
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("lengths must match")));
    }

    #[test]
    fn catches_missing_inputs() {
        let response = validate_transaction_shape(&ValidateActionRequest {
            network: "testnet".to_string(),
            action: None,
            transaction: serde_json::json!({
                "version": "0x0",
                "cellDeps": [{}],
                "headerDeps": [],
                "inputs": [],
                "outputs": [],
                "outputsData": [],
                "witnesses": []
            }),
        })
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("inputs must contain")));
    }

    #[test]
    fn catches_missing_cell_deps_for_xudt() {
        let response = validate_transaction_shape(&ValidateActionRequest {
            network: "testnet".to_string(),
            action: Some("xudt.transfer".to_string()),
            transaction: serde_json::json!({
                "version": "0x0",
                "cellDeps": [],
                "headerDeps": [],
                "inputs": [{}],
                "outputs": [{
                    "capacity": "0x1",
                    "type": {
                        "codeHash": "0x00",
                        "hashType": "type",
                        "args": "0x00"
                    }
                }],
                "outputsData": ["0x"],
                "witnesses": [{}]
            }),
        })
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("cellDeps")));
    }

    #[test]
    fn ckb_transfer_validation_requires_output_and_witness() {
        let response = validate_transaction_shape(&ValidateActionRequest {
            network: "testnet".to_string(),
            action: Some("ckb.transfer".to_string()),
            transaction: serde_json::json!({
                "version": "0x0",
                "cellDeps": [{"out_point": {}, "dep_type": "dep_group"}],
                "headerDeps": [],
                "inputs": [{}],
                "outputs": [],
                "outputsData": [],
                "witnesses": []
            }),
        })
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("outputs")));
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("witnesses")));
    }

    #[test]
    fn ckb_transfer_validation_catches_invalid_hex_fields() {
        let response = validate_transaction_shape(&ValidateActionRequest {
            network: "testnet".to_string(),
            action: Some("ckb.transfer".to_string()),
            transaction: serde_json::json!({
                "version": "0x0",
                "cellDeps": [{"out_point": {}, "dep_type": "dep_group"}],
                "headerDeps": [],
                "inputs": [{}],
                "outputs": [{"capacity": "10"}],
                "outputsData": ["abc"],
                "witnesses": ["0x"]
            }),
        })
        .unwrap();

        assert!(!response.valid);
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("capacity")));
        assert!(response
            .errors
            .iter()
            .any(|error| error.contains("outputsData")));
    }
}
