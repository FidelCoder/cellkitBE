use crate::error::ApiError;

pub async fn get_live_cells_for_lock(
    _indexer_url: &str,
    _lock_script: serde_json::Value,
    _limit: usize,
) -> Result<Vec<crate::ckb::cells::LiveCell>, ApiError> {
    Err(ApiError::not_implemented(
        "CKB indexer integration is not implemented yet. Cell selection requires live chain access.",
    ))
}
