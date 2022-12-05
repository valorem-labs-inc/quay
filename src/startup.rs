use std::net::TcpListener;
use std::str::FromStr;
use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_server::Handle;
use bb8::Pool;
use ethers::prelude::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tonic::transport::Server;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error_span;

use crate::configuration::{DatabaseSettings, Settings};
use crate::middleware::{track_prometheus_metrics, RequestId, RequestIdLayer};
use crate::redis::RedisConnectionManager;
use crate::routes::*;
use crate::services::*;
use crate::{bindings::Seaport, state::AppState};
use crate::request_for_quote::quote_server::QuoteServer;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    redis_pool: Pool<RedisConnectionManager>,
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
        db_pool,
        redis_pool,
        rpc,
        seaport,
    };

    // TODO(Cleanup duplicate state)
    let http = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .route("/metrics/prometheus", get(metrics_prometheus))
        .route(
            "/seaport/legacy/listings",
            post(seaport_legacy_create_listing).get(seaport_legacy_retrieve_listings),
        )
        .route(
            "/seaport/legacy/offers",
            post(seaport_legacy_create_offer).get(seaport_legacy_retrieve_offers),
        )
        // Legacy endpoints to keep compatibility
        .route(
            "/listings",
            post(seaport_legacy_create_listing).get(seaport_legacy_retrieve_listings),
        )
        .route(
            "/offers",
            post(seaport_legacy_create_offer).get(seaport_legacy_retrieve_offers),
        )
        // Layers/middleware
        .layer(tracing_layer)
        .layer(RequestIdLayer)
        .layer(middleware::from_fn(track_prometheus_metrics))
        .layer(cors)
        // State
        .with_state(state)
        .map_err(BoxError::from)
        .boxed_clone();

    let grpc = Server::builder()
        .add_service(QuoteServer::new(RFQService::default()))
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

    // TODO(Should we be using a tokio future here?)
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
        let db_pool = get_connection_pool(&configuration.database);
        let redis_pool = Pool::builder()
            .build(crate::redis::RedisConnectionManager::new(
                configuration.redis_url.expose_secret().as_str(),
            )?)
            .await?;

        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, db_pool, redis_pool, provider);

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
