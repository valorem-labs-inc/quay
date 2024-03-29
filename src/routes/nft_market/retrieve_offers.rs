use anyhow::Error;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use ethers::{abi::AbiEncode, prelude::*};
use http::StatusCode;
use sqlx::{query_as, PgPool};

use crate::structs::{DBConsideration, DBOffer, DBOrder, OrderQuery, RetrieveResponse};

pub async fn retrieve_offers(
    State(pool): State<PgPool>,
    query: Query<OrderQuery>,
) -> impl IntoResponse {
    match retrieve_offers_db(
        &pool,
        query.asset_contract_address.encode_hex(),
        query
            .token_ids
            .clone()
            .into_iter()
            .map(|token_id| {
                U256::from_str_radix(&token_id, 10)
                    .unwrap_or(U256::MAX)
                    .encode_hex()
            })
            .collect::<Vec<String>>()
            .as_slice(),
        query.offerer.encode_hex(),
        query.limit,
    )
    .await
    {
        Ok(retrieved_listings) => {
            (StatusCode::OK, Json::<RetrieveResponse>(retrieved_listings)).into_response()
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn retrieve_offers_db(
    pool: &PgPool,
    asset_contract_address: String,
    token_ids: &[String],
    offerer: String,
    limit: Option<i64>,
) -> Result<RetrieveResponse, Error> {
    let db_orders: Vec<DBOrder> = query_as!(
        DBOrder,
        r#"
            SELECT
                O.hash as "hash!",
                O.offerer::TEXT as "offerer!",
                O.zone::TEXT as "zone!",
                O.zone_hash as "zone_hash!",
                O.start_time as "start_time!",
                O.end_time as "end_time!",
                O.order_type as "order_type!",
                O.total_original_consideration_items as "total_original_consideration_items!",
                O.counter as "counter!",
                O.salt as "salt!",
                O.conduit_key as "conduit_key!",
                O.signature as "signature!",
                array_agg(DISTINCT (
                    OC.position,
                    OC.item_type,
                    OC.token::TEXT,
                    OC.identifier_or_criteria,
                    OC.start_amount,
                    OC.end_amount,
                    OC.recipient::TEXT
                )) AS "considerations!: Vec<DBConsideration>",
                array_agg(DISTINCT (
                    OOF.position,
                    OOF.item_type,
                    OOF.token::TEXT,
                    OOF.identifier_or_criteria,
                    OOF.start_amount,
                    OOF.end_amount
                )) AS "offers!: Vec<DBOffer>"
            FROM orders O
                INNER JOIN considerations OC ON O.hash = OC.order
                INNER JOIN offers OOF ON O.hash = OOF.order
            WHERE O.hash IN (
                SELECT C.order FROM considerations C 
                    WHERE (C.token = $1::TEXT::citext OR $1::TEXT::citext = '0x0000000000000000000000000000000000000000000000000000000000000000')
                    AND (C.identifier_or_criteria = ANY($2::TEXT[]) OR cardinality($2::TEXT[]) = 0)
            )
            AND (O.offerer = $3::TEXT::citext OR $3::TEXT::citext = '0x0000000000000000000000000000000000000000000000000000000000000000')
            GROUP BY O.hash
            LIMIT $4;
        "#,
        asset_contract_address,
        &token_ids[..],
        offerer,
        limit.unwrap_or(1)
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;

    RetrieveResponse::from_db_struct(&db_orders)
}
