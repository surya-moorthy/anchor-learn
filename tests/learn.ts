import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Learn } from "../target/types/learn";
import { assert } from "chai";

describe("learn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Learn as Program<Learn>; 
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  it("Say hello", async () => {
    await program.methods.helloWorld().accounts({}).rpc();
  })

  it("create new account", async () => {
    const newKeypair = anchor.web3.Keypair.generate();

    await program.methods.createSystemAccount().accounts({
        signer : wallet.publicKey,
        newAccount: newKeypair.publicKey     
    }).signers([newKeypair]).rpc();

    const lamports = await connection.getMinimumBalanceForRentExemption(0);

    const accountInfo = await connection.getAccountInfo(newKeypair.publicKey);
    assert(accountInfo.lamports === lamports);
  })

  
  
});
