//! SwiftRemit — Soroban Smart Contract
//!
//! A cross-border payment contract on the Stellar network.
//! Supports:
//!   - Registering users with a display name
//!   - Sending token payments between addresses
//!   - Querying transaction history per address
//!   - Admin fee configuration

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, symbol_short,
    Address, Env, String, Vec,
};

// ─────────────────────────────────────────────
//  Storage keys
// ─────────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Admin,
    FeeBps,                    // fee in basis points (e.g. 10 = 0.1%)
    UserName(Address),
    TxCount(Address),
    Tx(Address, u64),          // (sender, index) → TxRecord
}

// ─────────────────────────────────────────────
//  Data types
// ─────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug)]
pub struct TxRecord {
    pub from:      Address,
    pub to:        Address,
    pub amount:    i128,
    pub token:     Address,
    pub timestamp: u64,
    pub memo:      String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct UserProfile {
    pub address:      Address,
    pub display_name: String,
}

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────
#[contract]
pub struct RemitContract;

#[contractimpl]
impl RemitContract {

    // ── Initialise ──────────────────────────────────────────────────────
    /// Deploy the contract. Sets the admin and initial fee (basis points).
    pub fn initialize(env: Env, admin: Address, fee_bps: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialised");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
    }

    // ── User registration ────────────────────────────────────────────────
    /// Register or update a display name for the caller.
    pub fn register(env: Env, user: Address, display_name: String) {
        user.require_auth();
        env.storage()
            .persistent()
            .set(&DataKey::UserName(user.clone()), &display_name);
        env.events().publish(
            (symbol_short!("register"), user),
            display_name,
        );
    }

    /// Get a user's display name. Returns empty string if not registered.
    pub fn get_name(env: Env, user: Address) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::UserName(user))
            .unwrap_or(String::from_str(&env, ""))
    }

    // ── Payments ─────────────────────────────────────────────────────────
    /// Send `amount` of `token` from `from` to `to`.
    /// A protocol fee (fee_bps / 10000) is deducted and sent to admin.
    /// Records the transaction on-chain for both parties.
    pub fn send(
        env:    Env,
        from:   Address,
        to:     Address,
        token:  Address,
        amount: i128,
        memo:   String,
    ) {
        from.require_auth();

        assert!(amount > 0, "amount must be positive");

        let fee_bps: u32 = env
            .storage()
            .instance()
            .get(&DataKey::FeeBps)
            .unwrap_or(0);

        let fee: i128 = (amount * fee_bps as i128) / 10_000;
        let net: i128 = amount - fee;

        let token_client = token::Client::new(&env, &token);

        // Transfer net amount to recipient
        token_client.transfer(&from, &to, &net);

        // Transfer fee to admin (if any)
        if fee > 0 {
            let admin: Address = env
                .storage()
                .instance()
                .get(&DataKey::Admin)
                .unwrap();
            token_client.transfer(&from, &admin, &fee);
        }

        let ts = env.ledger().timestamp();

        let record = TxRecord {
            from:      from.clone(),
            to:        to.clone(),
            amount,
            token:     token.clone(),
            timestamp: ts,
            memo:      memo.clone(),
        };

        // Store for sender
        Self::push_tx(&env, &from, record.clone());
        // Store for recipient
        Self::push_tx(&env, &to, record.clone());

        // Emit event
        env.events().publish(
            (symbol_short!("send"), from, to),
            (amount, token, memo),
        );
    }

    // ── Transaction history ──────────────────────────────────────────────
    /// Returns the total number of transactions for an address.
    pub fn tx_count(env: Env, addr: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::TxCount(addr))
            .unwrap_or(0)
    }

    /// Returns up to `limit` most recent transactions for `addr`.
    pub fn get_txs(env: Env, addr: Address, limit: u32) -> Vec<TxRecord> {
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::TxCount(addr.clone()))
            .unwrap_or(0);

        let mut results = Vec::new(&env);
        let take = (limit as u64).min(count);

        for i in (0..take).rev() {
            let idx = count - 1 - i;
            if let Some(record) = env
                .storage()
                .persistent()
                .get::<DataKey, TxRecord>(&DataKey::Tx(addr.clone(), idx))
            {
                results.push_back(record);
            }
        }
        results
    }

    // ── Admin ────────────────────────────────────────────────────────────
    /// Update the protocol fee. Admin only.
    pub fn set_fee(env: Env, new_fee_bps: u32) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::FeeBps, &new_fee_bps);
    }

    /// Transfer admin role to a new address.
    pub fn transfer_admin(env: Env, new_admin: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::Admin, &new_admin);
    }

    /// Get current fee in basis points.
    pub fn get_fee(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::FeeBps)
            .unwrap_or(0)
    }

    // ── Internal helpers ─────────────────────────────────────────────────
    fn push_tx(env: &Env, addr: &Address, record: TxRecord) {
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::TxCount(addr.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Tx(addr.clone(), count), &record);
        env.storage()
            .persistent()
            .set(&DataKey::TxCount(addr.clone()), &(count + 1));
    }
}

