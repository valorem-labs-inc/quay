use ethers::abi::AbiEncode;

use super::InsertOnlyQuery;
use crate::{
    bindings::{ConsiderationItem, OfferItem},
    structs::OrderInput,
    telemetry::DatabaseMetrics,
};

pub fn save_order(hash: [u8; 32], order: &OrderInput) -> InsertOnlyQuery {
    let _timer = DatabaseMetrics::get()
        .database_queries
        .with_label_values(&["save_order"])
        .start_timer();

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
        hash.encode_hex(),
        order.parameters.offerer.encode_hex(),
        order.parameters.zone.encode_hex(),
        order.parameters.zone_hash.encode_hex(),
        order.parameters.start_time.as_u64() as i64,
        order.parameters.end_time.as_u64() as i64,
        order.parameters.order_type as i32,
        order.parameters.total_original_consideration_items as i32,
        order.parameters.nonce as i64,
        order.parameters.salt.to_string(),
        order.parameters.conduit_key.encode_hex(),
        order.signature.to_string(),
    )
}

pub fn save_offer(hash: [u8; 32], position: i32, offer: &OfferItem) -> InsertOnlyQuery {
    let _timer = DatabaseMetrics::get()
        .database_queries
        .with_label_values(&["save_offer"])
        .start_timer();

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
        hash.encode_hex(),
        offer.item_type as i32,
        offer.token.encode_hex(),
        offer.identifier_or_criteria.encode_hex(),
        offer.start_amount.encode_hex(),
        offer.end_amount.encode_hex()
    )
}

pub fn save_consideration(
    hash: [u8; 32],
    position: i32,
    consideration: &ConsiderationItem,
) -> InsertOnlyQuery {
    let _timer = DatabaseMetrics::get()
        .database_queries
        .with_label_values(&["save_consideration"])
        .start_timer();

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
        hash.encode_hex(),
        consideration.item_type as i32,
        consideration.token.encode_hex(),
        consideration.identifier_or_criteria.encode_hex(),
        consideration.start_amount.encode_hex(),
        consideration.end_amount.encode_hex(),
        consideration.recipient.encode_hex()
    )
}
