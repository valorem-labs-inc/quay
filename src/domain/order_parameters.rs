use ethers::types::{Address, Bytes, Selector, U256, U64};

struct OrderParameters {
    offerer: Address,
    zone: Address,
    zone_hash: Bytes,
    start_time: U256,
    end_time: U256,
    order_type: U64,
    salt: U256,
    conduit_key: Bytes,
    nonce: U64,
}
