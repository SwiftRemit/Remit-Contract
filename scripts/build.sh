#!/usr/bin/env bash
# Build the Soroban contract to WASM
set -e

echo "🔨 Building SwiftRemit Soroban contract..."

stellar contract build

echo "✅ Build complete!"
echo "   Output: target/wasm32-unknown-unknown/release/remit_contract.wasm"
