use anyhow::Error;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use ethers::{abi::AbiEncode, prelude::*};
use http::StatusCode;
use sqlx::PgPool;

use crate::bindings::seaport::Seaport;
use crate::structs::OrderInput;

pub async fn seaport_legacy_create_listing(
    State(db_pool): State<PgPool>,
    State(seaport): State<Seaport<Provider<Http>>>,
    Json(listing): Json<OrderInput>,
) -> impl IntoResponse {
    // TODO(Pass authenticated user details for verification in order)
    if insert_listing(&db_pool, &listing, &seaport).await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::OK).into_response()
}

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
    sqlx::query!(
        r#"
            INSERT INTO addresses (address)
                VALUES ($1::TEXT::citext)
                ON CONFLICT (address) DO NOTHING;
        "#,
        new_listing.parameters.offerer.encode_hex()
    )
    .execute(&mut tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    sqlx::query!(
        r#"
            INSERT INTO addresses (address)
                VALUES ($1::TEXT::citext)
                ON CONFLICT (address) DO NOTHING;
        "#,
        new_listing.parameters.zone.encode_hex()
    )
    .execute(&mut tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    sqlx::query!(
        r#"
            INSERT INTO orders (
                hash,
                offerer,
                zone,
                zone_hash,
                start_time,
                end_time,
                order_type,
                total_original_consideration_items,
                counter,
                salt,
                conduit_key,
                signature
            )
                VALUES ($1, $2::TEXT::citext, $3::TEXT::citext, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                ON CONFLICT (hash) DO NOTHING;
        "#,
        order_hash.encode_hex(),
        new_listing.parameters.offerer.encode_hex(),
        new_listing.parameters.zone.encode_hex(),
        new_listing.parameters.zone_hash.encode_hex(),
        new_listing.parameters.start_time.as_u64() as i64,
        new_listing.parameters.end_time.as_u64() as i64,
        new_listing.parameters.order_type as i32,
        new_listing.parameters.total_original_consideration_items as i32,
        new_listing.parameters.nonce as i64,
        new_listing.parameters.salt.to_string(),
        new_listing.parameters.conduit_key.encode_hex(),
        new_listing.signature.to_string(),
    )
    .execute(&mut tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    let mut position = 0;
    for offer in &new_listing.parameters.offer {
        sqlx::query!(
            r#"
                INSERT INTO addresses (address)
                    VALUES ($1::TEXT::citext)
                    ON CONFLICT (address) DO NOTHING;
            "#,
            offer.token.encode_hex()
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        // Insert offer item
        sqlx::query!(
            r#"
                INSERT INTO offers (
                    position,
                    "order",
                    item_type,
                    token,
                    identifier_or_criteria,
                    start_amount,
                    end_amount
                )
                    VALUES ($1, $2, $3, $4::TEXT::citext, $5, $6, $7)
                    ON CONFLICT ("order", position) DO NOTHING;
            "#,
            position,
            order_hash.encode_hex(),
            offer.item_type as i32,
            offer.token.encode_hex(),
            offer.identifier_or_criteria.encode_hex(),
            offer.start_amount.encode_hex(),
            offer.end_amount.encode_hex()
        )
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
        sqlx::query!(
            r#"
                INSERT INTO addresses (address)
                    VALUES ($1::TEXT::citext)
                    ON CONFLICT (address) DO NOTHING;
            "#,
            consideration.token.encode_hex()
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
        sqlx::query!(
            r#"
                INSERT INTO addresses (address)
                    VALUES ($1::TEXT::citext)
                    ON CONFLICT (address) DO NOTHING;
            "#,
            consideration.recipient.encode_hex()
        )
        .execute(&mut tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        sqlx::query!(
            r#"
                INSERT INTO considerations (
                    position,
                    "order",
                    item_type,
                    token,
                    identifier_or_criteria,
                    start_amount,
                    end_amount,
                    recipient
                )
                    VALUES ($1, $2, $3, $4::TEXT::citext, $5, $6, $7, $8::TEXT::citext)
                    ON CONFLICT ("order", position) DO NOTHING;
            "#,
            position,
            order_hash.encode_hex(),
            consideration.item_type as i32,
            consideration.token.encode_hex(),
            consideration.identifier_or_criteria.encode_hex(),
            consideration.start_amount.encode_hex(),
            consideration.end_amount.encode_hex(),
            consideration.recipient.encode_hex()
        )
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
