import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We Create a Project", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Is initialized!", async () => {
    const project = anchor.web3.Keypair.generate();
    const treasury = anchor.web3.Keypair.generate();
    const initializer = (program.provider as anchor.AnchorProvider).wallet;
    
    const tx = await program.methods.createProject("Clean",123,[])
    .accounts({
      project: project.publicKey,
      treasury: treasury.publicKey,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([project])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