// ─────────────────────────────────────────────
//  Tests
// ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token::{Client as TokenClient, StellarAssetClient},
        Env, String,
    };

    fn setup() -> (Env, Address, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin  = Address::generate(&env);
        let sender = Address::generate(&env);
        let recvr  = Address::generate(&env);

        // Deploy a mock SAC token
        let token_id = env.register_stellar_asset_contract_v2(admin.clone());
        let token_addr = token_id.address();

        // Mint tokens to sender
        let sac = StellarAssetClient::new(&env, &token_addr);
        sac.mint(&sender, &1_000_000_000);

        // Deploy remit contract
        let contract_id = env.register(RemitContract, ());
        let client = RemitContractClient::new(&env, &contract_id);
        client.initialize(&admin, &10u32); // 0.1% fee

        (env, contract_id, token_addr, sender, recvr)
    }

    #[test]
    fn test_initialize() {
        let (env, contract_id, _, _, _) = setup();
        let client = RemitContractClient::new(&env, &contract_id);
        assert_eq!(client.get_fee(), 10);
    }

    #[test]
    fn test_register_and_get_name() {
        let (env, contract_id, _, sender, _) = setup();
        let client = RemitContractClient::new(&env, &contract_id);
        client.register(&sender, &String::from_str(&env, "Alice"));
        assert_eq!(
            client.get_name(&sender),
            String::from_str(&env, "Alice")
        );
    }

    #[test]
    fn test_send_payment() {
        let (env, contract_id, token_addr, sender, recvr) = setup();
        let client = RemitContractClient::new(&env, &contract_id);

        env.ledger().with_mut(|l| l.timestamp = 1_000_000);

        client.send(
            &sender,
            &recvr,
            &token_addr,
            &100_000_000_i128,
            &String::from_str(&env, "test payment"),
        );

        let token = TokenClient::new(&env, &token_addr);

        // Recipient gets 99.9% (fee = 0.1% of 100_000_000 = 100_000)
        assert_eq!(token.balance(&recvr), 99_900_000);

        // Tx recorded for both parties
        assert_eq!(client.tx_count(&sender), 1);
        assert_eq!(client.tx_count(&recvr), 1);
    }

    #[test]
    fn test_get_txs() {
        let (env, contract_id, token_addr, sender, recvr) = setup();
        let client = RemitContractClient::new(&env, &contract_id);

        client.send(
            &sender,
            &recvr,
            &token_addr,
            &50_000_000_i128,
            &String::from_str(&env, "first"),
        );
        client.send(
            &sender,
            &recvr,
            &token_addr,
            &25_000_000_i128,
            &String::from_str(&env, "second"),
        );

        let txs = client.get_txs(&sender, &2u32);
        assert_eq!(txs.len(), 2);
    }

    #[test]
    fn test_set_fee() {
        let (env, contract_id, _, _, _) = setup();
        let client = RemitContractClient::new(&env, &contract_id);
        client.set_fee(&50u32); // 0.5%
        assert_eq!(client.get_fee(), 50);
    }
}
