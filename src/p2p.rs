use tokio::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    NewBlock(Vec<u8>),
    NewTransaction(Vec<u8>),
    GetChain,
    Chain(Vec<Vec<u8>>),
    Ping,
    Pong,
}

pub struct P2PNode {
    listener: Option<TcpListener>,
    peers: HashMap<String, mpsc::Sender<Message>>,
    tx: mpsc::Sender<(String, Message)>,
    rx: mpsc::Receiver<(String, Message)>,
}

impl P2PNode {
    pub async fn new(port: u16) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
        let (tx, rx) = mpsc::channel(100);

        P2PNode {
            listener: Some(listener),
            peers: HashMap::new(),
            tx,
            rx,
        }
    }

    pub async fn connect_peer(&mut self, address: &str) -> Result<(), String> {
        let stream = TcpStream::connect(address).await.map_err(|e| e.to_string())?;
        let (reader, writer) = stream.split();

        let (peer_tx, peer_rx) = mpsc::channel(100);
        self.peers.insert(address.to_string(), peer_tx);

        tokio::spawn(Self::handle_incoming(reader, address.to_string(), self.tx.clone()));
        tokio::spawn(Self::handle_outgoing(writer, peer_rx));

        Ok(())
    }

    async fn handle_incoming(
        mut reader: impl AsyncReadExt + Unpin,
        peer_id: String,
        tx: mpsc::Sender<(String, Message)>,
    ) {
        let mut buffer = [0; 4096];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    if let Ok(message) = bincode::deserialize(&buffer[..n]) {
                        if let Err(e) = tx.send((peer_id.clone(), message)) {
                            eprintln!("Failed to send message from {}: {}", peer_id, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from {}: {}", peer_id, e);
                    break;
                }
            }
        }
    }

    async fn handle_outgoing(
        mut writer: impl AsyncWriteExt + Unpin,
        mut rx: mpsc::Receiver<Message>,
    ) {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = writer.write_all(&bincode::serialize(&msg).unwrap()).await {
                eprintln!("Error writing to peer: {}", e);
                break;
            }
        }
    }

    pub async fn broadcast_block(&self, block: &[u8]) {
        let message = Message::NewBlock(block.to_vec());
        for (peer, sender) in &self.peers {
            if let Err(e) = sender.send(message.clone()).await {
                eprintln!("Failed to send block to {}: {}", peer, e);
            }
        }
    }

    pub async fn broadcast_transaction(&self, tx: &[u8]) {
        let message = Message::NewTransaction(tx.to_vec());
        for (peer, sender) in &self.peers {
            if let Err(e) = sender.send(message.clone()).await {
                eprintln!("Failed to send transaction to {}: {}", peer, e);
            }
        }
    }

    pub async fn start_listening(&mut self) {
        if let Some(listener) = &self.listener {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let (reader, writer) = stream.split();
                    let peer_addr = stream.peer_addr().unwrap().to_string();

                    let tx_clone = self.tx.clone();
                    tokio::spawn(Self::handle_incoming(reader, peer_addr.clone(), tx_clone));
                    tokio::spawn(Self::handle_outgoing(writer, mpsc::channel(100).1));

                    self.peers.insert(peer_addr, mpsc::channel(100).0);
                }
            }
        }
    }

    pub async fn process_messages(&self) {
        while let Some((from, msg)) = self.rx.recv().await {
            match msg {
                Message::NewBlock(block) => {
                    println!("Received new block from {}: {:?}", from, block.len());
                }
                Message::NewTransaction(tx) => {
                    println!("Received new transaction from {}: {:?}", from, tx.len());
                }
                _ => {}
            }
        }
    }
}
