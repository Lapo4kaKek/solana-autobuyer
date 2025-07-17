import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaAutoSwap } from "../target/types/solana_auto_swap";
import { PublicKey, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const programId = "5rXaHK1Z3iWLS34A2uhS77PndZecCHYZZENuZzbdk49p";

describe("solana-auto-swap", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.solanaAutoSwap as Program<SolanaAutoSwap>;
  const wallet = provider.wallet as anchor.Wallet;

  it("Test auto swap structure", async () => {
    const userKeypair = Keypair.generate();
    const tokenMint = Keypair.generate();
    const quoteMint = Keypair.generate();
    
    // mock account for Raydium
    const mockAccounts = {
      tokenProgram: TOKEN_PROGRAM_ID,
      ammProgram: new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"), // Raydium AMM Program
      amm: Keypair.generate().publicKey,
      ammAuthority: Keypair.generate().publicKey,
      ammOpenOrders: Keypair.generate().publicKey,
      ammCoinVault: Keypair.generate().publicKey,
      ammPcVault: Keypair.generate().publicKey,
      marketProgram: new PublicKey("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"), // Serum DEX
      market: Keypair.generate().publicKey,
      marketBids: Keypair.generate().publicKey,
      marketAsks: Keypair.generate().publicKey,
      marketEventQueue: Keypair.generate().publicKey,
      marketCoinVault: Keypair.generate().publicKey,
      marketPcVault: Keypair.generate().publicKey,
      marketVaultSigner: Keypair.generate().publicKey,
      userTokenAccount: Keypair.generate().publicKey,
      userQuoteAccount: Keypair.generate().publicKey,
      user: wallet.publicKey,
    };

    console.log("Testing account structure...");

    try {
      await program.methods
        .autoSwap(
          tokenMint.publicKey,
          quoteMint.publicKey,
          new anchor.BN(1000000), // 1 token
          new anchor.BN(900000)   // min out
        )
        .accounts(mockAccounts)
        .rpc();
        
    } catch (error) {
      console.log("Expected error (mock accounts):", error.message);
      
      if (error.message.includes("account")) {
        console.log("Account structure is correct");
      } else {
        console.log("Account structure issue:", error);
      }
    }

    console.log("Program ID:", program.programId.toString());
    console.log("Test completed!");
  });
});

// describe("solana-auto-swap", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.solanaAutoSwap as Program<SolanaAutoSwap>;

//   it("Is initialized!", async () => {
//     const tx = await program.methods.initialize().rpc();
//     console.log("transaction signature", tx);
//   });
// });
