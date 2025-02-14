use ed25519_dalek::{Keypair, Signature, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;
use std::error::Error;

pub fn generate_keypair() -> Keypair {
    let mut csprng = OsRng;
    Keypair::generate(&mut csprng)
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify_signature(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification() {
        let keypair = generate_keypair();
        let message = b"Test message";
        let signature = sign_message(&keypair, message);

        assert!(verify_signature(&keypair.public, message, &signature));
    }

    #[test]
    fn test_invalid_signature() {
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();
        let message = b"Test message";
        let signature = sign_message(&keypair1, message);

        assert!(!verify_signature(&keypair2.public, message, &signature));
    }
}
