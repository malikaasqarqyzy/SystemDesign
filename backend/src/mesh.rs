use crate::models::SyncBlock;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    noise,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct MeshBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<MeshEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeshEvent {
    NewBlock(SyncBlock),
    BlockRequest {
        from_peer: PeerId,
        since: chrono::DateTime<chrono::Utc>,
    },
    BlockResponse {
        from_peer: PeerId,
        blocks: Vec<SyncBlock>,
    },
}

#[derive(Clone)]
pub struct Network {
    swarm: Swarm<MeshBehaviour>,
    event_sender: mpsc::UnboundedSender<MeshEvent>,
    event_receiver: mpsc::UnboundedReceiver<MeshEvent>,
    peers: HashSet<PeerId>,
}

impl Network {
    pub fn new() -> Result<Self> {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&id_keys)
            .expect("Signing libp2p-noise static DH keypair failed.");

        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(libp2p::yamux::YamuxConfig::default())
            .boxed();

        let (response_sender, event_receiver) = mpsc::unbounded_channel();
        let event_sender = response_sender.clone();

        let mut behaviour = MeshBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns: Mdns::new(Default::default())?,
            response_sender,
        };

        // Subscribe to the sync topic
        let topic = Topic::new("sync");
        behaviour.floodsub.subscribe(topic);

        let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Ok(Self {
            swarm,
            event_sender,
            event_receiver,
            peers: HashSet::new(),
        })
    }

    pub async fn start(&mut self, addr: &str) -> Result<()> {
        let multiaddr = addr.parse()?;
        self.swarm.listen_on(multiaddr)?;
        
        loop {
            tokio::select! {
                event = self.swarm.next() => {
                    match event {
                        Some(e) => self.handle_swarm_event(e).await?,
                        None => break,
                    }
                }
                event = self.event_receiver.recv() => {
                    match event {
                        Some(e) => self.handle_mesh_event(e).await?,
                        None => break,
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_swarm_event(&mut self, event: libp2p::swarm::SwarmEvent) -> Result<()> {
        match event {
            // Handle different swarm events
            _ => Ok(()),
        }
    }

    async fn handle_mesh_event(&mut self, event: MeshEvent) -> Result<()> {
        match event {
            MeshEvent::NewBlock(block) => {
                // Broadcast new block to peers
                let message = serde_json::to_string(&MeshEvent::NewBlock(block))?;
                self.swarm
                    .behaviour_mut()
                    .floodsub
                    .publish(Topic::new("sync"), message.as_bytes());
            }
            MeshEvent::BlockRequest { from_peer, since } => {
                // Handle block request from peer
                // TODO: Implement block retrieval and response
            }
            MeshEvent::BlockResponse { from_peer, blocks } => {
                // Handle block response from peer
                // TODO: Implement block validation and storage
            }
        }
        Ok(())
    }

    pub async fn broadcast_block(&mut self, block: SyncBlock) -> Result<()> {
        self.event_sender
            .send(MeshEvent::NewBlock(block))
            .map_err(|e| anyhow!("Failed to send block: {}", e))?;
        Ok(())
    }

    pub async fn request_blocks(&mut self, peer: PeerId, since: chrono::DateTime<chrono::Utc>) -> Result<()> {
        self.event_sender
            .send(MeshEvent::BlockRequest { from_peer: peer, since })
            .map_err(|e| anyhow!("Failed to send block request: {}", e))?;
        Ok(())
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MeshBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MeshBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            if let Ok(mesh_event) = serde_json::from_slice::<MeshEvent>(&message.data) {
                self.response_sender
                    .send(mesh_event)
                    .expect("Failed to forward mesh event");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_network_creation() {
        let network = Network::new().unwrap();
        assert!(!network.peers.is_empty());
    }

    #[tokio::test]
    async fn test_block_broadcast() {
        let mut network = Network::new().unwrap();
        
        let block = SyncBlock {
            id: Uuid::new_v4(),
            previous_hash: "test_hash".to_string(),
            timestamp: Utc::now(),
            node_id: "test_node".to_string(),
            records: vec![],
            signature: "test_signature".to_string(),
        };

        network.broadcast_block(block).await.unwrap();
    }
} 