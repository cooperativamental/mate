import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mate } from "../target/types/mate";
import { assert } from "chai"
import * as web3 from "@solana/web3.js"
import { PublicKey } from "@solana/web3.js"

describe("We create a project", () => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  const program = anchor.workspace.Mate as Program<Mate>;

  it("Can create a project", async () => {
    const name = "-Project Name-"
    const group = "Group Name-"
    const project_type = "project_type"
    const ratio = 10
    const currency = "SOL"
    const amount = new anchor.BN(20)
    const client = new PublicKey('5xot9PVkphiX2adznghwrAuxGs2zeWisNSxMW6hU6Hkj')
    const member0 = (program.provider as anchor.AnchorProvider).wallet;
    const member1 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
    const member2 = new PublicKey("CUtKCTar8gb5VYCDWbX5yFMVrhbnod9aCNf4cfhD2qPK")
    const member3 = new PublicKey("6ivuS4xbpr61vMgMwPPnvhSw2sWs4fpKFt7baoiK3s3S")
    const member4 = new PublicKey("8i5WmmdGgibZ36h3xLEDsQ6N8QY71RpZqUoVieYZx1GL")
    const member5 = new PublicKey("GpMhH3hTKdGnJqAaXM9tZoq1GUSFMFNxN8bE5vZzrK6x")
    const member6 = new PublicKey("3dwJ6Xr534orZTwa8n2SJZQNeHY2FWLJ6SpxitBz3wva")
    const member7 = new PublicKey("5G2xXJ46kPo3R4SNmFRCg2z2ru68LCoeoWvZK3FfUGMF")
    const member8 = new PublicKey("6R3MnM7LrMpLSs5NRnniEQ6kDxTHfpY8Qg9hXMKVkaf4")
    const member9 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
    const payments = [
      {
        member: member0.publicKey,
        amount: new anchor.BN(1),
      },
      {
        member: member1,
        amount: new anchor.BN(10),
      },
      {
        member: member2,
        amount: new anchor.BN(100),
      },
      {
        member: member3,
        amount: new anchor.BN(20),
      },
      {
        member: member4,
        amount: new anchor.BN(30),
      },
      {
        member: member5,
        amount: new anchor.BN(40),
      },
      {
        member: member6,
        amount: new anchor.BN(50),
      },
      {
        member: member7,
        amount: new anchor.BN(60),
      },
      {
        member: member8,
        amount: new anchor.BN(70),
      },
      {
        member: member9,
        amount: new anchor.BN(81),
      },
    ] as unknown as Mate["types"][0]

    const treasury = anchor.web3.Keypair.generate();
    const [pdaPublicKey] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(name), Buffer.from(group)],
      program.programId,
    )

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
        project: pdaPublicKey,
        payer: anchorProvider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc()

    const storedProject = await program.account.project.fetch(pdaPublicKey)
    console.log(storedProject)
    assert.equal(storedProject.name, name)
  });
});
