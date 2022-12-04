use anyhow::Result;
use ethers::{abi::AbiEncode, prelude::*};

pub fn token_ids_to_u256_abi_encoded(token_ids: &[String]) -> Result<Vec<String>> {
    token_ids
        .iter()
        .map(|token_id| {
            Ok::<std::string::String, anyhow::Error>(
                U256::from_str_radix(token_id, 10)?.encode_hex(),
            )
        })
        .collect()
}
