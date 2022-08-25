use crate::structs::{SignedMessage, TypedSession};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse};
use ethers::abi::Address;
use siwe;
use std::time::SystemTime;

#[get("/nonce")]
#[tracing::instrument(name = "Getting an EIP-4361 nonce for session", skip(session))]
async fn get_nonce(session: TypedSession) -> HttpResponse {
    let nonce = siwe::nonce::generate_nonce();
    match session.insert_nonce(&nonce) {
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
    // Make sure we don't inherit a dirty settion expiry
    match session.insert_expiration_time(&OffsetDateTime::UNIX_EPOCH) {
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(nonce)
}

#[post("/verify")]
#[tracing::instrument(
    name = "Verifying user EIP-4361 session",
    skip(session, signed_message)
)]
async fn verify(session: TypedSession, signed_message: web::Json<SignedMessage>) -> HttpResponse {
    // Infallible becasuse the signature has already been validated
    let message = signed_message.message.clone();
    // The frontend must set a session expiry
    let session_nonce = match session.get_nonce() {
        Ok(nonce) => match nonce {
            Some(no) => no,
            None => return HttpResponse::UnprocessableEntity().body("Failed to get nonce"),
        },
        // Invalid nonce
        Err(_) => return HttpResponse::UnprocessableEntity().body("Failed to get nonce"),
    };

    // Verify the signed message
    match message.verify(
        signed_message.signature.0,
        Option::None,
        Option::Some(session_nonce.as_str()),
        Option::None,
    ) {
        Ok(_) => {}
        Err(error) => {
            return HttpResponse::UnprocessableEntity().body(format!("Invalid signature {error}"))
        }
    }
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 604800;
    match session.insert_expiration_time(&OffsetDateTime::from_unix_timestamp(now as i64).unwrap())
    {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to insert expiration time.")
        }
    }
    match session.insert_address(&Address::from(message.address)) {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to insert user address.")
        }
    }
    HttpResponse::Ok().finish()
}

#[get("/authenticate")]
#[tracing::instrument(name = "Checking user EIP-4361 authentication", skip(session))]
async fn authenticate(session: TypedSession) -> HttpResponse {
    match session.get_nonce() {
        Ok(_) => {}
        // Invalid nonce
        Err(_) => return HttpResponse::Unauthorized().body("Failed to get nonce"),
    }
    match session.get_expiration_time() {
        Ok(timestamp) => match timestamp {
            None => return HttpResponse::Unauthorized().body("Failed to get session expiration"),
            Some(ts) => {
                if OffsetDateTime::now_utc() > ts {
                    return HttpResponse::Unauthorized().body("Session expired");
                }
            }
        },
        // Invalid nonce
        Err(_) => return HttpResponse::Unauthorized().body("Failed to get session expiration"),
    }

    HttpResponse::Ok().finish()
}
