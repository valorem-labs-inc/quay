use libp2p::identity::Keypair;
use quay::configuration::GossipNodeSettings;
use quay::gossip::node::QuayGossipNode;

pub struct TestNode {
    pub keypair: Keypair,
    pub node: QuayGossipNode,
}

pub fn configure_node(name: String, bootstrap_list: Vec<String>) -> TestNode {
    let config: GossipNodeSettings = GossipNodeSettings {
        collection_addresses: None,
        seaport_bootnodes: Some(bootstrap_list),
        host_name: name,
        port: 0,
    };

    let keypair = Keypair::generate_ed25519();
    let mut node = QuayGossipNode::new(keypair.clone(), config).unwrap();

    TestNode { keypair, node }
}
