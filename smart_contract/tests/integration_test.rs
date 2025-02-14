use smart_contract::lib;
use smart_contract::VoteData;
use smart_contract::verify_votes;
use std::collections::HashMap;

#[test]
fn test_vote_verification_success() {
    let mut votes = HashMap::new();
    votes.insert("validator_1".to_string(), true);
    votes.insert("validator_2".to_string(), true);
    votes.insert("validator_3".to_string(), true);

    let quorum = 2;
    let result = verify_votes(&votes, quorum);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_vote_verification_failure() {
    let mut votes = HashMap::new();
    votes.insert("validator_1".to_string(), true);
    votes.insert("validator_2".to_string(), false);
    votes.insert("validator_3".to_string(), false);

    let quorum = 2;
    let result = verify_votes(&votes, quorum);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn test_vote_verification_insufficient_votes() {
    let mut votes = HashMap::new();
    votes.insert("validator_1".to_string(), true);
    
    let quorum = 2;
    let result = verify_votes(&votes, quorum);
    
    assert!(result.is_err());
}
