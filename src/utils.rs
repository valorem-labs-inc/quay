use crate::structs::TypedSession;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::HttpResponse;

pub async fn verify_session(session: &TypedSession) -> HttpResponse {
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
