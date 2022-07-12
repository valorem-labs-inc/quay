use crate::seaport::Order;
use actix_web::{post, web, HttpResponse, Responder};

// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/create-an-order

#[post("/listings")]
async fn create_listing(_form: web::Json<Order>) -> impl Responder {
    HttpResponse::Ok()
}
