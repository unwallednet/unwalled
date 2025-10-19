use crate::consensus::Transaction;
use anyhow::Result;
use libp2p::{
    gossipsub,
    identity,
    mdns,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp,
    yamux,
    Swarm,
    SwarmBuilder,
};
use tokio::sync::mpsc;

pub struct NetworkManager {
    pub swarm: Swarm<MyBehaviour>,
    pub peer_id: libp2p::PeerId,
    tx_to_consensus: mpsc::Sender<Transaction>,
}

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

impl NetworkManager {
    pub async fn new(tx_to_consensus: mpsc::Sender<Transaction>) -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = libp2p::PeerId::from(local_key.public());

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1Lazy)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::Config::default();
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;
        let transaction_topic = gossipsub::IdentTopic::new("transactions");
        gossipsub.subscribe(&transaction_topic)?;

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;

        let behaviour = MyBehaviour { gossipsub, mdns };
        let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(Self {
            swarm,
            peer_id,
            tx_to_consensus,
        })
    }
}

pub async fn event_loop(mut network_manager: NetworkManager) -> Result<()> {
    loop {
        tokio::select! {
            event = network_manager.swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    log::info!("Network listening on {address}");
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        log::info!("mDNS discovered a new peer: {peer_id}");
                        network_manager.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                    if let Ok(tx) = serde_json::from_slice::<Transaction>(&message.data) {
                        log::info!("Received gossiped transaction, sending to consensus.");
                        if let Err(e) = network_manager.tx_to_consensus.send(tx).await {
                            log::error!("Failed to send transaction to consensus channel: {}", e);
                        }
                    }
                },
                _ => {}
            }
        }
    }
}