use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use anyhow::Result;
use libp2p::{
    core::upgrade,
    gossipsub::{
        self, Gossipsub, GossipsubMessage, MessageAuthenticity, MessageId, ValidationMode,
    },
    identity::Keypair,
    mdns, mplex, noise,
    swarm::NetworkBehaviour,
    tcp, PeerId, Swarm, Transport,
};

pub const PROTOCOL_ID: &str = "/seaport/0.0.1";

#[derive(NetworkBehaviour)]
pub struct SeaportGossipBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: mdns::tokio::Behaviour,
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

        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
            .expect("Correct configuration");

        let swarm = {
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default())?;
            let behaviour = SeaportGossipBehaviour { gossipsub, mdns };
            Swarm::with_tokio_executor(transport, behaviour, peer_id)
        };

        Ok(QuayGossipNode {
            swarm,
            local_peer_id: peer_id,
        })
    }
}
