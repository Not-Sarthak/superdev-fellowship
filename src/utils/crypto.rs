use anyhow::{anyhow, Result};
use base58::{FromBase58};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

const MAX_MESSAGE_LENGTH: usize = 10_000;
const MIN_MESSAGE_LENGTH: usize = 1;

pub fn parse_secret_key(secret_str: &str) -> Result<Keypair> {
    if secret_str.is_empty() {
        return Err(anyhow!("Secret key string cannot be empty"));
    }
    
    if secret_str.len() > 200 {
        return Err(anyhow!("Secret key string too long (max 200 characters)"));
    }
    
    let bytes = secret_str
        .from_base58()
        .map_err(|e| anyhow!("Invalid base58 encoding: {:?}", e))?;

    if bytes.len() != 64 {
        return Err(anyhow!(
            "Secret Key Must be 64 Bytes, got {} Bytes", 
            bytes.len()
        ));
    }

    let mut secret_bytes = [0u8; 32];
    secret_bytes.copy_from_slice(&bytes[..32]);

    let secret_key = SecretKey::from_bytes(&secret_bytes)
        .map_err(|e| anyhow!("Invalid secret key: {}", e))?;

    let mut public_bytes = [0u8; 32];
    public_bytes.copy_from_slice(&bytes[32..]);

    let public_key = PublicKey::from_bytes(&public_bytes)
        .map_err(|e| anyhow!("Invalid public key: {}", e))?;

    let keypair = Keypair {
        secret: secret_key,
        public: public_key,
    };

    Ok(keypair)
}

pub fn sign_message(message: &str, keypair: &Keypair) -> Result<String> {
    validate_message(message)?;
    
    let message_bytes = message.as_bytes();
    let signature = keypair.sign(message_bytes);
    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

pub fn verify_signature(message: &str, signature_str: &str, pubkey_str: &str) -> Result<bool> {
    validate_message(message)?;
    validate_signature_string(signature_str)?;
    validate_pubkey_string(pubkey_str)?;
    
    let pubkey_bytes = pubkey_str
        .from_base58()
        .map_err(|e| anyhow!("Invalid public key encoding: {:?}", e))?;

    if pubkey_bytes.len() != 32 {
        return Err(anyhow!(
            "Public key must be 32 bytes, got {} bytes", 
            pubkey_bytes.len()
        ));
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&pubkey_bytes);

    let public_key = PublicKey::from_bytes(&key_bytes)
        .map_err(|e| anyhow!("Invalid public key: {}", e))?;

    let signature_bytes = general_purpose::STANDARD
        .decode(signature_str)
        .map_err(|e| anyhow!("Invalid signature encoding: {:?}", e))?;

    if signature_bytes.len() != 64 {
        return Err(anyhow!(
            "Signature must be 64 bytes, got {} bytes", 
            signature_bytes.len()
        ));
    }

    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(&signature_bytes);

    let signature = Signature::from_bytes(&sig_bytes)
        .map_err(|e| anyhow!("Invalid signature: {}", e))?;

    // Verify
    let message_bytes = message.as_bytes();
    Ok(public_key.verify(message_bytes, &signature).is_ok())
}

fn validate_message(message: &str) -> Result<()> {
    if message.is_empty() {
        return Err(anyhow!("Message cannot be empty"));
    }
    
    if message.len() < MIN_MESSAGE_LENGTH {
        return Err(anyhow!(
            "Message too short. Minimum length: {} characters", 
            MIN_MESSAGE_LENGTH
        ));
    }
    
    if message.len() > MAX_MESSAGE_LENGTH {
        return Err(anyhow!(
            "Message too long. Maximum length: {} characters", 
            MAX_MESSAGE_LENGTH
        ));
    }
    
    if message.contains('\0') {
        return Err(anyhow!("Message cannot contain null bytes"));
    }
    
    Ok(())
}

fn validate_signature_string(signature_str: &str) -> Result<()> {
    if signature_str.is_empty() {
        return Err(anyhow!("Signature string cannot be empty"));
    }
    
    if signature_str.len() > 200 {
        return Err(anyhow!("Signature string too long (max 200 characters)"));
    }
    
    Ok(())
}

/// Validate public key string format
fn validate_pubkey_string(pubkey_str: &str) -> Result<()> {
    if pubkey_str.is_empty() {
        return Err(anyhow!("Public key string cannot be empty"));
    }
    
    if pubkey_str.len() > 100 {
        return Err(anyhow!("Public key string too long (max 100 characters)"));
    }
    
    Ok(())
}