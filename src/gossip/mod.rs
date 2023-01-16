#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use)]

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
        self,
        error::{PublishError, SubscriptionError},
        Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
        MessageId, ValidationMode,
    },
    identify,
    identity::Keypair,
    mdns, mplex, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, PeerId, Swarm, Transport,
};
use tracing::info;

use crate::configuration::GossipNodeSettings;

/// The seaport gossip network protocol ID & version.
pub const PROTOCOL_ID: &str = "seaport/0.0.1";

/// The custom seaport gossip behavior.
#[derive(NetworkBehaviour)]
pub struct SeaportGossipBehaviour {
    /// Used for distributing orders among peers and for
    /// subscribing to information about collections, which are topics in the network.
    pub gossipsub: Gossipsub,
    /// Used for peer discovery and updates beyond the user-provided bootstrap list.
    pub identify: identify::Behaviour,
    /// Used for adding ping/pong functionality to the node.
    pub ping: ping::Behaviour,
    /// Used for automatic peer discovery in the local network.
    pub mdns: mdns::tokio::Behaviour,
}

impl SeaportGossipBehaviour {
    /// Publish data over the gossip network.
    pub fn publish(
        &mut self,
        topic: Topic,
        data: impl Into<Vec<u8>>,
    ) -> Result<MessageId, PublishError> {
        self.gossipsub.publish(topic, data)
    }

    /// Subscribe to a gossip topic.
    pub fn subscribe(&mut self, topic: &Topic) -> Result<bool, SubscriptionError> {
        self.gossipsub.subscribe(topic)
    }
}

/// A Quay Gossip node, which can join the Seaport Gossip network
/// for interacting with the P2P seaport market.
pub struct QuayGossipNode {
    /// The state of the network observed by the local node.
    pub swarm: Swarm<SeaportGossipBehaviour>,
    /// The node's peer ID.
    pub local_peer_id: PeerId,
    /// The node settings.
    pub config: GossipNodeSettings,
}

impl QuayGossipNode {
    /// Creates a new node instance from a Keypair.
    pub fn new(keypair: Keypair, config: GossipNodeSettings) -> Result<QuayGossipNode> {
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

        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .expect("Config should be valid");

        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(keypair.clone()),
            gossipsub_config,
        )
        .expect("Correct configuration");

        let swarm = {
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default())?;
            let behaviour = SeaportGossipBehaviour {
                gossipsub,
                mdns,
                ping: Default::default(),
                identify: identify::Behaviour::new(identify::Config::new(
                    PROTOCOL_ID.into(),
                    keypair.public(),
                )),
            };
            Swarm::with_tokio_executor(transport, behaviour, peer_id)
        };

        Ok(QuayGossipNode {
            swarm,
            local_peer_id: peer_id,
            config,
        })
    }

    /// Starts the node.
    pub async fn run(mut self) -> Result<()> {
        info!("Starting Quay Gossip Client");

        self.swarm.listen_on(
            format!("/ip4/{}/tcp/{}", self.config.host_name, self.config.port).parse()?,
        )?;

        self.swarm
            .behaviour_mut()
            .subscribe(&Topic::new("gossipsub:message"))?;
        info!("Local peer ID: {}", self.local_peer_id);

        loop {
            tokio::select! {
                    event = self.swarm.select_next_some() => {
                        println!("{:?}", event);
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
                                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                                            info!("Added peer {}", peer);
                                        }
                                    }
                                    mdns::Event::Expired(list) => {
                                        for (peer, _) in list {
                                            if !self.swarm.behaviour().mdns.has_node(&peer) {
                                                self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer);
                                                info!("Removed peer {}", peer);
                                            }
                                        }
                                    }
                                }
                            }
                            SwarmEvent::Behaviour(SeaportGossipBehaviourEvent::Identify(_)) => {},
                            _ => {}

                    }
                }
            }
        }
    }
}
