#!/bin/bash

echo "🚀 Deploying Block's Resources contract to testnet..."

# Build the contract
cd contracts/blocks-resources
soroban contract build

# Deploy
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/blocks_resources.wasm \
    --source alice \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase 'Test SDF Network ; September 2015')

echo "✅ Contract deployed successfully!"
echo "Contract ID: $CONTRACT_ID"

# Save contract ID
echo $CONTRACT_ID > ../../contracts/blocks-resources.txt
cd ../..
