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

pub const PROTOCOL_ID: &str = "seaport/0.0.1";

#[derive(NetworkBehaviour)]
pub struct SeaportGossipBehaviour {
    pub gossipsub: Gossipsub,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
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

pub struct QuayGossipNode {
    pub swarm: Swarm<SeaportGossipBehaviour>,
    pub local_peer_id: PeerId,
}

impl QuayGossipNode {
    pub fn new(keypair: Keypair) -> Result<QuayGossipNode> {
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
        })
    }

    pub async fn run(mut self, config: GossipNodeSettings) -> Result<()> {
        info!("Starting Quay Gossip Client");
        self.swarm
            .listen_on(format!("/ip4/{}/tcp/{}", config.host_name, config.port).parse()?)?;

        loop {
            tokio::select! {
                    event = self.swarm.select_next_some() => {
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
                                        }
                                    }
                                    mdns::Event::Expired(list) => {
                                        for (peer, _) in list {
                                            if !self.swarm.behaviour().mdns.has_node(&peer) {
                                                self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer);
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
