import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { assert } from "chai"
import * as web3 from "@solana/web3.js"
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We create a project", () => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Can create a project", async () => {
    const name = "project name"
    const group = "group name"
    const project_type = "project_type"
    const ratio = 10
    const payments = []
    const currency = "SOL"
    const amount = new anchor.BN(20)
    const client = new PublicKey('5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj')
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
      [Buffer.from("project"), Buffer.from(name)],
      program.programId,
    )
  
    await program.methods
      .createProjectPda(
        name,
        group,
        project_type,
        ratio,
        payments,
        currency,
        amount,
        new anchor.BN(Date.now()),
        new anchor.BN(Date.now()),
        client
        )
      .accounts({
        project: groupPublicKey,
        treasury: treasury.publicKey,
        payer: anchorProvider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc()
  
    const storedGroup = await program.account.project.fetch(groupPublicKey)
    console.log(storedGroup)
    assert.equal(storedGroup.name, name)

  });
});
