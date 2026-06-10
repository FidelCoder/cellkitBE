#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveCell {
    pub out_point: serde_json::Value,
    pub output: serde_json::Value,
    pub output_data: String,
}
