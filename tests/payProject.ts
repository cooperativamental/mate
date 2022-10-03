import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We Create a Project and then pay for it", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;
  const project = anchor.web3.Keypair.generate();
  const payer = (program.provider as anchor.AnchorProvider).wallet;
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
  const payments = [] as unknown as Mate["types"][0]
  const ricardo = new PublicKey('5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj')

  it("Creating 'will pay' project...", async () => {

    console.log(payer.publicKey)
    const tx = await program.methods.createProject(
      'will pay',
      "CLeaners",
      "v0",
      1000,
      [
      {member: member0.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member1.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member2.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member3.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member4.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member5.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member6.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member7.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member8.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member9.publicKey,
        amount:new anchor.BN(1),
      },
      ] as unknown as Mate["types"][0],
      "",
      "SOL",
      new anchor.BN(123),
      new anchor.BN(Date.now()),
      new anchor.BN(Date.now()),
      ricardo
      )
    .accounts({
      project: project.publicKey,
      treasury: treasury.publicKey,
      initializer: payer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([project])
    .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });


  it("modifing project...", async () => {


    /* Parameters:
        name: String,
        group: String,
        project_type: String,
        ratio: u16,
        payments: Vec<Payment>,
        next: String,
        currency: String,
        amount: u64,
        start_date: u8,
        end_date: u8,
        client: Pubkey,
    */

    // La funcion new anchor.BN() la usamos cuando usamos un numero que puede ser muy grande (u64, i64)
    const tx = await program.methods.payProject()
    .accounts({
      payer: payer.publicKey,
      project: project.publicKey,
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
    .signers([payer])
    .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });
});


const publicKeyFromBn = (feePayer) => {
  const bigNumber = new anchor.BN(feePayer._bn, 16)
  const decoded = { _bn: bigNumber };
  return new PublicKey(decoded);
};