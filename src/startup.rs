use axum::routing::get;
use axum::Router;
use axum::ServiceExt as axumServiceExt;
use ethers::prelude::*;
use futures::Future;
use sqlx::{PgPool, Pool, Postgres};
use std::net::SocketAddr;
use std::str::FromStr;
use tower::util::ServiceExt;
use tower_http::trace::TraceLayer;

use crate::routes::*;

use crate::configuration::{DatabaseSettings, Settings};
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    address: SocketAddr,
    connection_pool: Pool<Postgres>,
    provider: Provider<Http>,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());

        let address = SocketAddr::from_str(&format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        ))
        .unwrap();

        Ok(Self {
            address,
            connection_pool,
            provider,
        })
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        run(self.address, self.connection_pool, self.provider).await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    address: SocketAddr,
    db_pool: PgPool,
    rpc: Provider<Http>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .with_state(db_pool)
        .with_state(rpc)
        .layer(TraceLayer::new_for_http())
        .boxed_clone();

    let server = axum_server::bind(address).serve(app.into_make_service());

    server
}
