import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We create a group", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Is initialized!", async () => {
    const group = anchor.web3.Keypair.generate();
    const treasury = anchor.web3.Keypair.generate();
    const initializer = (program.provider as anchor.AnchorProvider).wallet;
    
    const tx = await program.methods.createGroup("Disney",1000,[])
    .accounts({
      group: group.publicKey,
      treasury: treasury.publicKey,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([group])
    .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });
});
