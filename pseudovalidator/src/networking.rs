use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};
use std::error::Error;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::config::NetworkingConfig;
use crate::config::load_pseudovalidator_config;

pub struct Network {
    peers: Arc<Mutex<HashMap<String, String>>>, // Stores peer addresses
    config: NetworkingConfig,
}

impl Network {
    pub async fn new(config: NetworkingConfig) -> Result<Self, Box<dyn Error>> {
        let pseudo_config = load_pseudovalidator_config()?;
        let peer_list = pseudo_config.trusted_validators.iter()
            .map(|v| (v.public_key.clone(), v.public_key.clone()))
            .collect();
        
        Ok(Self {
            peers: Arc::new(Mutex::new(peer_list)),
            config,
        })
    }

    pub async fn start_server(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(&self.config.bind_address).await?;
        info!("Listening for peer connections on {}", &self.config.bind_address);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            let peers = self.peers.clone();
            info!("Accepted connection from {}", addr);

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                match socket.read(&mut buffer).await {
                    Ok(size) if size > 0 => {
                        let message = String::from_utf8_lossy(&buffer[..size]).to_string();
                        info!("Received data: {}", message);

                        if message.starts_with("PEER:") {
                            let peer_addr = message.replace("PEER:", "");
                            peers.lock().await.insert(peer_addr.clone(), peer_addr);
                            info!("Registered new peer: {}", peer_addr);
                        } else {
                            let _ = socket.write_all(b"Acknowledged").await;
                        }
                    }
                    Ok(_) => info!("Connection closed by peer"),
                    Err(e) => error!("Failed to read from socket: {}", e),
                }
            });
        }
    }

    pub async fn send_message(&self, peer: &str, message: &str) -> Result<(), Box<dyn Error>> {
        match TcpStream::connect(peer).await {
            Ok(mut stream) => {
                stream.write_all(message.as_bytes()).await?;
                info!("Sent message to {}", peer);
            }
            Err(e) => {
                error!("Failed to connect to peer {}: {}", peer, e);
            }
        }
        Ok(())
    }
}
