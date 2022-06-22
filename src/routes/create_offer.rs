use actix_web::{post, HttpResponse, Responder};

// TODO(Implement model)
// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/create-an-offer

#[post("/offers")]
async fn create_offer() -> impl Responder {
    HttpResponse::Ok()
}
