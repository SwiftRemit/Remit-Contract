/**
 * Test payment script
 * Run: node scripts/send-payment.js
 */
import "dotenv/config";
import { sendXLM } from "../src/stellar.js";

const senderSecret = process.env.SENDER_SECRET_KEY;
const recipientPublicKey = process.env.RECIPIENT_PUBLIC_KEY;

if (!senderSecret || !recipientPublicKey) {
  console.error("❌ SENDER_SECRET_KEY and RECIPIENT_PUBLIC_KEY must be set in .env");
  process.exit(1);
}

const amount = "10"; // XLM
const memo = "SwiftRemit test";

console.log(`💸 Sending ${amount} XLM to ${recipientPublicKey}...\n`);

try {
  const result = await sendXLM({ senderSecret, recipientPublicKey, amount, memo });
  console.log("✅ Payment successful!");
  console.log(`   Transaction Hash: ${result.hash}`);
  console.log(`   Ledger: ${result.ledger}`);
  console.log(`\n🔗 View on Stellar Expert:`);
  console.log(`   https://stellar.expert/explorer/testnet/tx/${result.hash}`);
} catch (err) {
  console.error("❌ Payment failed:", err.message);
}
