use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use ethers::prelude::*;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::net::TcpListener;
use std::str::FromStr;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

use crate::routes::*;

use crate::configuration::{DatabaseSettings, Settings};
use crate::seaport::Seaport;
use sqlx::postgres::PgPoolOptions;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let redis_store = RedisSessionStore::new(configuration.redis_url.expose_secret()).await?;
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let provider: Provider<Http> =
            Provider::new(Http::from_str(configuration.rpc.uri.as_str()).unwrap());
        let server = run(
            listener,
            connection_pool,
            configuration.application.hmac_secret,
            provider,
            redis_store,
        )?;

        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
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
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: Secret<String>,
    rpc: Provider<Http>,
    redis_store: RedisSessionStore,
) -> Result<Server, anyhow::Error> {
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let db_pool = web::Data::new(db_pool);
    let provider = Arc::new(rpc);
    let seaport = web::Data::new(Seaport::new(
        H160::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap(),
        provider,
    ));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .service(health_check)
            .service(offers)
            .service(listings)
            .service(create_listing)
            .service(create_offer)
            .service(get_nonce)
            .service(verify)
            .service(authenticate)
            .app_data(db_pool.clone())
            .app_data(seaport.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
