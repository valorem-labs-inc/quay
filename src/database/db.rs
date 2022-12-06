use sqlx::{postgres::PgArguments, query::Query, Postgres};

pub type InsertOnlyQuery = Query<'static, Postgres, PgArguments>;
