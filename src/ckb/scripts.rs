use crate::{config::AppConfig, error::ApiError};

pub fn testnet_xudt_cell_dep(config: &AppConfig) -> Result<crate::config::CellDepConfig, ApiError> {
    config.testnet_xudt_cell_dep()
}
