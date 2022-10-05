import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"
import * as web3 from "@solana/web3.js"

describe("We create a group", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Creando Grupo!", async () => {
    const group = anchor.web3.Keypair.generate();
    const treasury = anchor.web3.Keypair.generate();
    const initializer = (program.provider as anchor.AnchorProvider).wallet;
    const member0 = anchor.web3.Keypair.generate();
    const member1 = anchor.web3.Keypair.generate();
    const member2 = anchor.web3.Keypair.generate();
    const member3 = anchor.web3.Keypair.generate();
    const member4 = anchor.web3.Keypair.generate();
    const member5 = anchor.web3.Keypair.generate();
    const member6 = anchor.web3.Keypair.generate();
    const member7 = anchor.web3.Keypair.generate();
    const member8 = anchor.web3.Keypair.generate();
    const member9 = anchor.web3.Keypair.generate();
    const tx = await program.methods.createGroup(
      "Testing",
      1000,
      [
        member0.publicKey,
        member1.publicKey,
        member2.publicKey,
        member3.publicKey,
        member4.publicKey,
        member5.publicKey,
        member6.publicKey,
        member7.publicKey,
        member8.publicKey,
        member9.publicKey,
      ]
    )
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
