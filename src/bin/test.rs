use libp2p::{identity::Keypair, PeerId, development_transport, identify::{Identify, IdentifyConfig, IdentifyEvent}, Swarm, Multiaddr, futures::StreamExt, swarm::SwarmEvent};

#[tokio::main]
async fn main() {
    let keys = Keypair::generate_ed25519();
    let peerid = PeerId::from(keys.public());
    println!("your peerid is:\n{}", peerid);
    
    let transport = development_transport(keys.clone()).await.unwrap();
    let behaviour = Identify::new(IdentifyConfig::new("/ipfs/mohammad/1.0.0".to_string(), keys.public().clone()));

    let mut swarm = Swarm::new(transport, behaviour, peerid);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    if let Some(addr) = std::env::args().nth(1) {
        let address: Multiaddr = addr.parse().unwrap();
        swarm.dial(address).unwrap();
        println!("dialing to: {}", addr);
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr {  address, .. } => {
                println!("your listener address:\n{}", address);
            },
            SwarmEvent::Behaviour(IdentifyEvent::Sent { peer_id }) => {
                println!("sent: {}", peer_id);
            },
            SwarmEvent::Behaviour(IdentifyEvent::Received { peer_id, info }) => {
                println!("receved:\n{}\n{:?}", peer_id, info);
            },
            _ => ()
        }
    }

}