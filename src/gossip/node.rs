use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use anyhow::Result;
use ethers::types::H160;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    gossipsub::{
        self,
        error::{PublishError, SubscriptionError},
        Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
        MessageId, TopicHash, ValidationMode,
    },
    identify,
    identity::Keypair,
    mdns, mplex, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, PeerId, Swarm, Transport,
};
use tracing::{info, warn};

use crate::configuration::GossipNodeSettings;

use super::types::SeaportGossipsubEvent;

/// The seaport gossip network protocol ID & version.
pub const PROTOCOL_ID: &str = "seaport/0.1.0";

/// The gossipsub heartbeat interval.
pub const GOSSIPSUB_HEARTBEAT_INTERVAL: u64 = 10;

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

        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(GOSSIPSUB_HEARTBEAT_INTERVAL))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(QuayGossipNode::message_id_fn)
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

    /// The message ID function that will be use to dedupe gossipsub messages.
    ///
    /// Exposed as an associated function for ease of access.
    pub fn message_id_fn(message: &GossipsubMessage) -> MessageId {
        // TODO: Use Seaport's gossip message ID fn
        // see: https://github.com/ProjectOpenSea/seaport-gossip/blob/main/src/util/serialize.ts#L18-L29
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    }

    /// Starts the node.
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting Quay Gossip Client");

        self.swarm.listen_on(
            format!("/ip4/{}/tcp/{}", self.config.host_name, self.config.port).parse()?,
        )?;

        self.swarm
            .behaviour_mut()
            .subscribe(&Topic::new("gossipsub:message"))?;

        let collection_addresses = self.config.collection_addresses.clone().unwrap_or(vec![]);

        info!("Local peer ID: {}", self.local_peer_id);

        for address in collection_addresses.iter() {
            match self
                .swarm
                .behaviour_mut()
                .subscribe(&Topic::new(hex::encode(address)))
            {
                Ok(_) => info!("Successfully subscribed to collection/topic {}", address),
                Err(e) => warn!("Subscription/topic err: {}", e.to_string()),
            }
        }

        loop {
            tokio::select! {
                    event = self.swarm.select_next_some() => {
                        println!("{:?}", event);
                        match event {
                            SwarmEvent::NewListenAddr { address, .. } => {
                                info!("Listening on {address:?}");
                            }
                            SwarmEvent::Behaviour(SeaportGossipBehaviourEvent::Gossipsub(GossipsubEvent::Message{ message, .. })) => {
                                self.on_gossipsub_message(message);
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

    fn on_gossipsub_message(&self, message: GossipsubMessage) {
        info!(
            "Received: '{:?}' from {:?}",
            String::from_utf8_lossy(&message.data),
            message.source
        );
    }

    /// Publishes a seaport gossip event to the P2P network.
    pub fn publish_gossipsup_message(
        &mut self,
        event: SeaportGossipsubEvent,
    ) -> Result<(), anyhow::Error> {
        let mut addresses: Vec<H160> = event
            .order
            .offer
            .iter()
            .map(|offer| H160::from_slice(offer.token.as_slice()))
            .collect();
        let mut consideration_addresses: Vec<H160> = event
            .order
            .consideration
            .iter()
            .map(|consideration| H160::from_slice(consideration.offer.token.as_slice()))
            .collect();
        addresses.append(&mut consideration_addresses);
        addresses.sort();
        addresses.dedup();
        let serialized_event = ssz_rs::serialize(&event)?;
        // TODO: Look into parallelizing, probably with rayon
        for address in addresses {
            self.swarm.behaviour_mut().gossipsub.publish(
                TopicHash::from_raw(address.clone().to_string()),
                serialized_event.clone(),
            )?;
        }

        Ok(())
    }
}
