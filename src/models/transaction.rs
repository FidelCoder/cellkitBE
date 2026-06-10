use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSkeleton {
    pub version: String,
    pub cell_deps: Vec<serde_json::Value>,
    pub header_deps: Vec<serde_json::Value>,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    pub outputs_data: Vec<serde_json::Value>,
    pub witnesses: Vec<serde_json::Value>,
}

impl Default for TransactionSkeleton {
    fn default() -> Self {
        Self {
            version: "0x0".to_string(),
            cell_deps: Vec::new(),
            header_deps: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            outputs_data: Vec::new(),
            witnesses: Vec::new(),
        }
    }
}
