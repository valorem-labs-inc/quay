use anyhow::Result;
use ethers::{abi::AbiEncode, prelude::*};
use sqlx::{
    postgres::{PgArguments, PgQueryResult},
    query::Query,
    PgExecutor, PgPool, Postgres,
};

pub type InsertOnlyQuery = Query<'static, Postgres, PgArguments>;
