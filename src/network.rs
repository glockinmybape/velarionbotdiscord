use libp2p::{Swarm, TcpConfig, Transport, Multiaddr, identity};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_node(storage: Arc<BlockchainStorage>) {
    let transport = TcpConfig::new();
    let mut swarm = Swarm::with_tokio_executor(transport, (), identity::ed25519::Keypair::generate());

    // Подключение к узлам
    swarm.dial(Multiaddr::from("/ip4/127.0.0.1/tcp/30333")).unwrap();

    loop {
        match swarm.select_next_some().await {
            Ok(event) => {
                if let libp2p::swarm::Event::NewListenAddr { address, .. } = event {
                    println!("Слушаем на {}", address);
                }
            }
            Err(e) => eprintln!("Ошибка сети: {}", e),
        }
    }
}