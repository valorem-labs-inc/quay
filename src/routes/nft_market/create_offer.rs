use anyhow::Error;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};

use ethers::prelude::*;
use http::StatusCode;
use sqlx::PgPool;

use crate::structs::OrderInput;
use crate::{
    bindings::seaport::Seaport,
    database::{save_address, save_consideration, save_offer, save_order},
};

#[tracing::instrument(
name = "Adding a new offer",
skip(offer, db_pool, seaport),
fields(
offerer = %offer.parameters.offerer,
)
)]
pub async fn create_offer(
    State(db_pool): State<PgPool>,
    State(seaport): State<Seaport<Provider<Http>>>,
    Json(offer): Json<OrderInput>,
) -> impl IntoResponse {
    //let authenticated = verify_session(&session).await;

    if insert_offer_db(&db_pool, &offer, &seaport).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::OK).into_response()
}

pub async fn insert_offer_db(
    pool: &PgPool,
    new_offer: &OrderInput,
    seaport: &Seaport<Provider<Http>>,
) -> Result<(), Error> {
    // Could we generate this without an RPC call?

    let order_hash = seaport
        .get_order_hash(new_offer.to_components().await)
        .call()
        .await
        .expect("failed to calculate hash");
    // TODO(Ensure the order hasn't been filled)
    // TODO(Any other semantic validation which needs to occur from the RPC)
    // TODO(Implement additional queries for offers and considerations)

    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {:?}", e);
        e
    })?;
    save_address(new_offer.parameters.offerer)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    save_address(new_offer.parameters.zone)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    save_order(order_hash, new_offer)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    let mut position = 0;
    for offer in &new_offer.parameters.offer {
        save_address(offer.token)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;

        save_offer(order_hash, position, offer)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;
        position += 1;
    }
    position = 0;
    for consideration in &new_offer.parameters.consideration {
        save_address(consideration.token)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;
        save_address(consideration.recipient)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;

        save_consideration(order_hash, position, consideration)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;
        position += 1;
    }
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {:?}", e);
        e
    })?;
    Ok(())
}
