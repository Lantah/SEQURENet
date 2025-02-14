use tokio::main;
use log::{info, error};
use pseudovalidator::{config::load_config, networking::start_networking, quorum::check_quorum};
use std::error::Error;
use reqwest::Client;
use soroban_sdk::{Env, Address, BytesN};
use serde_json::json;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    pretty_env_logger::init();
    info!("Starting pseudovalidator...");

    // Load configuration
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return Err(Box::new(e));
        }
    };

    // Start networking module
    if let Err(e) = start_networking(&config).await {
        error!("Networking error: {}", e);
        return Err(Box::new(e));
    }

    // Perform quorum check
    if let Err(e) = check_quorum().await {
        error!("Quorum verification failed: {}", e);
        return Err(Box::new(e));
    }

    // Submit result to blockchain
    if let Err(e) = submit_to_blockchain(true, &config).await {
        error!("Failed to submit result to blockchain: {}", e);
        return Err(Box::new(e));
    }

    info!("Pseudovalidator is running successfully.");
    Ok(())
}

async fn submit_to_blockchain(result: bool, config: &pseudovalidator::config::Config) -> Result<(), Box<dyn Error>> {
    let contract_address = &config.blockchain.contract_address;
    let network = &config.blockchain.network;
    
    let env = Env::default();
    let contract = Address::from_bytes(contract_address.as_bytes());
    let function = BytesN::from_slice("publish_result".as_bytes());
    
    let client = Client::new();
    let endpoint = match network.as_str() {
        "metriq-testnet" => "https://testnet.metriq.network",
        "metriq-mainnet" => "https://mainnet.metriq.network",
        _ => return Err("Invalid network specified".into()),
    };

    let tx_payload = json!({
        "contract": contract.to_string(),
        "function": function.to_string(),
        "args": {
            "result": result
        },
        "signer": config.keys.private_key
    });

    let response = client.post(format!("{}/tx/submit", endpoint))
        .json(&tx_payload)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Successfully submitted verification result to blockchain.");
    } else {
        error!("Failed to submit transaction: {:?}", response.text().await?);
    }

    Ok(())
}
