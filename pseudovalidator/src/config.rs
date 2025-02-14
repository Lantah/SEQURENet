use serde::Deserialize;
use std::fs;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TrustedValidator {
    pub validator_id: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub validator_id: String,
    pub public_key: String,
    pub trusted_validators: Vec<TrustedValidator>,
    pub peer_address: String,
    pub quorum_threshold: f64,
}

pub fn is_valid_scp_public_key(key: &str) -> bool {
    key.len() == 56 && key.starts_with("G") && key.chars().all(|c| "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".contains(c))
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string("config/config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    if !is_valid_scp_public_key(&config.public_key) {
        return Err("Invalid public key format for validator".into());
    }

    for trusted_validator in &config.trusted_validators {
        if !is_valid_scp_public_key(&trusted_validator.public_key) {
            return Err(format!(
                "Invalid public key format for trusted validator: {}",
                trusted_validator.validator_id
            )
            .into());
        }
    }

    Ok(config)
}
