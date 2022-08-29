import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mint } from "../target/types/mint";

describe("mint", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mint as Program<Mint>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
