use actix_web::{get, HttpResponse, Responder};

// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/create-an-order

#[get("/offers")]
async fn offers() -> impl Responder {
    HttpResponse::Ok()
}
