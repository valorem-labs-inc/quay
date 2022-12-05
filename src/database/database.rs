use anyhow::Result;
use ethers::{prelude::*, abi::AbiEncode};
use sqlx::{Postgres, postgres::{PgQueryResult, PgArguments}, PgPool, PgExecutor, query::Query};

pub type InsertOnlyQuery = Query<'static, Postgres, PgArguments>;
