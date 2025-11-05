import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kailua } from "../target/types/kailua";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("kailua submit_query test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.kailua as Program<Kailua>;
  const payer = provider.wallet.payer; // signer for paying account creation

  it("Submits a query!", async () => {
    // Generate a new Keypair for the query account
    const queryAccount = Keypair.generate();

    // Prepare test data
    const queryId = "test-query-001";
    const agentId = "agent-abc";
    const queryPayload = "get_price_solana";
    const payment = new anchor.BN(500); // Example payment in smallest units (e.g., 0.0005 USDC)

    // Call the submit_query function
    const tx = await program.methods
      .newQuery(queryId, agentId, queryPayload, payment)
      .accounts({
        signer: payer.publicKey,
        query: queryAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([queryAccount]) // Needed because we're creating a new account
      .rpc();

    console.log("Transaction signature:", tx);

    // Fetch the on-chain account to assert values
    const queryData = await program.account.query.fetch(queryAccount.publicKey);

    console.log("Stored query account:", queryData);

    // Assertions
    if (queryData.queryId !== queryId) throw new Error("queryId mismatch");
    if (queryData.agentId !== agentId) throw new Error("agentId mismatch");
    if (queryData.queryPayload !== queryPayload)
      throw new Error("queryPayload mismatch");
    const paymentStatusVariant = Object.keys(queryData.paymentStatus)[0];
    if (paymentStatusVariant !== "pending") {
      throw new Error("paymentStatus should be Pending");
    }
  });
});
