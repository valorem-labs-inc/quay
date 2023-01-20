use std::net::TcpListener;
use std::str::FromStr;
use std::sync::Arc;

use async_redis_session::RedisSessionStore;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_server::Handle;
use axum_sessions::SessionLayer;
use bb8::Pool;
use ethers::prelude::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use redis::aio::ConnectionManager;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tonic::transport::Server;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;

use crate::middleware::{track_prometheus_metrics, RequestIdLayer};
use crate::redis_pool::RedisConnectionManager;
use crate::rfq::rfq_server::RfqServer;
use crate::routes::*;
use crate::services::*;
use crate::session::session_server::SessionServer;
use crate::structs::RetrieveResponse;
use crate::{bindings::Seaport, state::AppState};
use crate::{
    configuration::{DatabaseSettings, Settings},
    telemetry::TowerMakeSpanWithConstantId,
};

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    redis_pool: Pool<RedisConnectionManager>,
    redis_multiplexed: ConnectionManager,
    session_layer: SessionLayer<RedisSessionStore>,
    rpc: Provider<Http>,
) -> BoxFuture<'static, Result<(), std::io::Error>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_check,
            metrics_prometheus,
            create_listing,
            retrieve_listings,
            create_offer,
            retrieve_offers,
            get_nonce,
            verify,
            authenticate
        ),
        components(
            schemas(RetrieveResponse)
        ),
        tags(
            (name = "quay", description = "Quay is an open source, high performance backend for the Seaport smart 
            contracts")
        )
    )]
    struct ApiDoc;

    let provider = Arc::new(rpc.clone());

    let seaport = Seaport::new(
        H160::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap(),
        provider,
    );

    let cors = CorsLayer::very_permissive();

    let state = AppState {
        db_pool,
        redis_pool,
        redis_multiplexed,
        rpc,
        seaport,
    };

    // TODO(Cleanup duplicate state)
    let http = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health_check", get(health_check))
        .route("/metrics/prometheus", get(metrics_prometheus))
        .route("/listings", post(create_listing).get(retrieve_listings))
        .route("/offers", post(create_offer).get(retrieve_offers))
        .route("/nonce", get(get_nonce))
        .route("/verify", post(verify))
        .route("/authenticate", get(authenticate))
        .route(
            "/spec/v3",
            get(|| async { ApiDoc::openapi().to_json().unwrap() }),
        )
        // Layers/middleware
        .layer(TraceLayer::new_for_http().make_span_with(TowerMakeSpanWithConstantId))
        .layer(RequestIdLayer)
        .layer(session_layer.clone())
        .layer(middleware::from_fn(track_prometheus_metrics))
        .layer(cors)
        // State
        .with_state(state)
        .map_err(BoxError::from)
        .boxed_clone();

    let grpc = Server::builder()
        .layer(RequestIdLayer)
        .layer(TraceLayer::new_for_http().make_span_with(TowerMakeSpanWithConstantId))
        .layer(session_layer)
        .add_service(RfqServer::new(RFQService::new()))
        .add_service(SessionServer::new(SessionService::default()))
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
            .build(crate::redis_pool::RedisConnectionManager::new(
                configuration.redis_url.expose_secret().as_str(),
            )?)
            .await?;
        let redis_multiplexed = ConnectionManager::new(redis::Client::open(
            configuration.redis_url.expose_secret().as_str(),
        )?)
        .await?;

        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let store = RedisSessionStore::new(redis_multiplexed.clone(), Some("/sessions".into()));
        let secret = configuration
            .application
            .hmac_secret
            .expose_secret()
            .as_bytes(); // MUST be at least 64 bytes!
        let session_layer = SessionLayer::new(store, secret);

        let server = run(
            listener,
            db_pool,
            redis_pool,
            redis_multiplexed,
            session_layer,
            provider,
        );

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
