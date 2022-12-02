use std::net::TcpListener;
use std::str::FromStr;

use axum::routing::get;
use axum::Router;
use axum::ServiceExt as axumServiceExt;
use ethers::prelude::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
use tower::util::ServiceExt;
use tower_http::trace::TraceLayer;

use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::*;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    rpc: Provider<Http>,
) -> BoxFuture<'static, Result<(), std::io::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .with_state(db_pool)
        .with_state(rpc)
        .layer(TraceLayer::new_for_http())
        .boxed_clone();

    axum_server::from_tcp(listener)
        .serve(app.into_make_service())
        .boxed()
}


pub struct Application {
    server: BoxFuture<'static, Result<(), std::io::Error>>,
    port: u16,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, connection_pool, provider);

        Ok(Self { server, port })
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}