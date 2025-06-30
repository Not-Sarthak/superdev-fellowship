use axum::Json;
use base58::ToBase58;

use crate::types::*;
use crate::utils::{parse_secret_key, sign_message as utils_sign_message, verify_signature};

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> ApiResponse<SignMessageResponse> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return ApiResponse::error("Missing required fields".to_string());
    }

    let keypair = match parse_secret_key(&payload.secret) {
        Ok(kp) => kp,
        Err(e) => return ApiResponse::error(format!("Invalid secret key: {}", e)),
    };

    let signature = match utils_sign_message(&payload.message, &keypair) {
        Ok(sig) => sig,
        Err(e) => return ApiResponse::error(format!("Failed to sign message: {}", e)),
    };
    let public_key = keypair.public.to_bytes().to_base58();

    let response = SignMessageResponse {
        signature,
        public_key,
        message: payload.message,
    };

    ApiResponse::success(response)
}

pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> ApiResponse<VerifyMessageResponse> {
    let valid = match verify_signature(&payload.message, &payload.signature, &payload.pubkey) {
        Ok(is_valid) => is_valid,
        Err(e) => {
            return ApiResponse::error(format!("Verification failed: {}", e))
        }
    };

    let response = VerifyMessageResponse {
        valid,
        message: payload.message,
        pubkey: payload.pubkey,
    };

    ApiResponse::success(response)
} 