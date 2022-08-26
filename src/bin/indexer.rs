use quay::configuration::get_configuration;
use quay::indexer;
use quay::telemetry::{get_subscriber, init_subscriber};
use tracing::error;

fn main() {
    let subscriber = get_subscriber("quay".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    if let Err(e) = indexer::run(configuration) {
        error!("Unhandled application error, panicking.");
        panic!("{}", e);

        // Later, when there are handled cases: process::exit(2..n);
    }
}
