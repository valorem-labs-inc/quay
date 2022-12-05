use anyhow::Result;
use ethers::{abi::AbiEncode, prelude::*};
use sqlx::{
    postgres::{PgArguments, PgQueryResult},
    query::Query,
    PgExecutor, PgPool, Postgres,
};

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
