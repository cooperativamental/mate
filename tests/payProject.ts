import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram } from "@solana/web3.js"
import * as web3 from "@solana/web3.js"

const anchorProvider = anchor.AnchorProvider.env();
anchor.setProvider(anchorProvider);
const program = anchor.workspace.Mate as Program<Mate>;

const payer = (program.provider as anchor.AnchorProvider).wallet;
const name = "Will Pay"
const group = "group"
const project_type = "project_type"
const ratio = 10

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
const payments = [
  {member: member0.publicKey,
    amount:new anchor.BN(1),
  }
]
const currency = "SOL"
const amount = new anchor.BN(20)
const client = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')
const treasury = anchor.web3.Keypair.generate();

const [groupPublicKey] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("project"), Buffer.from(name), Buffer.from(group)],
  program.programId,
)

describe("We Create a Project and then pay for it", () => {
  it("Creating 'will pay' project...", async () => {
    await program.methods
      .createProject(
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
  });

  it("modifing project...", async () => {
    const tx = await program.methods.payProject()
      .accounts({
        payer: payer.publicKey,
        project: groupPublicKey,
        member0: member0.publicKey,
        member1: member1.publicKey,
        member2: member2.publicKey,
        member3: member3.publicKey,
        member4: member4.publicKey,
        member5: member5.publicKey,
        member6: member6.publicKey,
        member7: member7.publicKey,
        member8: member8.publicKey,
        member9: member9.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });
});