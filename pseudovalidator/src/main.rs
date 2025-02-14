use tokio::main;
use log::{info, error};
use pseudovalidator::{config::load_config, networking::start_networking, quorum::check_quorum};
use std::error::Error;

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

    info!("Pseudovalidator is running successfully.");
    Ok(())
}
