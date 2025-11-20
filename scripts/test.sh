#!/bin/bash

CONTRACT_ID=$(cat contracts/blocks-resources.txt)

echo "🧪 Testing Block's Resources contract..."

# Register a resource
echo "Registering resource..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source alice \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase 'Test SDF Network ; September 2015' \
    -- \
    register_resource \
    --id "WOOD001" \
    --name "Sustainable Wood" \
    --resource_type "Wood" \
    --quantity 1000 \
    --origin "Amazon" \
    --carbon_footprint 50 \
    --owner GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH

# Create transfer
echo "Creating transfer..."
TX_ID=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --source alice \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase 'Test SDF Network ; September 2015' \
    -- \
    create_transfer \
    --resource_id "WOOD001" \
    --from GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH \
    --to GDIK733AD4V2CDQMMZGXLFGNM7W3234IORXIGVT2BCHSUDAWI42BRD3J \
    --quantity 500)

echo "Transfer created with ID: $TX_ID"

# Sign transaction (first signature)
echo "Adding first signature..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source alice \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase 'Test SDF Network ; September 2015' \
    -- \
    sign_transaction \
    --tx_id 1 \
    --signer GBMY6UIHGFIPM2ZWXBB5U7AJTSASWL7BB4PVYVNAZZQYEUQU3UYJOWYH

echo "✅ Test completed! Transfer pending second signature."
