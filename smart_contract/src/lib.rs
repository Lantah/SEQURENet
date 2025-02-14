//! Smart Contract for Decentralized Verification System
//! Handles vote aggregation, quorum verification, and publishing results on-chain.

#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address, BytesN};

#[contract]
pub struct VerificationContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteData {
    pub validator: Address,
    pub result: bool,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregatedVote {
    pub votes: Vec<VoteData>,
    pub consensus_result: bool,
}

#[contractimpl]
impl VerificationContract {
    /// Submit aggregated votes for verification and storage on-chain.
    pub fn submit_votes(env: Env, aggregated_vote: AggregatedVote) -> bool {
        let quorum_threshold: usize = 80; // Assume 80% quorum threshold
        let total_votes = aggregated_vote.votes.len();
        let mut positive_votes = 0;
        
        for vote in &aggregated_vote.votes {
            if vote.result {
                positive_votes += 1;
            }
        }
        
        let quorum_met = (positive_votes * 100 / total_votes) >= quorum_threshold;
        if quorum_met {
            env.storage().set(Symbol::new(&env, "last_verification"), &aggregated_vote);
        }
        quorum_met
    }
    
    /// Retrieve the last recorded verification result.
    pub fn get_last_verification(env: Env) -> Option<AggregatedVote> {
        env.storage().get(Symbol::new(&env, "last_verification"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Env as TestEnv, BytesN, Env};
    
    #[test]
    fn test_vote_submission() {
        let env = Env::default();
        let validator1 = Address::random(&env);
        let validator2 = Address::random(&env);
        
        let vote1 = VoteData {
            validator: validator1.clone(),
            result: true,
            signature: BytesN::from_array(&env, &[0u8; 64]),
        };
        
        let vote2 = VoteData {
            validator: validator2.clone(),
            result: true,
            signature: BytesN::from_array(&env, &[0u8; 64]),
        };
        
        let aggregated_vote = AggregatedVote {
            votes: vec![vote1, vote2],
            consensus_result: true,
        };
        
        let result = VerificationContract::submit_votes(env.clone(), aggregated_vote.clone());
        assert!(result);
        
        let retrieved_vote = VerificationContract::get_last_verification(env).unwrap();
        assert_eq!(retrieved_vote, aggregated_vote);
    }
}
