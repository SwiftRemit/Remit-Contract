#!/usr/bin/env bash
# Deploy the contract to Stellar testnet
set -e

source .env

echo "🚀 Deploying SwiftRemit contract to testnet..."

CONTRACT_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/remit_contract.wasm \
  --source "$DEPLOYER_SECRET" \
  --network testnet)

echo "✅ Contract deployed!"
echo "   Contract ID: $CONTRACT_ID"

# Save contract ID to .env
echo "" >> .env
echo "CONTRACT_ID=$CONTRACT_ID" >> .env

echo ""
echo "🔧 Initialising contract..."

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$DEPLOYER_SECRET" \
  --network testnet \
  -- initialize \
  --admin "$ADMIN_PUBLIC_KEY" \
  --fee_bps 10

echo "✅ Contract initialised with 0.1% fee"
echo "   Admin: $ADMIN_PUBLIC_KEY"
