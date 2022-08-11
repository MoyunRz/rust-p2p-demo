use libp2p::{
    futures::StreamExt,identity,
    mdns::{Mdns,MdnsConfig,MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    ping::{Ping,PingConfig},
    PeerId
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("peer id:{:?} ", new_peer_id);
    // 创建一个行为，会与下面的 swarm 关联
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    // 利用密钥对创建传输
    let transport = libp2p::development_transport(new_key).await?;
    // 将参数传入，让swarm监听地址
    let mut swarm= Swarm::new(transport, behaviour, new_peer_id);
    // ip4 类型
    // 0.0.0.0 电脑上所有的ipv4类型
    // tcp 通信方式
    // 0 端口号，随机，可以指定
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 进行循环
    loop {
        // 监听器
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr{address,..} => {
                // 如果事件发生
                println!("Listener on Local Address {:?}", address)
            }
            SwarmEvent::Behaviour(
                MdnsEvent::Discovered(peers)
            ) => { 

                for (peer,addr) in peers {
                    println!("discovered {} {}",peer,addr)
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer,addr) in expired {
                    println!("expired {} {}",peer,addr)
                }
            }
            
            _ => {} //不发生, 什么事也不做
        }
    }
}
