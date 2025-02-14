//! Unit tests for Pseudovalidator Configuration Loading

use std::fs;
use pseudovalidator::config::{Config, is_valid_scp_public_key};

#[test]
fn test_load_valid_config() {
    let config_content = r#"
        validator_id: "validator_123"
        public_key: "GABCDEF12345678901234567890123456789012345678901234567890"
        trusted_validators:
          - validator_id: "validator_456"
            public_key: "GXYZ7890123456789012345678901234567890123456789012345678"
        quorum_threshold: 0.8
        external_verifier: "./test_verifier"
    "#;
    
    let config_path = "./test_valid_config.yaml";
    fs::write(config_path, config_content).expect("Failed to write test config");
    
    let config = Config::load(config_path).expect("Failed to load config");
    
    assert_eq!(config.validator_id, "validator_123");
    assert!(is_valid_scp_public_key(&config.public_key));
    assert_eq!(config.trusted_validators.len(), 1);
    assert_eq!(config.trusted_validators[0].validator_id, "validator_456");
    assert!(is_valid_scp_public_key(&config.trusted_validators[0].public_key));
    assert_eq!(config.quorum_threshold, 0.8);
    assert_eq!(config.external_verifier, "./test_verifier");
    
    fs::remove_file(config_path).expect("Failed to clean up test config file");
}

#[test]
fn test_load_invalid_config() {
    let config_content = r#"
        validator_id: "validator_123"
        public_key: "INVALID123456"  # Invalid format
        trusted_validators:
          - validator_id: "validator_456"
            public_key: "GXYZ7890123456789012345678901234567890123456789012345678"
        quorum_threshold: "high"  # Invalid type
        external_verifier: 12345  # Invalid type
    "#;
    
    let config_path = "./test_invalid_config.yaml";
    fs::write(config_path, config_content).expect("Failed to write test config");
    
    let config_result = Config::load(config_path);
    assert!(config_result.is_err(), "Invalid config should fail to load");
    
    fs::remove_file(config_path).expect("Failed to clean up test config file");
}

#[test]
fn test_load_missing_external_verifier() {
    let config_content = r#"
        validator_id: "validator_123"
        public_key: "GABCDEF12345678901234567890123456789012345678901234567890"
        trusted_validators:
          - validator_id: "validator_456"
            public_key: "GXYZ7890123456789012345678901234567890123456789012345678"
        quorum_threshold: 0.8
    "#;
    
    let config_path = "./test_missing_external_verifier.yaml";
    fs::write(config_path, config_content).expect("Failed to write test config");
    
    let config = Config::load(config_path).expect("Failed to load config");
    
    assert_eq!(config.validator_id, "validator_123");
    assert!(is_valid_scp_public_key(&config.public_key));
    assert_eq!(config.trusted_validators.len(), 1);
    assert_eq!(config.trusted_validators[0].validator_id, "validator_456");
    assert!(is_valid_scp_public_key(&config.trusted_validators[0].public_key));
    assert_eq!(config.quorum_threshold, 0.8);
    assert!(config.external_verifier.is_empty() || config.external_verifier == "./external_verifier");
    
    fs::remove_file(config_path).expect("Failed to clean up test config file");
}