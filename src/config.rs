use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Testnet,
}

impl Network {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Testnet => "testnet",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub network: Network,
    pub ckb_rpc_url: Option<String>,
    pub ckb_indexer_url: Option<String>,
    pub default_fee_rate: u64,
    pub xudt: XudtConfig,
    pub secp256k1: Secp256k1Config,
}

#[derive(Debug, Clone, Default)]
pub struct XudtConfig {
    pub code_hash: Option<String>,
    pub hash_type: Option<String>,
    pub tx_hash: Option<String>,
    pub index: Option<String>,
    pub dep_type: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Secp256k1Config {
    pub code_hash: Option<String>,
    pub hash_type: Option<String>,
    pub tx_hash: Option<String>,
    pub index: Option<String>,
    pub dep_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellDepConfig {
    pub code_hash: String,
    pub hash_type: String,
    pub out_point: serde_json::Value,
    pub dep_type: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let network = match env::var("CKB_NETWORK")
            .unwrap_or_else(|_| "testnet".to_string())
            .as_str()
        {
            "testnet" => Network::Testnet,
            _ => Network::Testnet,
        };

        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(8080),
            network,
            ckb_rpc_url: optional_env("CKB_RPC_URL"),
            ckb_indexer_url: optional_env("CKB_INDEXER_URL"),
            default_fee_rate: env::var("DEFAULT_FEE_RATE")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(1000),
            xudt: XudtConfig {
                code_hash: optional_env("TESTNET_XUDT_CODE_HASH"),
                hash_type: optional_env("TESTNET_XUDT_HASH_TYPE")
                    .or_else(|| Some("type".to_string())),
                tx_hash: optional_env("TESTNET_XUDT_TX_HASH"),
                index: optional_env("TESTNET_XUDT_INDEX"),
                dep_type: optional_env("TESTNET_XUDT_DEP_TYPE")
                    .or_else(|| Some("code".to_string())),
            },
            secp256k1: Secp256k1Config {
                code_hash: optional_env("TESTNET_SECP256K1_CODE_HASH"),
                hash_type: optional_env("TESTNET_SECP256K1_HASH_TYPE")
                    .or_else(|| Some("type".to_string())),
                tx_hash: optional_env("TESTNET_SECP256K1_TX_HASH"),
                index: optional_env("TESTNET_SECP256K1_INDEX"),
                dep_type: optional_env("TESTNET_SECP256K1_DEP_TYPE")
                    .or_else(|| Some("dep_group".to_string())),
            },
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn require_indexer_url(&self) -> Result<&str, crate::error::ApiError> {
        self.ckb_indexer_url.as_deref().ok_or_else(|| {
            crate::error::ApiError::missing_config(
                "CKB indexer is not configured. Set CKB_INDEXER_URL.",
            )
        })
    }

    pub fn testnet_xudt_cell_dep(&self) -> Result<CellDepConfig, crate::error::ApiError> {
        let Some(code_hash) = self.xudt.code_hash.clone() else {
            return Err(crate::error::ApiError::missing_config(
                "xUDT cell dep is not configured for this network.",
            ));
        };
        let Some(tx_hash) = self.xudt.tx_hash.clone() else {
            return Err(crate::error::ApiError::missing_config(
                "xUDT cell dep is not configured for this network.",
            ));
        };
        let Some(index) = self.xudt.index.clone() else {
            return Err(crate::error::ApiError::missing_config(
                "xUDT cell dep is not configured for this network.",
            ));
        };

        Ok(CellDepConfig {
            code_hash,
            hash_type: self
                .xudt
                .hash_type
                .clone()
                .unwrap_or_else(|| "type".to_string()),
            out_point: serde_json::json!({
                "tx_hash": tx_hash,
                "index": index,
            }),
            dep_type: self
                .xudt
                .dep_type
                .clone()
                .unwrap_or_else(|| "code".to_string()),
        })
    }
}

fn optional_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
