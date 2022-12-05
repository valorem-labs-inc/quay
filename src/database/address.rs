use ethers::{abi::AbiEncode, prelude::*};

use super::InsertOnlyQuery;
use crate::telemetry::DatabaseMetrics;

pub fn save_address(address: H160) -> InsertOnlyQuery {
    let _timer = DatabaseMetrics::get()
        .database_queries
        .with_label_values(&["save_address"])
        .start_timer();

    sqlx::query!(
        r#"
            INSERT INTO addresses (address)
                VALUES ($1::TEXT::citext)
                ON CONFLICT (address) DO NOTHING;
        "#,
        address.encode_hex()
    )
}
