use axum::{routing::{get, post}, Router, Json};
use serde_json::{json, Value};
use std::env;
use std::net::SocketAddr;

mod handlers;
mod message;
mod token;
mod wallet;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    println!("Starting Solana HTTP Server...");

    let app = Router::new()
        .route("/keypair", post(handlers::generate_keypair))
        .route("/token/create", post(handlers::create_token))
        // .route("/token/mint", post(handlers::mint_token))
        .route("/message/sign", post(handlers::sign_message));
        // .route("/message/verify", post(handlers::verify_message))
        // .route("/send/sol", post(handlers::send_sol))
        // .route("/send/token", post(handlers::send_token));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
