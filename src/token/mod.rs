use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::{initialize_mint, mint_to, transfer};

use crate::types::*;
use crate::utils::parse_pubkey;

pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> ApiResponse<CreateTokenResponse> {
    let mint_authority = match parse_pubkey(&payload.mint_authority) {
        Ok(pk) => pk,
        Err(e) => {
            return ApiResponse::error(format!("Invalid mint authority: {}", e))
        }
    };

    let mint = match parse_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid mint: {}", e)),
    };



    let instruction = match initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        payload.decimals,
    ) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Failed to create initialize mint instruction: {}", e);
            return ApiResponse::error("Failed to create initialize mint instruction".to_string());
        }
    };

    let accounts = instruction
        .accounts
        .iter()
        .map(|acc| InstructionAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let response = CreateTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ApiResponse::success(response)
}

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> ApiResponse<InstructionResponse> {
    let mint = match parse_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid mint: {}", e)),
    };

    let destination = match parse_pubkey(&payload.destination) {
        Ok(pk) => pk,
        Err(e) => {
            return ApiResponse::error(format!("Invalid destination: {}", e))
        }
    };

    let authority = match parse_pubkey(&payload.authority) {
        Ok(pk) => pk,
        Err(e) => {
            return ApiResponse::error(format!("Invalid authority: {}", e))
        }
    };

    let instruction = match mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    ) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Failed to create mint to instruction: {}", e);
            return ApiResponse::error("Failed to create mint to instruction".to_string());
        }
    };

    let accounts = instruction
        .accounts
        .iter()
        .map(|acc| InstructionAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ApiResponse::success(response)
}

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> ApiResponse<SendTokenResponse> {
    let destination = match parse_pubkey(&payload.destination) {
        Ok(pk) => pk,
        Err(e) => {
            return ApiResponse::error(format!("Invalid destination: {}", e))
        }
    };

    let mint = match parse_pubkey(&payload.mint) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid mint: {}", e)),
    };

    let owner = match parse_pubkey(&payload.owner) {
        Ok(pk) => pk,
        Err(e) => return ApiResponse::error(format!("Invalid owner: {}", e)),
    };

    let source = get_associated_token_address(&owner, &mint);
    let dest = get_associated_token_address(&destination, &mint);

    let instruction = match transfer(
        &spl_token::id(),
        &source,
        &dest,
        &owner,
        &[],
        payload.amount,
    ) {
        Ok(inst) => inst,
        Err(e) => {
            eprintln!("Failed to create transfer instruction: {}", e);
            return ApiResponse::error("Failed to create transfer instruction".to_string());
        }
    };

    let accounts = instruction
        .accounts
        .iter()
        .map(|acc| TokenAccount {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let response = SendTokenResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    ApiResponse::success(response)
} 