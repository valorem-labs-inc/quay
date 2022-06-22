use actix_web::{post, HttpResponse, Responder};

// TODO(Implement model)
// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/create-an-order

#[post("/listings")]
async fn create_listing() -> impl Responder {
    HttpResponse::Ok()
}
