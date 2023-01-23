use anyhow::Error;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use axum_sessions::extractors::ReadableSession;
use ethers::prelude::*;
use http::StatusCode;
use sqlx::PgPool;

use crate::auth::verify_session;
use crate::{
    bindings::seaport::Seaport,
    database::{save_address, save_consideration, save_offer},
};
use crate::{database::save_order, structs::OrderInput};

/// Create listing
///
/// Create a new listing
#[utoipa::path(
    post,
    path = "/listings",
    request_body = OrderInput,
    responses(
        (status = 200, description = "Create listing successfully"),
        (status = 500, description = "Failed to create listing")
    )
)]
#[tracing::instrument(
name = "Adding a new listing",
skip(db_pool, seaport, session, listing),
fields(
offerer = %listing.parameters.offerer,
)
)]
pub async fn create_listing(
    session: ReadableSession,
    State(db_pool): State<PgPool>,
    State(seaport): State<Seaport<Provider<Http>>>,
    Json(listing): Json<OrderInput>,
) -> impl IntoResponse {
    let authenticated = verify_session(&session).await.into_response();
    if authenticated.status() != StatusCode::OK {
        return authenticated;
    }

    if insert_listing(&db_pool, &listing, &seaport).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::OK).into_response()
}

#[tracing::instrument(
    name = "Saving new listing details in the database",
    skip(new_listing, pool, seaport)
)]
pub async fn insert_listing(
    pool: &PgPool,
    new_listing: &OrderInput,
    seaport: &Seaport<Provider<Http>>,
) -> Result<(), Error> {
    // Could we generate this without an RPC call?

    let order_hash = seaport
        .get_order_hash(new_listing.to_components().await)
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
    save_address(new_listing.parameters.offerer)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    save_address(new_listing.parameters.zone)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    save_order(order_hash, new_listing)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    let mut position = 0;
    for offer in &new_listing.parameters.offer {
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
    for consideration in &new_listing.parameters.consideration {
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
