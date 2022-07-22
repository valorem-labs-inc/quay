use crate::seaport::{Order, OrderComponents, Seaport};
use actix_web::{post, web, HttpResponse};
use ethers::prelude::*;
use sqlx::PgPool;

#[post("/offers")]
#[tracing::instrument(
name = "Adding a new offer",
skip(offer, pool, seaport),
fields(
offerer = %offer.parameters.offerer,
)
)]
async fn create_offer(
    offer: web::Json<Order>,
    pool: web::Data<PgPool>,
    seaport: web::Data<Seaport<Provider<ethers::providers::Http>>>,
) -> HttpResponse {
    if insert_offer(&pool, &offer, &seaport).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new offer details in the database",
    skip(new_offer, _pool, seaport)
)]
pub async fn insert_offer(
    _pool: &PgPool,
    new_offer: &Order,
    seaport: &Seaport<Provider<Http>>,
) -> Result<(), sqlx::Error> {
    // TODO(Implement queries)
    // The order model in the database differs significantly from the contract order parameters
    // Hashes are used, which must be updated from indexed events
    let order_hash = seaport
        .get_order_hash(OrderComponents::from_parameters(seaport, &new_offer.parameters).await)
        .call()
        .await
        .expect("failed to calculate hash");
    let _order_status = seaport
        .get_order_status(order_hash)
        .call()
        .await
        .expect("failed to get order status");
    Ok(())
}
