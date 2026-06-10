use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use crate::{
    actions::{
        capacity_lock::CapacityLockBuilder, cell_consolidation::CellConsolidationBuilder,
        ckb_transfer::CkbTransferBuilder, data_cell_create::DataCellCreateBuilder, estimate_fee,
        registry, validate, xudt_transfer::XudtTransferBuilder, ActionBuilder,
    },
    error::ApiError,
    models::{
        requests::{
            CapacityLockRequest, CellConsolidationRequest, CkbTransferRequest,
            DataCellCreateRequest, EstimateFeeRequest, ValidateActionRequest, XudtTransferRequest,
        },
        responses::{
            ActionsResponse, BuildActionResponse, EstimateFeeResponse, ValidateActionResponse,
        },
    },
};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/actions", get(list_actions))
        .route("/api/actions/validate", post(validate_action))
        .route("/api/actions/estimate-fee", post(estimate_action_fee))
        .route("/api/actions/ckb-transfer/build", post(build_ckb_transfer))
        .route(
            "/api/actions/xudt-transfer/build",
            post(build_xudt_transfer),
        )
        .route(
            "/api/actions/cell-consolidation/build",
            post(build_cell_consolidation),
        )
        .route(
            "/api/actions/capacity-lock/build",
            post(build_capacity_lock),
        )
        .route(
            "/api/actions/data-cell-create/build",
            post(build_data_cell_create),
        )
}

async fn list_actions() -> Json<ActionsResponse> {
    Json(ActionsResponse {
        actions: registry::mvp_actions(),
    })
}

async fn build_ckb_transfer(
    State(state): State<AppState>,
    Json(request): Json<CkbTransferRequest>,
) -> Result<Json<BuildActionResponse>, ApiError> {
    CkbTransferBuilder
        .build(request, &state.config)
        .await
        .map(Json)
}

async fn build_xudt_transfer(
    State(state): State<AppState>,
    Json(request): Json<XudtTransferRequest>,
) -> Result<Json<BuildActionResponse>, ApiError> {
    XudtTransferBuilder
        .build(request, &state.config)
        .await
        .map(Json)
}

async fn build_cell_consolidation(
    State(state): State<AppState>,
    Json(request): Json<CellConsolidationRequest>,
) -> Result<Json<BuildActionResponse>, ApiError> {
    CellConsolidationBuilder
        .build(request, &state.config)
        .await
        .map(Json)
}

async fn build_capacity_lock(
    State(state): State<AppState>,
    Json(request): Json<CapacityLockRequest>,
) -> Result<Json<BuildActionResponse>, ApiError> {
    CapacityLockBuilder
        .build(request, &state.config)
        .await
        .map(Json)
}

async fn build_data_cell_create(
    State(state): State<AppState>,
    Json(request): Json<DataCellCreateRequest>,
) -> Result<Json<BuildActionResponse>, ApiError> {
    DataCellCreateBuilder
        .build(request, &state.config)
        .await
        .map(Json)
}

async fn validate_action(
    Json(request): Json<ValidateActionRequest>,
) -> Result<Json<ValidateActionResponse>, ApiError> {
    validate::validate_transaction_shape(&request).map(Json)
}

async fn estimate_action_fee(
    Json(request): Json<EstimateFeeRequest>,
) -> Result<Json<EstimateFeeResponse>, ApiError> {
    estimate_fee::estimate_fee(&request).map(Json)
}
