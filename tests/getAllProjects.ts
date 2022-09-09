import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";

describe("We get alll Projects", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("get all projects", async () => {
    const provider = anchor.getProvider();
    anchor.setProvider(provider);
    
    const project = await program.account.project.all()
    console.log(project)
  });
});
