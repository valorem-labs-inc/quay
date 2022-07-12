use crate::seaport::Order;
use actix_web::{post, web, HttpResponse, Responder};

// TODO(Implement queries)

#[post("/offers")]
async fn create_offer(_offer: web::Json<Order>) -> impl Responder {
    HttpResponse::Ok()
}
