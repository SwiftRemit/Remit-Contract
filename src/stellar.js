import * as StellarSdk from "@stellar/stellar-sdk";
import "dotenv/config";

const isTestnet = process.env.STELLAR_NETWORK !== "mainnet";

export const server = new StellarSdk.Horizon.Server(
  process.env.STELLAR_HORIZON_URL || "https://horizon-testnet.stellar.org"
);

export const networkPassphrase = isTestnet
  ? StellarSdk.Networks.TESTNET
  : StellarSdk.Networks.PUBLIC;

export { StellarSdk, isTestnet };

/**
 * Generate a new Stellar keypair
 */
export const generateKeypair = () => {
  const keypair = StellarSdk.Keypair.random();
  return {
    publicKey: keypair.publicKey(),
    secretKey: keypair.secret(),
  };
};

/**
 * Fund a testnet account via Friendbot
 */
export const fundTestnetAccount = async (publicKey) => {
  const response = await fetch(
    `https://friendbot.stellar.org?addr=${encodeURIComponent(publicKey)}`
  );
  if (!response.ok) {
    throw new Error(`Friendbot failed: ${response.statusText}`);
  }
  return response.json();
};

/**
 * Get account balances
 */
export const getBalances = async (publicKey) => {
  const account = await server.loadAccount(publicKey);
  return account.balances.map((b) => ({
    asset: b.asset_type === "native" ? "XLM" : b.asset_code,
    balance: b.balance,
  }));
};

/**
 * Send a native XLM payment
 */
export const sendXLM = async ({ senderSecret, recipientPublicKey, amount, memo = "" }) => {
  const senderKeypair = StellarSdk.Keypair.fromSecret(senderSecret);
  const senderAccount = await server.loadAccount(senderKeypair.publicKey());

  const txBuilder = new StellarSdk.TransactionBuilder(senderAccount, {
    fee: await server.fetchBaseFee(),
    networkPassphrase,
  })
    .addOperation(
      StellarSdk.Operation.payment({
        destination: recipientPublicKey,
        asset: StellarSdk.Asset.native(),
        amount: String(amount),
      })
    )
    .setTimeout(30);

  if (memo) txBuilder.addMemo(StellarSdk.Memo.text(memo));

  const transaction = txBuilder.build();
  transaction.sign(senderKeypair);

  const result = await server.submitTransaction(transaction);
  return { hash: result.hash, ledger: result.ledger };
};

/**
 * Send a USDC payment (requires trustline)
 */
export const sendUSDC = async ({
  senderSecret,
  recipientPublicKey,
  amount,
  usdcIssuer,
  memo = "",
}) => {
  const senderKeypair = StellarSdk.Keypair.fromSecret(senderSecret);
  const senderAccount = await server.loadAccount(senderKeypair.publicKey());

  const usdc = new StellarSdk.Asset("USDC", usdcIssuer);

  const txBuilder = new StellarSdk.TransactionBuilder(senderAccount, {
    fee: await server.fetchBaseFee(),
    networkPassphrase,
  })
    .addOperation(
      StellarSdk.Operation.payment({
        destination: recipientPublicKey,
        asset: usdc,
        amount: String(amount),
      })
    )
    .setTimeout(30);

  if (memo) txBuilder.addMemo(StellarSdk.Memo.text(memo));

  const transaction = txBuilder.build();
  transaction.sign(senderKeypair);

  const result = await server.submitTransaction(transaction);
  return { hash: result.hash, ledger: result.ledger };
};

/**
 * Add a trustline for an asset (required before receiving non-XLM assets)
 */
export const addTrustline = async ({ accountSecret, assetCode, assetIssuer }) => {
  const keypair = StellarSdk.Keypair.fromSecret(accountSecret);
  const account = await server.loadAccount(keypair.publicKey());

  const asset = new StellarSdk.Asset(assetCode, assetIssuer);

  const transaction = new StellarSdk.TransactionBuilder(account, {
    fee: await server.fetchBaseFee(),
    networkPassphrase,
  })
    .addOperation(StellarSdk.Operation.changeTrust({ asset }))
    .setTimeout(30)
    .build();

  transaction.sign(keypair);
  const result = await server.submitTransaction(transaction);
  return { hash: result.hash };
};
