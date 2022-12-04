use crate::{
    structs::{DBConsideration, DBOffer, DBOrder, RetrieveOrdersQuery, RetrieveResponse},
    utils::token_ids_to_u256_abi_encoded,
};
use actix_web::{error, http::StatusCode};
use ethers::abi::AbiEncode;
use paperclip::actix::{
    api_v2_operation, get,
    web::{self, Json},
};
use sqlx::{query_as, PgPool};

async fn seaport_opensea_retrieve_listings(
    query: web::Query<RetrieveOrdersQuery>,
    pool: web::Data<PgPool>,
) -> Result<Json<RetrieveResponse>, actix_web::Error> {
    let token_ids = match token_ids_to_u256_abi_encoded(&query.token_ids) {
        Ok(token_ids) => token_ids,
        Err(e) => {
            return Err(
                error::InternalError::new(e.to_string(), StatusCode::UNPROCESSABLE_ENTITY).into(),
            )
        }
    };
    match retrieve_listings(
        &pool,
        query.asset_contract_address.encode_hex(),
        token_ids.as_slice(),
        query.active.unwrap_or(false),
        query.limit,
    )
    .await
    {
        Ok(retrieved_listings) => Ok(Json(retrieved_listings)),
        Err(e) => {
            Err(error::InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into())
        }
    }
}

async fn retrieve_listings(
    pool: &PgPool,
    asset_contract_address: String,
    token_ids: &[String],
    active: bool,
    limit: Option<i64>,
) -> Result<RetrieveResponse, anyhow::Error> {
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
                O.salt as "salt!",
                O.conduit_key as "conduit_key!",
                O.signature as "signature!",
                O.listing_time as "listing_time!",
                O.counter as "counter!",
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
            WHERE
                O.hash IN (
                    SELECT OF.order FROM offers OF
                        WHERE (OF.token = $1)
                        AND (OF.identifier_or_criteria = ANY($2::TEXT[]))
                )
               AND
                ((NOT $3) OR (
                        O.cancelled = FALSE
                    AND O.finalized = FALSE
                    AND O.marked_invalid = FALSE
                    AND O.start_time <= extract(epoch from now())
                    AND O.end_time >= extract(epoch from now())
                ))
            GROUP BY O.hash
            LIMIT $4;
        "#,
        asset_contract_address,
        &token_ids[..],
        active,
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
