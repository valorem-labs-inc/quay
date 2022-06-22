use quay::configuration::get_configuration;
use quay::startup::run;
use quay::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("quay".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());
    let listener = TcpListener::bind(address).expect("Failed to bind web service port.");
    run(listener, connection_pool)?.await
}
