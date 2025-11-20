#!/bin/bash
set -e

echo "🚀 Setting up Block's Resources development environment..."

# Install Stellar CLI
curl -L https://github.com/stellar/stellar-cli/releases/latest/download/stellar-cli-linux-x64.tar.gz | tar xz
sudo mv stellar /usr/local/bin/

# Install Soroban CLI (latest version)
cargo install --locked soroban-cli

# Install Node.js dependencies for frontend
cd frontend && npm install && cd ..

# Setup Rust environment
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-contract

# Create wallets directory
mkdir -p wallets

echo "✅ Setup complete! Environment ready for Block's Resources development."
