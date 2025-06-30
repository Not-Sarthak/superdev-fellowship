# Solana HTTP Server

A clean, modular HTTP server built with Axum for Solana blockchain operations including wallet management, token operations, and message signing.

## Features

- **Wallet Operations**: Generate keypairs, send SOL
- **Token Operations**: Create tokens, mint tokens, transfer tokens
- **Message Operations**: Sign and verify messages
- **Clean Architecture**: Organized by functionality with separation of concerns

## API Endpoints

- `POST /keypair` - Generate a new keypair
- `POST /token/create` - Create a new token
- `POST /token/mint` - Mint tokens to an account
- `POST /message/sign` - Sign a message
- `POST /message/verify` - Verify a message signature
- `POST /send/sol` - Send SOL transaction
- `POST /send/token` - Send token transaction

## Local Development

```bash
# Clone the repository
git clone <your-repo-url>
cd axum-server

# Run the server
cargo run

# Test endpoints
curl -X POST http://localhost:8080/keypair
```

## Deployment on Render

### Method 1: Using render.yaml (Recommended)

1. **Push to GitHub**:
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git branch -M main
   git remote add origin <your-github-repo-url>
   git push -u origin main
   ```

2. **Deploy on Render**:
   - Go to [Render Dashboard](https://dashboard.render.com)
   - Click "New" → "Web Service"
   - Connect your GitHub repository
   - Render will automatically detect the `render.yaml` file
   - Click "Deploy"

### Method 2: Manual Configuration

1. **Push to GitHub** (same as above)

2. **Create Web Service on Render**:
   - Go to [Render Dashboard](https://dashboard.render.com)
   - Click "New" → "Web Service"
   - Connect your GitHub repository
   - Configure:
     - **Name**: `axum-solana-server`
     - **Environment**: `Rust`
     - **Build Command**: `cargo build --release`
     - **Start Command**: `./target/release/axum-server`
     - **Environment Variables**:
       - `PORT`: `10000`
       - `RUST_LOG`: `info`

3. **Deploy**: Click "Create Web Service"

### Method 3: Docker Deployment

1. **Enable Docker** in Render service settings
2. Render will automatically use the provided `Dockerfile`

## Environment Variables

- `PORT`: Server port (default: 8080, Render uses 10000)
- `RUST_LOG`: Logging level (default: info)

## Project Structure

```
src/
├── handlers.rs          # Handler re-exports
├── main.rs             # Server entry point
├── types.rs            # Type definitions
├── utils/              # Utility modules
│   ├── mod.rs         # Utils re-exports
│   ├── crypto.rs      # Cryptographic operations
│   └── solana.rs      # Solana-specific utilities
├── message/           # Message operations
│   └── mod.rs
├── token/             # Token operations
│   └── mod.rs
└── wallet/            # Wallet operations
    └── mod.rs
```

## Example Usage

### Generate Keypair
```bash
curl -X POST https://your-app.onrender.com/keypair
```

### Sign Message
```bash
curl -X POST https://your-app.onrender.com/message/sign \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Hello, Solana!",
    "secret": "your_base58_encoded_secret_key"
  }'
```

## Production Considerations

- The server uses production-ready error handling
- All routes are properly organized by functionality
- Environment-based port configuration for deployment
- Clean separation of concerns for maintainability

## License

MIT
