use axum::response::IntoResponse;
use axum_sessions::extractors::WritableSession;
use http::{header, HeaderMap, StatusCode};
use std::time::{SystemTime, UNIX_EPOCH};

const NONCE_KEY: &'static str = "nonce";
const EXPIRATION_TIME_KEY: &'static str = "expirationTime";
const USER_ADDRESS_KEY: &'static str = "userAddress";

fn unix_timestamp() -> Result<u64, anyhow::Error> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

//#[tracing::instrument(name = "Getting an EIP-4361 nonce for session", skip(session))]
pub async fn get_nonce(mut session: WritableSession) -> impl IntoResponse {
    let nonce = siwe::generate_nonce();
    match &session.insert(NONCE_KEY, &nonce) {
        Ok(_) => {}
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
    // Make sure we don't inherit a dirty settion expiry
    let ts = match unix_timestamp() {
        Ok(ts) => ts,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };
    match session.insert(EXPIRATION_TIME_KEY, ts) {
        Ok(_) => {}
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, nonce).into_response()
}
