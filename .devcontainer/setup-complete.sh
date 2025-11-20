#!/bin/bash
set -e

echo "🚀 Block's Resources - Complete Setup"

# Install Stellar CLI latest
echo "Installing Stellar CLI..."
curl -L https://github.com/stellar/stellar-cli/releases/latest/download/stellar-cli-linux-x64.tar.gz | tar xz
sudo mv stellar /usr/local/bin/

# Install Rust targets
echo "Setting up Rust..."
rustup target add wasm32-unknown-unknown
rustup update

# Install Node.js tools
echo "Installing Node.js dependencies..."
cd frontend
npm install -g http-server live-server
npm install
cd ..

# Make scripts executable
chmod +x scripts/*.sh

# Create necessary directories
mkdir -p logs wallets deployment

# Setup GitHub CLI for CI
echo "Setting up GitHub CLI..."
gh auth setup-git

# Verify installations
echo "Verifying installations..."
stellar --version
rustc --version
node --version

echo "✅ Setup complete! Your environment is ready for Block's Resources development."
echo ""
echo "Next steps:"
echo "1. Run: ./scripts/deploy.sh"
echo "2. Run: ./scripts/initialize.sh"
echo "3. Start frontend: cd frontend && python -m http.server 8000"
echo ""
echo "For demo: ./scripts/setup-testnet.sh"
