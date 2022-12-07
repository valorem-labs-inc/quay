use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent},
    identity, mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
};
use quay::{
    configuration::get_configuration,
    gossip::QuayGossipNode,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::log::info;

#[derive(NetworkBehaviour)]
struct SeaportGossipBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("quay-gossip".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    info!("Starting Quay Gossip Client");

    let _config = get_configuration().expect("Could not read config");

    let keypair = identity::Keypair::generate_ed25519();
    let mut node = QuayGossipNode::new(keypair)?;

    node.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        tokio::select! {
                event = node.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Listening on {address:?}");
                        }
                        SwarmEvent::Behaviour(quay::gossip::SeaportGossipBehaviourEvent::Gossipsub(GossipsubEvent::Message{ message, .. })) => {
                            info!(
                                    "Received: '{:?}' from {:?}",
                                    String::from_utf8_lossy(&message.data),
                                    message.source
                                );
                        }
                        SwarmEvent::Behaviour(quay::gossip::SeaportGossipBehaviourEvent::Mdns(event)) => {
                            match event {
                                mdns::Event::Discovered(list) => {
                                    for (peer, _) in list {
                                        node.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                                    }
                                }
                                mdns::Event::Expired(list) => {
                                    for (peer, _) in list {
                                        if !node.swarm.behaviour().mdns.has_node(&peer) {
                                            node.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}

                }
            }
        }
    }
}
