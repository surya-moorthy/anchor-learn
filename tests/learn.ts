import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Learn } from "../target/types/learn";

describe("learn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Learn as Program<Learn>;

  it("Say hello", async () => {
    await program.methods.helloWorld().accounts({}).rpc();
  })
});
