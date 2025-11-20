#!/bin/bash

# Cargar Contract ID
CONTRACT_ID=$(cat contracts/blocks-resources.txt)

echo "🔧 Initializing Block's Resources contract..."

# Wallets (testnet)
WALLET1="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
WALLET2="GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH"
WALLET3="GDIK733AD4V2CDQMMZGXLFGNM7W3234IORXIGVT2BCHSUDAWI42BRD3J"

# Initialize multisig
soroban contract invoke \
    --id $CONTRACT_ID \
    --source alice \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase 'Test SDF Network ; September 2015' \
    -- \
    initialize \
    --signers "[\"$WALLET1\", \"$WALLET2\", \"$WALLET3\"]" \
    --required_signatures 2

echo "✅ Contract initialized with multisig (2/3)!"
