/**
 * Check balance of a Stellar account
 * Run: node scripts/check-balance.js
 */
import "dotenv/config";
import { getBalances } from "../src/stellar.js";

const publicKey = process.env.SENDER_PUBLIC_KEY;

if (!publicKey) {
  console.error("❌ SENDER_PUBLIC_KEY not set in .env");
  process.exit(1);
}

console.log(`🔍 Checking balance for: ${publicKey}\n`);

try {
  const balances = await getBalances(publicKey);
  console.log("💰 Balances:");
  balances.forEach((b) => {
    console.log(`   ${b.asset}: ${b.balance}`);
  });
} catch (err) {
  console.error("❌ Error:", err.message);
}
