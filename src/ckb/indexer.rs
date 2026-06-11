use serde::Deserialize;

use crate::{
    ckb::{
        address::script_to_json,
        cells::{is_empty_data, parse_hex_u64_quantity, LiveCell, LiveCellOutPoint},
    },
    error::ApiError,
};

const DEFAULT_PAGE_LIMIT: u32 = 100;
const MAX_PAGES: usize = 100;

pub async fn get_live_cells_by_lock(
    indexer_url: &str,
    lock_script: ckb_types::packed::Script,
    limit: u32,
) -> Result<Vec<LiveCell>, ApiError> {
    let client = reqwest::Client::new();
    let page_limit = if limit == 0 {
        DEFAULT_PAGE_LIMIT
    } else {
        limit
    };
    let mut cursor: Option<String> = None;
    let mut all_cells = Vec::new();

    for _ in 0..MAX_PAGES {
        if all_cells.len() >= page_limit as usize {
            break;
        }

        let remaining = page_limit.saturating_sub(all_cells.len() as u32).max(1);
        let requested = remaining.min(DEFAULT_PAGE_LIMIT);
        let response = request_cells_page(
            &client,
            indexer_url,
            script_to_json(&lock_script),
            requested,
            cursor.as_deref(),
        )
        .await?;

        if response.objects.is_empty() {
            break;
        }

        let next_cursor = response.last_cursor.clone();
        for cell in response.objects {
            all_cells.push(cell.try_into_live_cell()?);
            if all_cells.len() >= page_limit as usize {
                break;
            }
        }

        if cursor.as_deref() == Some(next_cursor.as_str()) || is_empty_data(&next_cursor) {
            break;
        }
        cursor = Some(next_cursor);
    }

    Ok(all_cells)
}

pub async fn get_ordinary_live_cells_by_lock(
    indexer_url: &str,
    lock_script: ckb_types::packed::Script,
    limit: u32,
) -> Result<Vec<LiveCell>, ApiError> {
    let cells = get_live_cells_by_lock(indexer_url, lock_script, limit).await?;
    Ok(cells
        .into_iter()
        .filter(|cell| cell.is_ordinary_ckb_cell())
        .collect())
}

pub async fn get_live_cells_for_lock(
    indexer_url: &str,
    lock_script: serde_json::Value,
    limit: usize,
) -> Result<Vec<LiveCell>, ApiError> {
    let client = reqwest::Client::new();
    let response = request_cells_page(
        &client,
        indexer_url,
        lock_script,
        limit.try_into().unwrap_or(DEFAULT_PAGE_LIMIT),
        None,
    )
    .await?;
    response
        .objects
        .into_iter()
        .map(IndexerCellJson::try_into_live_cell)
        .collect()
}

async fn request_cells_page(
    client: &reqwest::Client,
    indexer_url: &str,
    lock_script: serde_json::Value,
    limit: u32,
    cursor: Option<&str>,
) -> Result<CellsPageJson, ApiError> {
    let search_key = serde_json::json!({
        "script": lock_script,
        "script_type": "lock",
        "script_search_mode": "exact",
        "with_data": true,
        "group_by_transaction": false,
    });

    let mut params = vec![
        search_key,
        serde_json::json!("asc"),
        serde_json::json!(format!("0x{limit:x}")),
    ];
    if let Some(cursor) = cursor {
        params.push(serde_json::json!(cursor));
    }

    let body = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "get_cells",
        "params": params,
    });

    let response = client
        .post(indexer_url)
        .json(&body)
        .send()
        .await
        .map_err(|_| ApiError::bad_request("CKB indexer is unreachable"))?;

    if !response.status().is_success() {
        return Err(ApiError::bad_request("CKB indexer is unreachable"));
    }

    let rpc_response: RpcResponseJson = response
        .json()
        .await
        .map_err(|_| ApiError::bad_request("invalid CKB indexer response"))?;

    if rpc_response.error.is_some() {
        return Err(ApiError::bad_request("invalid CKB indexer response"));
    }

    rpc_response
        .result
        .ok_or_else(|| ApiError::bad_request("invalid CKB indexer response"))
}

#[derive(Debug, Deserialize)]
struct RpcResponseJson {
    result: Option<CellsPageJson>,
    error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct CellsPageJson {
    objects: Vec<IndexerCellJson>,
    last_cursor: String,
}

#[derive(Debug, Deserialize)]
struct IndexerCellJson {
    output: IndexerCellOutputJson,
    output_data: Option<String>,
    out_point: LiveCellOutPoint,
    block_number: Option<String>,
    tx_index: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IndexerCellOutputJson {
    capacity: String,
    lock: serde_json::Value,
    #[serde(rename = "type")]
    type_script: Option<serde_json::Value>,
}

impl IndexerCellJson {
    fn try_into_live_cell(self) -> Result<LiveCell, ApiError> {
        let capacity_shannons = parse_hex_u64_quantity(&self.output.capacity)
            .map_err(|_| ApiError::bad_request("invalid CKB indexer response"))?;
        let block_number = self
            .block_number
            .as_deref()
            .map(parse_hex_u64_quantity)
            .transpose()
            .map_err(|_| ApiError::bad_request("invalid CKB indexer response"))?;
        let tx_index = self
            .tx_index
            .as_deref()
            .map(parse_hex_u64_quantity)
            .transpose()
            .map_err(|_| ApiError::bad_request("invalid CKB indexer response"))?
            .map(|value| value as u32);

        Ok(LiveCell {
            out_point: self.out_point,
            capacity_shannons,
            lock_script: self.output.lock,
            type_script: self.output.type_script,
            output_data: self.output_data.unwrap_or_else(|| "0x".to_string()),
            block_number,
            tx_index,
        })
    }
}
