use actix_web::{get, HttpResponse, Responder};

// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/retrieve-listings

#[get("/listings")]
async fn listings() -> impl Responder {
    HttpResponse::Ok()
}
