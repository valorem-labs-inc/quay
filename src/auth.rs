use std::time::{SystemTime, UNIX_EPOCH};

use axum::response::IntoResponse;
use axum_sessions::extractors::ReadableSession;
use ethers::abi::ethereum_types::Signature;

use http::StatusCode;
use serde::{Deserialize, Serialize};
use siwe::Message;

pub const NONCE_KEY: &str = "nonce";
pub const EXPIRATION_TIME_KEY: &str = "expirationTime";
pub const USER_ADDRESS_KEY: &str = "userAddress";

pub fn unix_timestamp() -> Result<u64, anyhow::Error> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

// EIP-4361 based session

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedMessage {
    pub signature: Signature,
    pub message: Message,
}

pub async fn verify_session(session: &ReadableSession) -> impl IntoResponse {
    match session.get::<String>(NONCE_KEY) {
        Some(_) => {}
        // Invalid nonce
        None => return (StatusCode::UNAUTHORIZED, "Failed to get nonce").into_response(),
    }
    let now = match unix_timestamp() {
        Ok(now) => now,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get unix timestamp.",
            )
                .into_response()
        }
    };
    match session.get::<u64>(EXPIRATION_TIME_KEY) {
        None => {
            return (StatusCode::UNAUTHORIZED, "Failed to get session expiration").into_response()
        }
        Some(ts) => {
            if now > ts {
                return (StatusCode::UNAUTHORIZED, "Session expired").into_response();
            }
        }
    }

    StatusCode::OK.into_response()
}
