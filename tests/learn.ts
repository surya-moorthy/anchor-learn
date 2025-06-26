import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Learn } from "../target/types/learn";
import { assert } from "chai";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { PublicKey } from "@solana/web3.js";

describe("learn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Learn as Program<Learn>; 
  const connection = provider.connection;
  const payer = provider.wallet as anchor.Wallet;

  const [] = PublicKey.findProgramAddressSync([Buffer.from("USER"),payer.publicKey.toBuffer()],program.programId);

  it("Create User Account", async () => {
    await program.methods.createUser("surya").accounts({
      user: payer.publicKey,
    }).rpc(); 
  })
  
});
