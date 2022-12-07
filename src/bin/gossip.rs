use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    gossipsub::{
        self, Gossipsub, GossipsubEvent, GossipsubMessage, MessageAuthenticity, MessageId,
        ValidationMode,
    },
    identity, mdns, mplex, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, PeerId, Swarm, Transport,
};
use quay::telemetry::{get_subscriber, init_subscriber};
use tracing::log::info;

// We create a custom network behaviour that combines Gossipsub and Mdns.
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

    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(&keypair.public());

    let transport = tcp::tokio::Transport::new(tcp::Config::new().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseAuthenticated::xx(&keypair)
                .expect("Signing Libp2p noise static keypair failed"),
        )
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // TODO: Use Seaport's gossip message ID fn
    // see: https://github.com/ProjectOpenSea/seaport-gossip/blob/main/src/util/serialize.ts#L18-L29
    let message_id_fn = |message: &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()
        .expect("Config should be valid");

    // build a gossipsub network behaviour
    let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
        .expect("Correct configuration");

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default())?;
        let behaviour = SeaportGossipBehaviour { gossipsub, mdns };
        Swarm::with_tokio_executor(transport, behaviour, peer_id)
    };

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Listening on {address:?}");
                        }
                        SwarmEvent::Behaviour(SeaportGossipBehaviourEvent::Gossipsub(GossipsubEvent::Message{ message, .. })) => {
                            info!(
                                    "Received: '{:?}' from {:?}",
                                    String::from_utf8_lossy(&message.data),
                                    message.source
                                );
                        }
                        SwarmEvent::Behaviour(SeaportGossipBehaviourEvent::Mdns(event)) => {
                            match event {
                                mdns::Event::Discovered(list) => {
                                    for (peer, _) in list {
                                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                                    }
                                }
                                mdns::Event::Expired(list) => {
                                    for (peer, _) in list {
                                        if !swarm.behaviour().mdns.has_node(&peer) {
                                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer);
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
