use std::net::TcpListener;
use std::str::FromStr;
use std::sync::Arc;

use axum::{middleware, routing::{get,post}, Router};
use axum_server::Handle;
use ethers::prelude::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tonic::transport::Server;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error_span;

use crate::{bindings::Seaport, state::AppState};
use crate::configuration::{DatabaseSettings, Settings};
use crate::middleware::{track_prometheus_metrics, RequestId, RequestIdLayer};
use crate::request_for_quote::request_for_quote_server::RequestForQuoteServer;
use crate::routes::*;
use crate::services::*;

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
    let provider = Arc::new(rpc.clone());

    let seaport = Seaport::new(
        H160::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap(),
        provider,
    );

    let tracing_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".into());
        error_span!(
            "request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    });
    let cors = CorsLayer::very_permissive();

    let state = AppState {
        db_pool: db_pool.clone(),
        rpc: rpc.clone(),
        seaport: seaport.clone()
    };

    // TODO(Cleanup duplicate state)
    let http = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .route("/metrics/prometheus", get(metrics_prometheus))
        .layer(tracing_layer)
        .layer(RequestIdLayer)
        .layer(middleware::from_fn(track_prometheus_metrics))
        .layer(cors)
        .with_state(state)
        .map_err(BoxError::from)
        .boxed_clone();

    let grpc = Server::builder()
        .add_service(RequestForQuoteServer::new(MyRFQ::default()))
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .boxed_clone();

    let http_grpc = Steer::new(vec![http, grpc], |req: &Request<Body>, _svcs: &[_]| {
        if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
            0
        } else {
            1
        }
    });

    let handle = Handle::new();

    axum_server::from_tcp(listener)
        .handle(handle)
        .serve(Shared::new(http_grpc))
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
