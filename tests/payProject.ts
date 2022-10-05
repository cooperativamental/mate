import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We Create a Project and then pay for it", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;
  const project = anchor.web3.Keypair.generate();
  const payer = (program.provider as anchor.AnchorProvider).wallet;
  const member0 = (program.provider as anchor.AnchorProvider).wallet;
  const member1 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member2 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member3 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member4 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member5 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member6 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member7 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member8 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const member9 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const treasury = anchor.web3.Keypair.generate();
  const payments = [] as unknown as Mate["types"][0]
  const ricardo = new PublicKey('5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj')

  it("Creating 'will pay' project...", async () => {

    console.log(payer.publicKey)
    const tx = await program.methods.createProject(
      'will pay',
      "eeee",
      "v0",
      1000,
      [
      {member: member0.publicKey,
        amount:new anchor.BN(1),
      },
      {member: member1,
        amount:new anchor.BN(10),
      },
      {member: member2,
        amount:new anchor.BN(100),
      },
      {member: member3,
        amount:new anchor.BN(20),
      },
      {member: member4,
        amount:new anchor.BN(30),
      },
      {member: member5,
        amount:new anchor.BN(40),
      },
      {member: member6,
        amount:new anchor.BN(50),
      },
      {member: member7,
        amount:new anchor.BN(60),
      },
      {member: member8,
        amount:new anchor.BN(70),
      },
      {member: member9,
        amount:new anchor.BN(81),
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
      member1: member1,
      member2: member2,
      member3: member3,
      member4: member4,
      member5: member5,
      member6: member6,
      member7: member7,
      member8: member8,
      member9: member9,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });
});