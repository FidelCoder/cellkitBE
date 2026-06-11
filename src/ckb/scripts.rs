use crate::{
    config::{AppConfig, CellDepConfig, Network},
    error::ApiError,
};

pub fn testnet_xudt_cell_dep(config: &AppConfig) -> Result<CellDepConfig, ApiError> {
    config.testnet_xudt_cell_dep()
}

pub fn get_secp256k1_cell_dep(
    config: &AppConfig,
    network: Network,
) -> Result<serde_json::Value, ApiError> {
    match network {
        Network::Testnet => testnet_secp256k1_cell_dep(config),
    }
}

fn testnet_secp256k1_cell_dep(config: &AppConfig) -> Result<serde_json::Value, ApiError> {
    let Some(tx_hash) = config.secp256k1.tx_hash.clone() else {
        return Err(missing_secp256k1_config());
    };
    let Some(index) = config.secp256k1.index.clone() else {
        return Err(missing_secp256k1_config());
    };
    let Some(dep_type) = config.secp256k1.dep_type.clone() else {
        return Err(missing_secp256k1_config());
    };

    if dep_type != "code" && dep_type != "dep_group" {
        return Err(ApiError::missing_config(
            "testnet secp256k1 cell dep config is missing",
        ));
    }

    Ok(serde_json::json!({
        "out_point": {
            "tx_hash": tx_hash,
            "index": index,
        },
        "dep_type": dep_type,
    }))
}

fn missing_secp256k1_config() -> ApiError {
    ApiError::missing_config("testnet secp256k1 cell dep config is missing")
}

#[cfg(test)]
mod tests {
    use super::get_secp256k1_cell_dep;
    use crate::config::{AppConfig, Network, Secp256k1Config, XudtConfig};

    fn base_config() -> AppConfig {
        AppConfig {
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
            network: Network::Testnet,
            ckb_rpc_url: None,
            ckb_indexer_url: None,
            default_fee_rate: 1000,
            xudt: XudtConfig::default(),
            secp256k1: Secp256k1Config::default(),
        }
    }

    #[test]
    fn missing_secp256k1_config_returns_clear_error() {
        let error = get_secp256k1_cell_dep(&base_config(), Network::Testnet).unwrap_err();
        assert_eq!(
            error.to_string(),
            "testnet secp256k1 cell dep config is missing"
        );
    }

    #[test]
    fn loads_testnet_secp256k1_cell_dep_from_config() {
        let mut config = base_config();
        config.secp256k1.tx_hash =
            Some("0x0000000000000000000000000000000000000000000000000000000000000001".to_string());
        config.secp256k1.index = Some("0x0".to_string());
        config.secp256k1.dep_type = Some("dep_group".to_string());

        let dep = get_secp256k1_cell_dep(&config, Network::Testnet).unwrap();
        assert_eq!(dep["dep_type"], "dep_group");
    }
}
