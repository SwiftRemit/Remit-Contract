# ⚡ SwiftRemit — Soroban Smart Contract

A Soroban smart contract on the Stellar network for cross-border payments.

## Tech Stack

- **Rust** — contract language
- **Soroban SDK 22** — smart contract framework
- **Stellar CLI** — build, deploy, invoke
- **WASM** — compiled target (`wasm32-unknown-unknown`)

## Contract Functions

| Function | Description |
|---|---|
| `initialize(admin, fee_bps)` | Deploy and configure the contract |
| `register(user, display_name)` | Register a user display name |
| `get_name(user)` | Get a user's display name |
| `send(from, to, token, amount, memo)` | Send a token payment |
| `tx_count(addr)` | Get total transaction count for an address |
| `get_txs(addr, limit)` | Get recent transactions for an address |
| `set_fee(new_fee_bps)` | Update protocol fee (admin only) |
| `transfer_admin(new_admin)` | Transfer admin role |
| `get_fee()` | Get current fee in basis points |

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli
```

## Getting Started

```bash
# 1. Build the contract
stellar contract build

# 2. Run tests
cargo test

# 3. Set up environment
cp .env.example .env
# Fill in your keypairs and token address

# 4. Deploy to testnet
bash scripts/deploy.sh

# 5. Invoke functions
bash scripts/invoke.sh
```

## Project Structure

```
Remit-Contract/
├── src/
│   └── lib.rs              # Contract logic + tests
├── scripts/
│   ├── build.sh            # Build to WASM
│   ├── deploy.sh           # Deploy + initialise on testnet
│   └── invoke.sh           # Example invocations
├── .stellar/
│   └── network.toml        # Testnet / mainnet RPC config
├── Cargo.toml              # Rust dependencies
└── .env.example            # Environment variables template
```

## Payment Flow

1. Caller invokes `send(from, to, token, amount, memo)`
2. Contract deducts protocol fee (`fee_bps / 10000 * amount`)
3. Net amount transferred to recipient via SAC token
4. Fee transferred to admin address
5. `TxRecord` stored on-chain for both sender and recipient
6. `send` event emitted

## Fee Structure

Fees are set in **basis points** (1 bps = 0.01%):
- Default: `10 bps` = **0.1%**
- Configurable by admin via `set_fee()`

## GitHub

[https://github.com/SwiftRemit/Remit-Contract](https://github.com/SwiftRemit/Remit-Contract)
