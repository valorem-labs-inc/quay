use crate::structs::{DBConsideration, DBOffer, DBOrder, OrderQuery, RetrieveResponse};
use actix_web::{get, web, HttpResponse, Responder};
use anyhow::Error;
use ethers::{abi::AbiEncode, types::U256};
use sqlx::{query_as, PgPool};

// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/retrieve-listings

#[get("/listings")]
#[tracing::instrument(
name = "Fetching listings matching the passed tokenIds",
skip(query, pool),
fields(
asset_contract_address = query.asset_contract_address.encode_hex(),
token_ids = query.token_ids.join(","),
limit = %query.limit.unwrap_or(1),
)
)]
async fn listings(query: web::Query<OrderQuery>, pool: web::Data<PgPool>) -> impl Responder {
    match retrieve_listings(
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
        query.limit,
    )
    .await
    {
        Ok(retrieved_listings) => HttpResponse::Ok().json(retrieved_listings),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Fetching listings matching the passed token_ids from the database",
    skip(pool, asset_contract_address, token_ids, limit)
)]
async fn retrieve_listings(
    pool: &PgPool,
    asset_contract_address: String,
    token_ids: &[String],
    limit: Option<i64>,
) -> Result<RetrieveResponse, Error> {
    let db_orders: Vec<DBOrder> = query_as!(
        DBOrder,
        r#"
            SELECT
                O.hash as "hash!",
                O.offerer as "offerer!",
                O.zone as "zone!",
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
                    OC.token,
                    OC.identifier_or_criteria,
                    OC.start_amount,
                    OC.end_amount,
                    OC.recipient
                )) AS "considerations!: Vec<DBConsideration>",
                array_agg(DISTINCT (
                    OOF.position,
                    OOF.item_type,
                    OOF.token,
                    OOF.identifier_or_criteria,
                    OOF.start_amount,
                    OOF.end_amount
                )) AS "offers!: Vec<DBOffer>"
            FROM orders O
                INNER JOIN considerations OC ON O.hash = OC.order
                INNER JOIN offers OOF ON O.hash = OOF.order
            WHERE O.hash IN (
                SELECT OF.order FROM offers OF
                    WHERE (OF.token = $1)
                    AND (OF.identifier_or_criteria = ANY($2::TEXT[]))
            )
            GROUP BY O.hash
            LIMIT $3;
        "#,
        asset_contract_address,
        &token_ids[..],
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
