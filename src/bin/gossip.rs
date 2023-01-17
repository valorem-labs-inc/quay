use anyhow::Result;
use libp2p::identity;
use quay::{
    configuration::get_configuration,
    gossip::node::QuayGossipNode,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("quay-gossip".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Could not read config");

    let keypair = identity::Keypair::generate_ed25519();

    let node = QuayGossipNode::new(keypair, config.gossip)?;

    if let Err(e) = node.run().await {
        error!("Unhandled node error. Exiting");
        panic!("{}", e);
    }

    Ok(())
}
