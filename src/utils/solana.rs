//! Solana-specific utility functions
//! 
//! This module contains utilities for:
//! - Public key parsing and validation
//! - Solana address operations

use anyhow::{anyhow, Result};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Parse a string into a Solana Pubkey
pub fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey> {
    Pubkey::from_str(pubkey_str).map_err(|e| anyhow!("Invalid public key: {}", e))
} 