use ckb_types::{bytes::Bytes, packed::WitnessArgs, prelude::*};

use crate::{ckb::cells::CellSelection, error::ApiError, models::transaction::TransactionSkeleton};

pub fn empty_skeleton() -> TransactionSkeleton {
    TransactionSkeleton::default()
}

pub fn build_ckb_transfer_skeleton(
    selection: &CellSelection,
    sender_lock: serde_json::Value,
    receiver_lock: serde_json::Value,
    secp256k1_cell_dep: serde_json::Value,
) -> TransactionSkeleton {
    let mut outputs = vec![ordinary_output_json(
        selection.target_output_shannons,
        receiver_lock,
    )];
    let mut outputs_data = vec![serde_json::json!("0x")];

    if selection.change_shannons > 0 {
        outputs.push(ordinary_output_json(selection.change_shannons, sender_lock));
        outputs_data.push(serde_json::json!("0x"));
    }

    TransactionSkeleton {
        version: "0x0".to_string(),
        cell_deps: vec![secp256k1_cell_dep],
        header_deps: Vec::new(),
        inputs: selection
            .selected_cells
            .iter()
            .map(|cell| cell.input_json())
            .collect(),
        outputs,
        outputs_data,
        witnesses: vec![serde_json::json!(secp256k1_placeholder_witness_hex())],
    }
}

pub fn estimate_transaction_size_bytes(
    transaction: &TransactionSkeleton,
) -> Result<usize, ApiError> {
    serde_json::to_vec(transaction)
        .map(|bytes| bytes.len())
        .map_err(|error| {
            ApiError::Internal(format!("failed to estimate transaction size: {error}"))
        })
}

pub fn secp256k1_placeholder_witness_hex() -> String {
    let placeholder_witness = WitnessArgs::new_builder()
        .lock(Some(Bytes::from(vec![0u8; 65])).pack())
        .build();
    bytes_to_hex(placeholder_witness.as_bytes().as_ref())
}

fn ordinary_output_json(capacity_shannons: u64, lock: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "capacity": hex_quantity(capacity_shannons),
        "lock": lock,
        "type": null,
    })
}

pub fn hex_quantity(value: u64) -> String {
    format!("0x{value:x}")
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(2 + bytes.len() * 2);
    output.push_str("0x");
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{build_ckb_transfer_skeleton, estimate_transaction_size_bytes, hex_quantity};
    use crate::ckb::cells::{CellSelection, LiveCell, LiveCellOutPoint};

    fn selection() -> CellSelection {
        CellSelection {
            selected_cells: vec![LiveCell {
                out_point: LiveCellOutPoint {
                    tx_hash: "0x0000000000000000000000000000000000000000000000000000000000000001"
                        .to_string(),
                    index: "0x0".to_string(),
                },
                capacity_shannons: 10_000,
                lock_script: serde_json::json!({}),
                type_script: None,
                output_data: "0x".to_string(),
                block_number: None,
                tx_index: None,
            }],
            total_input_shannons: 10_000,
            target_output_shannons: 5_000,
            estimated_fee_shannons: 1_000,
            change_shannons: 4_000,
            tiny_change_added_to_fee: false,
        }
    }

    #[test]
    fn builds_skeleton_with_matching_outputs_data() {
        let tx = build_ckb_transfer_skeleton(
            &selection(),
            serde_json::json!({"args":"0xsender"}),
            serde_json::json!({"args":"0xreceiver"}),
            serde_json::json!({"dep_type":"dep_group"}),
        );
        assert_eq!(tx.inputs.len(), 1);
        assert_eq!(tx.outputs.len(), tx.outputs_data.len());
        assert_eq!(tx.witnesses.len(), 1);
        assert_eq!(tx.outputs[0]["capacity"], hex_quantity(5_000));
    }

    #[test]
    fn estimates_transaction_size() {
        let tx = build_ckb_transfer_skeleton(
            &selection(),
            serde_json::json!({"args":"0xsender"}),
            serde_json::json!({"args":"0xreceiver"}),
            serde_json::json!({"dep_type":"dep_group"}),
        );
        assert!(estimate_transaction_size_bytes(&tx).unwrap() > 0);
    }
}
