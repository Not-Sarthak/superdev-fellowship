//! Wallet and SOL transaction handlers
//! 
//! This module contains handlers for wallet operations:
//! - Generating new keypairs
//! - Sending SOL transactions

use axum::Json;
use base58::ToBase58;
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use solana_sdk::system_instruction;

use crate::types::*;
use crate::utils::parse_pubkey;

pub async fn generate_keypair() -> ApiResponse<KeypairResponse> {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);

    let mut secret_bytes = [0u8; 64];
    secret_bytes[..32].copy_from_slice(&keypair.secret.to_bytes());
    secret_bytes[32..].copy_from_slice(&keypair.public.to_bytes());

    let response = KeypairResponse {
        pubkey: keypair.public.to_bytes().to_base58(),
        secret: secret_bytes.to_base58(),
    };

    ApiResponse::success(response)
}

pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> ApiResponse<SendSolResponse> {
    let from = match parse_pubkey(&payload.from) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid from address: {}", e)),
    };

    let to = match parse_pubkey(&payload.to) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid to address: {}", e)),
    };



    let instruction = system_instruction::transfer(&from, &to, payload.lamports);

    let response = SendSolResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction
            .accounts
            .iter()
            .map(|acc| acc.pubkey.to_string())
            .collect(),
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ApiResponse::success(response)
} 