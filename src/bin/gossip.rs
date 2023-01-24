use anyhow::Result;
use clap::Parser;
use ethers::types::H160;
use libp2p::identity;
use quay::{
    configuration::get_configuration,
    gossip::node::QuayGossipNode,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::{error, info};

/// The options for running a gossip node.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct GossipNodeArgs {
    /// The port the node will run from. If this is not assigned,
    /// a random port will be used.
    #[arg(short, long)]
    port: Option<u16>,
    /// The collection addresses the node will subscribe to
    #[arg(short, long)]
    collection_addresses: Option<Vec<H160>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("quay-gossip".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let mut config = get_configuration().expect("Could not read config");
    let keypair = identity::Keypair::generate_ed25519();

    let args = GossipNodeArgs::parse();

    if let Some(port) = args.port {
        config.gossip.port = port;
    }

    if let Some(collection_addresses) = args.collection_addresses {
        config.gossip.collection_addresses = Some(collection_addresses);
    }

    info!("Starting node on port {}", config.gossip.port);

    let mut node = QuayGossipNode::new(keypair, config.gossip)?;

    if let Err(e) = node.run().await {
        error!("Unhandled node error: {}", e.to_string());
        panic!("{}", e);
    }

    Ok(())
}
