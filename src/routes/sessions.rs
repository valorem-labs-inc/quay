use axum::extract::Json;
use axum::response::IntoResponse;
use axum_sessions::extractors::{ReadableSession, WritableSession};

use ethers::types::Address;
use http::{header, HeaderMap, StatusCode};

use siwe::VerificationOpts;

use crate::auth::*;

#[utoipa::path(
    get,
    path = "/nonce",
    responses(
        (status = 200, description = "Get nonce for session", body = Stirng),
        (status = 500, description = "Failed to get nonce")
    )
)]
#[tracing::instrument(name = "Getting an EIP-4361 nonce for session", skip(session))]
pub async fn get_nonce(mut session: WritableSession) -> impl IntoResponse {
    let nonce = siwe::generate_nonce();
    match &session.insert(NONCE_KEY, &nonce) {
        Ok(_) => {}
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set nonce.").into_response()
        }
    }
    // Make sure we don't inherit a dirty session expiry
    let ts = match unix_timestamp() {
        Ok(ts) => ts,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get unix timestamp.",
            )
                .into_response()
        }
    };
    match session.insert(EXPIRATION_TIME_KEY, ts) {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to set expiration.",
            )
                .into_response()
        }
    }
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, nonce).into_response()
}

/// Verify
///
/// Verify
#[utoipa::path(
    post,
    path = "/verify",
    request_body = SignedMessage,
    responses(
        (status = 200, description = "Successfully verified"),
        (status = 422, description = "Failed to get nonce or failed to validate signature"),
        (status = 500, description = "Failed to varify")
    )
)]
#[tracing::instrument(
    name = "Verifying user EIP-4361 session",
    skip(session, signed_message)
)]
pub async fn verify(
    mut session: WritableSession,
    signed_message: Json<SignedMessage>,
) -> impl IntoResponse {
    // Infallible because the signature has already been validated
    let message = signed_message.message.clone();
    // The frontend must set a session expiry
    let session_nonce = match session.get(NONCE_KEY) {
        Some(no) => no,
        None => return (StatusCode::UNPROCESSABLE_ENTITY, "Failed to get nonce.").into_response(),
    };

    // Verify the signed message
    match message
        .verify(
            signed_message.signature.as_ref(),
            &VerificationOpts {
                nonce: Some(session_nonce),
                ..Default::default()
            },
        )
        .await
    {
        Ok(_) => {}
        Err(error) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Invalid signature {error}."),
            )
                .into_response()
        }
    }
    let now = match unix_timestamp() {
        Ok(now) => now,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get timestamp.",
            )
                .into_response()
        }
    };
    let expiry = now + 604800;
    match session.insert(EXPIRATION_TIME_KEY, expiry) {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert expiration time.",
            )
                .into_response()
        }
    }
    match session.insert(USER_ADDRESS_KEY, Address::from(message.address)) {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert user address.",
            )
                .into_response()
        }
    }
    (StatusCode::OK).into_response()
}

/// Authenticate
///
/// Verify session
#[utoipa::path(
    get,
    path = "/authenticate",
    responses(
        (status = 200, description = "Successfully verified session"),
        (status = 401, description = "Failed to verify session"),
        (status = 500, description = "Fialed to verify session")
    )
)]
#[tracing::instrument(name = "Checking user EIP-4361 authentication", skip(session))]
pub async fn authenticate(session: ReadableSession) -> impl IntoResponse {
    verify_session(&session).await
}
