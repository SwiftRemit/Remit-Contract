/**
 * Setup script: generates a new keypair and funds it on testnet
 * Run: node scripts/setup-testnet.js
 */
import { generateKeypair, fundTestnetAccount } from "../src/stellar.js";

console.log("🚀 SwiftRemit Testnet Setup\n");

const keypair = generateKeypair();
console.log("✅ New Stellar Keypair Generated:");
console.log(`   Public Key:  ${keypair.publicKey}`);
console.log(`   Secret Key:  ${keypair.secretKey}`);
console.log("\n⚠️  Save your secret key securely. Add it to your .env file.\n");

console.log("💧 Funding account via Friendbot...");
try {
  await fundTestnetAccount(keypair.publicKey);
  console.log("✅ Account funded with 10,000 XLM on testnet!");
  console.log(`\n🔗 View on Stellar Expert:`);
  console.log(`   https://stellar.expert/explorer/testnet/account/${keypair.publicKey}`);
} catch (err) {
  console.error("❌ Funding failed:", err.message);
}

console.log("\n📝 Add these to your .env file:");
console.log(`SENDER_PUBLIC_KEY=${keypair.publicKey}`);
console.log(`SENDER_SECRET_KEY=${keypair.secretKey}`);
