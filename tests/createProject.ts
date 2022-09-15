import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { PublicKey, SystemProgram} from "@solana/web3.js"

describe("We Create a Project", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Mate as Program<Mate>;

  it("Creating project...", async () => {
    const project = anchor.web3.Keypair.generate();
    const treasury = anchor.web3.Keypair.generate();
    const initializer = (program.provider as anchor.AnchorProvider).wallet;
    const payments = [] as unknown as Mate["types"][0]
    const ricardo = new PublicKey('5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj')

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
    const tx = await program.methods.createProject(
      "Clean apartment",
      "CLeaners",
      "v0",
      1000,
      payments,
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
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([project])
    .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });
});
