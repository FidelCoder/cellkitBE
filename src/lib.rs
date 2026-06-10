pub mod actions;
pub mod ckb;
pub mod config;
pub mod error;
pub mod models;
pub mod routes;
pub mod utils;

use axum::Router;
use config::AppConfig;

pub fn app(config: AppConfig) -> Router {
    routes::router(config)
}
