use crate::seaport::Order;
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

#[post("/offers")]
#[tracing::instrument(
name = "Adding a new offer",
skip(offer, pool),
fields(
offerer = %offer.parameters.offerer,
)
)]
async fn create_offer(offer: web::Json<Order>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_offer = match offer.0.try_into() {
        Ok(offer) => offer,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    if insert_offer(&pool, &new_offer).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Saving new offer details in the database",
    skip(new_offer, pool)
)]
pub async fn insert_offer(pool: &PgPool, new_offer: &Order) -> Result<(), sqlx::Error> {
    // TODO(Implement queries)
    // The order model in the database differs significantly from the contract order parameters
    // Hashes are used, which must be updated from indexed events
    Ok(())
}
