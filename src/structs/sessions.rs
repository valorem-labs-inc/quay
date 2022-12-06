use axum_sessions::async_session::Session;
use ethers::abi::ethereum_types::Signature;
use ethers::abi::AbiEncode;
use ethers::prelude::Address;
use serde::{Deserialize, Serialize};
use siwe::Message;

// EIP-4361 based session

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedMessage {
    pub signature: Signature,
    pub message: Message,
}
