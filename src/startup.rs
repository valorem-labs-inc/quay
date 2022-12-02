use axum::routing::get;
use axum::Router;
use ethers::prelude::*;
use futures::Future;
use sqlx::{PgPool, Pool, Postgres};
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::sync::Arc;
use tower::BoxError;

use crate::routes::*;

use crate::bindings::seaport::Seaport;
use crate::configuration::{DatabaseSettings, Settings};
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    port: u16,
    connection_pool: Pool<Postgres>,
    provider: Provider<Http>,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let port = configuration.application.port;
        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());

        let server = run(port, connection_pool, provider)?;

        // We "save" the bound port in one of `Application`'s fields
        Ok(Self {
            port,
            connection_pool,
            provider,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        run(self.port, self.connection_pool, self.provider).await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

// Workaround for type based data retrieval
#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Debug)]
pub struct RPCUri(pub String);

pub fn run(
    port: u16,
    db_pool: PgPool,
    rpc: Provider<Http>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .with_state(db_pool)
        .with_state(rpc)
        .map_err(BoxError::from)
        .boxed_clone();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server = axum_server::bind(addr).serve(app.into_make_service());

    server
}
