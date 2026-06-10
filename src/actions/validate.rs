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
    let outputs_data = tx.get("outputsData").and_then(|value| value.as_array());
    match (outputs, outputs_data) {
        (Some(outputs), Some(outputs_data)) if outputs.len() != outputs_data.len() => {
            errors.push(
                "transaction.outputs and transaction.outputsData lengths must match".to_string(),
            );
        }
        _ => {}
    }

    let cell_deps = tx.get("cellDeps").and_then(|value| value.as_array());
    let has_type_scripts = contains_key_named(tx, "type") || contains_key_named(tx, "typeScript");
    let action_needs_xudt_deps = request.action.as_deref() == Some("xudt.transfer");
    if (has_type_scripts || action_needs_xudt_deps)
        && matches!(cell_deps, Some(items) if items.is_empty())
    {
        errors.push("transaction.cellDeps must include required script deps for xUDT/type-script transactions".to_string());
    }

    let witnesses = tx.get("witnesses").and_then(|value| value.as_array());
    if let (Some(inputs), Some(witnesses)) = (inputs, witnesses) {
        if !inputs.is_empty() && witnesses.is_empty() {
            warnings.push(
                "transaction.witnesses is empty; wallet signing usually needs witness placeholders"
                    .to_string(),
            );
        }
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
                "outputs": [{}, {}],
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
}
