use crate::error::ApiError;

pub const SHANNONS_PER_CKB: u64 = 100_000_000;
pub const MIN_ORDINARY_CELL_CAPACITY_SHANNONS: u64 = 61 * SHANNONS_PER_CKB;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct LiveCellOutPoint {
    pub tx_hash: String,
    pub index: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LiveCell {
    pub out_point: LiveCellOutPoint,
    pub capacity_shannons: u64,
    pub lock_script: serde_json::Value,
    pub type_script: Option<serde_json::Value>,
    pub output_data: String,
    pub block_number: Option<u64>,
    pub tx_index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellSelection {
    pub selected_cells: Vec<LiveCell>,
    pub total_input_shannons: u64,
    pub target_output_shannons: u64,
    pub estimated_fee_shannons: u64,
    pub change_shannons: u64,
    pub tiny_change_added_to_fee: bool,
}

impl LiveCell {
    pub fn is_ordinary_ckb_cell(&self) -> bool {
        self.type_script.is_none() && is_empty_data(&self.output_data)
    }

    pub fn occupied_capacity_shannons(&self) -> u64 {
        MIN_ORDINARY_CELL_CAPACITY_SHANNONS
    }

    pub fn spendable_capacity_shannons(&self) -> Option<u64> {
        self.is_ordinary_ckb_cell()
            .then_some(self.capacity_shannons)
    }

    pub fn input_json(&self) -> serde_json::Value {
        serde_json::json!({
            "since": "0x0",
            "previous_output": {
                "tx_hash": self.out_point.tx_hash,
                "index": self.out_point.index,
            }
        })
    }
}

impl CellSelection {
    pub fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        if self.tiny_change_added_to_fee {
            warnings.push(
                "change below minimum cell capacity was added to transaction fee".to_string(),
            );
        }
        if self.selected_cells.len() > 20 {
            warnings.push("large input count may increase transaction fee".to_string());
        }
        warnings
    }
}

pub fn select_cells_for_capacity(
    cells: &[LiveCell],
    target_amount_shannons: u64,
    estimated_fee_shannons: u64,
) -> Result<CellSelection, ApiError> {
    let required = target_amount_shannons
        .checked_add(estimated_fee_shannons)
        .ok_or_else(|| ApiError::bad_request("required capacity is too large"))?;

    let mut spendable_cells: Vec<LiveCell> = cells
        .iter()
        .filter(|cell| cell.is_ordinary_ckb_cell())
        .cloned()
        .collect();

    spendable_cells.sort_by(|left, right| {
        left.capacity_shannons
            .cmp(&right.capacity_shannons)
            .then_with(|| left.out_point.tx_hash.cmp(&right.out_point.tx_hash))
            .then_with(|| left.out_point.index.cmp(&right.out_point.index))
    });

    let available = spendable_cells.iter().try_fold(0u64, |sum, cell| {
        sum.checked_add(cell.capacity_shannons)
            .ok_or_else(|| ApiError::bad_request("available capacity is too large"))
    })?;

    let mut selected_cells = Vec::new();
    let mut total_input_shannons = 0u64;
    for cell in spendable_cells {
        total_input_shannons = total_input_shannons
            .checked_add(cell.capacity_shannons)
            .ok_or_else(|| ApiError::bad_request("selected capacity is too large"))?;
        selected_cells.push(cell);
        if total_input_shannons >= required {
            break;
        }
    }

    if total_input_shannons < required {
        return Err(ApiError::bad_request(format!(
            "insufficient capacity: required {required} shannons, available {available} shannons"
        )));
    }

    let raw_change = total_input_shannons - required;
    let (change_shannons, estimated_fee_shannons, tiny_change_added_to_fee) =
        if raw_change > 0 && raw_change < MIN_ORDINARY_CELL_CAPACITY_SHANNONS {
            (
                0,
                estimated_fee_shannons
                    .checked_add(raw_change)
                    .ok_or_else(|| ApiError::bad_request("estimated fee is too large"))?,
                true,
            )
        } else {
            (raw_change, estimated_fee_shannons, false)
        };

    Ok(CellSelection {
        selected_cells,
        total_input_shannons,
        target_output_shannons: target_amount_shannons,
        estimated_fee_shannons,
        change_shannons,
        tiny_change_added_to_fee,
    })
}

pub fn parse_hex_u64_quantity(value: &str) -> Result<u64, ApiError> {
    let stripped = value
        .strip_prefix("0x")
        .ok_or_else(|| ApiError::bad_request("hex quantity must start with 0x"))?;
    if stripped.is_empty() {
        return Err(ApiError::bad_request("hex quantity cannot be empty"));
    }
    u64::from_str_radix(stripped, 16)
        .map_err(|_| ApiError::bad_request("hex quantity is not a valid u64"))
}

pub fn is_empty_data(value: &str) -> bool {
    value.trim().is_empty() || value.trim() == "0x"
}

#[cfg(test)]
mod tests {
    use super::{
        is_empty_data, select_cells_for_capacity, LiveCell, LiveCellOutPoint,
        MIN_ORDINARY_CELL_CAPACITY_SHANNONS,
    };

    fn cell(tx_hash: &str, index: &str, capacity_shannons: u64) -> LiveCell {
        LiveCell {
            out_point: LiveCellOutPoint {
                tx_hash: tx_hash.to_string(),
                index: index.to_string(),
            },
            capacity_shannons,
            lock_script: serde_json::json!({"code_hash":"0x00","hash_type":"type","args":"0x"}),
            type_script: None,
            output_data: "0x".to_string(),
            block_number: Some(1),
            tx_index: Some(0),
        }
    }

    #[test]
    fn ordinary_cell_returns_true() {
        assert!(cell("0x01", "0x0", 100).is_ordinary_ckb_cell());
    }

    #[test]
    fn type_script_cell_returns_false() {
        let mut live_cell = cell("0x01", "0x0", 100);
        live_cell.type_script = Some(serde_json::json!({"code_hash":"0x01"}));
        assert!(!live_cell.is_ordinary_ckb_cell());
    }

    #[test]
    fn non_empty_data_cell_returns_false_for_transfer_spend() {
        let mut live_cell = cell("0x01", "0x0", 100);
        live_cell.output_data = "0x01".to_string();
        assert!(!live_cell.is_ordinary_ckb_cell());
        assert!(is_empty_data("0x"));
    }

    #[test]
    fn selects_enough_cells_deterministically() {
        let cells = vec![
            cell("0x03", "0x0", 150),
            cell("0x01", "0x0", 50),
            cell("0x02", "0x0", 75),
        ];
        let selection = select_cells_for_capacity(&cells, 100, 10).unwrap();
        let hashes: Vec<_> = selection
            .selected_cells
            .iter()
            .map(|cell| cell.out_point.tx_hash.as_str())
            .collect();
        assert_eq!(hashes, vec!["0x01", "0x02"]);
        assert_eq!(selection.total_input_shannons, 125);
    }

    #[test]
    fn returns_insufficient_capacity_error() {
        let cells = vec![cell("0x01", "0x0", 50)];
        let error = select_cells_for_capacity(&cells, 100, 10).unwrap_err();
        assert!(error.to_string().contains("insufficient capacity"));
    }

    #[test]
    fn creates_valid_change_when_change_is_above_minimum() {
        let cells = vec![cell("0x01", "0x0", MIN_ORDINARY_CELL_CAPACITY_SHANNONS * 3)];
        let selection =
            select_cells_for_capacity(&cells, MIN_ORDINARY_CELL_CAPACITY_SHANNONS, 1_000).unwrap();
        assert!(selection.change_shannons >= MIN_ORDINARY_CELL_CAPACITY_SHANNONS);
        assert!(!selection.tiny_change_added_to_fee);
    }

    #[test]
    fn adds_tiny_change_to_fee_when_below_minimum() {
        let cells = vec![cell(
            "0x01",
            "0x0",
            MIN_ORDINARY_CELL_CAPACITY_SHANNONS + 2_000,
        )];
        let selection =
            select_cells_for_capacity(&cells, MIN_ORDINARY_CELL_CAPACITY_SHANNONS, 1_000).unwrap();
        assert_eq!(selection.change_shannons, 0);
        assert_eq!(selection.estimated_fee_shannons, 2_000);
        assert!(selection.tiny_change_added_to_fee);
    }
}
