/**
 * Fund an existing testnet account via Friendbot
 * Run: node scripts/fund-account.js
 */
import "dotenv/config";
import { fundTestnetAccount } from "../src/stellar.js";

const publicKey = process.env.SENDER_PUBLIC_KEY;

if (!publicKey) {
  console.error("❌ SENDER_PUBLIC_KEY not set in .env");
  process.exit(1);
}

console.log(`💧 Funding ${publicKey} via Friendbot...\n`);

try {
  await fundTestnetAccount(publicKey);
  console.log("✅ Account funded with 10,000 XLM!");
} catch (err) {
  console.error("❌ Funding failed:", err.message);
}
