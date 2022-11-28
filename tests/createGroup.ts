import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { assert } from "chai"
import * as web3 from "@solana/web3.js"

describe("We create a group", () => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Can create a group", async () => {
    const name = "-Group Name-"
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
    const treasury = anchor.web3.Keypair.generate();
  
    const [groupPublicKey] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("group"), Buffer.from(name)],
      program.programId,
    )
  
    await program.methods
      .createGroup(
        name,
        10,
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
        group: groupPublicKey,
        payer: anchorProvider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc()
  
    const storedGroup = await program.account.group.fetch(groupPublicKey)
    console.log(storedGroup)
    assert.equal(storedGroup.name, name)

  });
});
