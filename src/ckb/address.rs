use std::str::FromStr;

use ckb_jsonrpc_types::Script as JsonScript;
use ckb_sdk::{Address, NetworkType};
use ckb_types::packed::Script;

use crate::{config::Network, error::ApiError};

pub fn parse_address_to_script(
    address: &str,
    expected_network: Network,
) -> Result<Script, ApiError> {
    let parsed = parse_address(address)?;
    ensure_network(&parsed, expected_network)?;
    Ok((&parsed).into())
}

pub fn validate_address_network(address: &str, expected_network: Network) -> Result<(), ApiError> {
    let parsed = parse_address(address)?;
    ensure_network(&parsed, expected_network)
}

pub fn script_to_json(script: &Script) -> serde_json::Value {
    serde_json::to_value(JsonScript::from(script.clone()))
        .expect("CKB script JSON serialization should not fail")
}

fn parse_address(address: &str) -> Result<Address, ApiError> {
    Address::from_str(address.trim()).map_err(|_| ApiError::bad_request("invalid CKB address"))
}

fn ensure_network(address: &Address, expected_network: Network) -> Result<(), ApiError> {
    let expected = match expected_network {
        Network::Testnet => NetworkType::Testnet,
    };

    if address.network() != expected {
        return Err(ApiError::bad_request(
            "address network does not match requested network",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_address_to_script, validate_address_network};
    use crate::config::Network;
    use ckb_sdk::{Address, AddressPayload, NetworkType};
    use ckb_types::{h160, prelude::Unpack, H160};

    fn testnet_address() -> String {
        let hash: H160 = h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64");
        let payload = AddressPayload::from_pubkey_hash(hash);
        Address::new(NetworkType::Testnet, payload, false).to_string()
    }

    #[test]
    fn rejects_mainnet_address_for_testnet_request() {
        let hash: H160 = h160!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64");
        let payload = AddressPayload::from_pubkey_hash(hash);
        let mainnet = Address::new(NetworkType::Mainnet, payload, false).to_string();

        let error = validate_address_network(&mainnet, Network::Testnet).unwrap_err();
        assert!(error
            .to_string()
            .contains("address network does not match requested network"));
    }

    #[test]
    fn rejects_invalid_address() {
        let error = validate_address_network("not-a-ckb-address", Network::Testnet).unwrap_err();
        assert!(error.to_string().contains("invalid CKB address"));
    }

    #[test]
    fn parses_valid_testnet_address() {
        let address = testnet_address();
        let script = parse_address_to_script(&address, Network::Testnet).unwrap();
        let args: Vec<u8> = script.args().unpack();
        assert_eq!(args.len(), 20);
    }
}
