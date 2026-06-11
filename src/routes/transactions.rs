use axum::{extract::State, routing::post, Json, Router};

use crate::{
    actions::transactions,
    error::ApiError,
    models::{
        requests::SignedTransactionRequest,
        responses::{
            BroadcastTransactionResponse, DryRunTransactionResponse,
            ValidateSignedTransactionResponse,
        },
    },
};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/transactions/validate-signed",
            post(validate_signed_transaction),
        )
        .route("/api/transactions/dry-run", post(dry_run_transaction))
        .route("/api/transactions/broadcast", post(broadcast_transaction))
}

async fn validate_signed_transaction(
    Json(request): Json<SignedTransactionRequest>,
) -> Result<Json<ValidateSignedTransactionResponse>, ApiError> {
    transactions::validate_signed_transaction(&request).map(Json)
}

async fn dry_run_transaction(
    State(state): State<AppState>,
    Json(request): Json<SignedTransactionRequest>,
) -> Result<Json<DryRunTransactionResponse>, ApiError> {
    transactions::dry_run_signed_transaction(request, &state.config)
        .await
        .map(Json)
}

async fn broadcast_transaction(
    State(state): State<AppState>,
    Json(request): Json<SignedTransactionRequest>,
) -> Result<Json<BroadcastTransactionResponse>, ApiError> {
    transactions::broadcast_signed_transaction(request, &state.config)
        .await
        .map(Json)
}
