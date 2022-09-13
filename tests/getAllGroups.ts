import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";

describe("We get Groups", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Get all!", async () => {
    const provider = anchor.getProvider();
    anchor.setProvider(provider);
    
    const groups = await program.account.group.all()
    console.log(groups)
  });
});
