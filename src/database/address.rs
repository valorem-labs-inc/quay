use anyhow::Result;
use ethers::{prelude::*, abi::AbiEncode};
use sqlx::{Postgres, postgres::{PgQueryResult, PgArguments}, PgPool, PgExecutor, query::Query};

use super::InsertOnlyQuery;

pub fn save_address(address: H160) -> InsertOnlyQuery {
    sqlx::query!(
        r#"
            INSERT INTO addresses (address)
                VALUES ($1::TEXT::citext)
                ON CONFLICT (address) DO NOTHING;
        "#,
        address.encode_hex()
    )
}
