pub mod actions;
pub mod health;
pub mod transactions;

use std::sync::Arc;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

impl From<AppConfig> for AppState {
    fn from(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

pub fn router(config: AppConfig) -> Router {
    let state = AppState::from(config);

    Router::new()
        .merge(health::router())
        .merge(actions::router())
        .merge(transactions::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
