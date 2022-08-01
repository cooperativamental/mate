import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";

describe("mate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
