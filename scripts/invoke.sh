#!/usr/bin/env bash
# Example invocations for the deployed contract
set -e

source .env

echo "📋 SwiftRemit Contract Invocation Examples"
echo "Contract: $CONTRACT_ID"
echo ""

# ── Register a user ──────────────────────────────────────────────────────
echo "1️⃣  Registering user..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SENDER_SECRET" \
  --network testnet \
  -- register \
  --user "$SENDER_PUBLIC_KEY" \
  --display_name "Alice"

# ── Get user name ─────────────────────────────────────────────────────────
echo ""
echo "2️⃣  Getting user name..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network testnet \
  -- get_name \
  --user "$SENDER_PUBLIC_KEY"

# ── Send payment ──────────────────────────────────────────────────────────
echo ""
echo "3️⃣  Sending payment..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SENDER_SECRET" \
  --network testnet \
  -- send \
  --from "$SENDER_PUBLIC_KEY" \
  --to "$RECIPIENT_PUBLIC_KEY" \
  --token "$TOKEN_ADDRESS" \
  --amount 100000000 \
  --memo "SwiftRemit payment"

# ── Get transaction history ───────────────────────────────────────────────
echo ""
echo "4️⃣  Getting transaction history..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network testnet \
  -- get_txs \
  --addr "$SENDER_PUBLIC_KEY" \
  --limit 10

# ── Get fee ───────────────────────────────────────────────────────────────
echo ""
echo "5️⃣  Current fee (basis points)..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network testnet \
  -- get_fee
