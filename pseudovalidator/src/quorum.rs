use std::collections::HashMap;
use log::{info, error};
use std::error::Error;
use crate::networking::Network;
use tokio::sync::mpsc;

pub struct QuorumChecker {
    network: Network,
    quorum_threshold: f64,
}

impl QuorumChecker {
    pub fn new(network: Network, quorum_threshold: f64) -> Self {
        Self { network, quorum_threshold }
    }

    pub async fn check_quorum(&self, votes: &HashMap<String, bool>) -> Result<bool, Box<dyn Error>> {
        let total_votes = votes.len() as f64;
        let positive_votes = votes.values().filter(|&&vote| vote).count() as f64;

        if total_votes == 0.0 {
            error!("No votes received, cannot determine quorum.");
            return Err("No votes available to check quorum.".into());
        }

        let quorum_met = (positive_votes / total_votes) >= self.quorum_threshold;
        
        info!("Quorum check: {} out of {} votes are positive (Threshold: {}%), quorum met: {}", 
            positive_votes, total_votes, self.quorum_threshold * 100.0, quorum_met
        );
        
        Ok(quorum_met)
    }
    
    pub async fn broadcast_vote(&self, peer: &str, vote: bool) -> Result<(), Box<dyn Error>> {
        let message = format!("VOTE:{}", vote);
        self.network.send_message(peer, &message).await
    }
}