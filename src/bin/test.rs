use libp2p::{
    development_transport,
    futures::StreamExt,
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, IdentTopic, MessageAuthenticity},
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    identity::Keypair,
    kad::{store::MemoryStore, Kademlia, KademliaEvent, QueryResult},
    swarm::SwarmEvent,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
struct Behav {
    kademlia: Kademlia<MemoryStore>,
    identify: Identify,
    gossipsub: Gossipsub,
}

enum Event {
    Kademlia(KademliaEvent),
    Identify(IdentifyEvent),
    Gossipsub(GossipsubEvent),
}

impl From<KademliaEvent> for Event {
    fn from(value: KademliaEvent) -> Self {
        Self::Kademlia(value)
    }
}

impl From<IdentifyEvent> for Event {
    fn from(value: IdentifyEvent) -> Self {
        Self::Identify(value)
    }
}

impl From<GossipsubEvent> for Event {
    fn from(value: GossipsubEvent) -> Self {
        Self::Gossipsub(value)
    }
}

#[tokio::main]
async fn main() {
    let keys = Keypair::generate_ed25519();
    let peerid = PeerId::from(keys.public());
    let topics = IdentTopic::new("test");
    println!(
        "your peerid is:\n{}\n**********************************",
        peerid
    );

    let transport = development_transport(keys.clone()).await.unwrap();
    let identify = Identify::new(IdentifyConfig::new(
        "/ipfs/mohammad/push/1.0.0".to_string(),
        keys.public().clone(),
    ));
    let store = MemoryStore::new(peerid.clone());
    let kademlia = Kademlia::new(peerid.clone(), store);
    let messageauthenticity = MessageAuthenticity::Signed(keys);
    let ghossipsubconfig = GossipsubConfig::default();
    let gossipsub: Gossipsub = Gossipsub::new(messageauthenticity, ghossipsubconfig).unwrap();
    let mut behaviour = Behav {
        kademlia,
        identify,
        gossipsub,
    };
    behaviour.gossipsub.subscribe(&topics.clone()).unwrap();
    let mut swarm = Swarm::new(transport, behaviour, peerid);
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    if let Some(addr) = std::env::args().nth(1) {
        let address: Multiaddr = addr.parse().unwrap();
        swarm.dial(address).unwrap();
        println!("dialing to: {}\n**********************************", addr);
    }

    // let mut peerstable = Vec::new();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!(
                    "your listener address:\n{}\n**********************************",
                    address
                );
            }
            SwarmEvent::Behaviour(Event::Identify(identifyevent)) => match identifyevent {
                IdentifyEvent::Received { peer_id, info } => {
                    for addr in info.listen_addrs {
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                    }
                    // println!("{}=>received\n**********************************", peer_id);
                    swarm.behaviour_mut().kademlia.bootstrap().unwrap();
                    swarm
                        .behaviour_mut()
                        .kademlia
                        .get_closest_peers(info.public_key.to_peer_id());
                }
                _ => (),
            },
            SwarmEvent::Behaviour(Event::Kademlia(kademliaevent)) => match kademliaevent {
                KademliaEvent::RoutingUpdated { peer, .. } => {
                    swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer.clone());
                    swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topics.clone(), "hello".as_bytes())
                        .unwrap();
                }
                KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                    QueryResult::GetClosestPeers(p) => match p {
                        Ok(peer) => println!(
                            "get closest:\n{:?}\n**********************************",
                            peer.peers
                        ),
                        Err(e) => println!("{}", e),
                    },
                    _ => (),
                },
                _ => (),
            },
            SwarmEvent::Behaviour(Event::Gossipsub(gossibsubevent)) => match gossibsubevent {
                GossipsubEvent::Message {
                    propagation_source,
                    message,
                    ..
                } => {
                    println!(
                        "you get:\n{}\nfrom this peer:\n{}\n**********************************",
                        String::from_utf8(message.data).unwrap(),
                        propagation_source
                    );
                }
                _ => (),
            },
            _ => (),
        }
    }
}
