import { airdropSolTo } from "./initializeKeypair"
import * as web3 from "@solana/web3.js"

async function main() {
  const connection = new web3.Connection(web3.clusterApiUrl("devnet"))
process.argv.map(async (pubkey,i)=>i>1&&await airdropSolTo(connection, new web3.PublicKey(pubkey)))

}

main()
  .then(() => {
    console.log("Finished successfully")
    process.exit(0)
  })
  .catch((error) => {
    console.log(error)
    process.exit(1)
  })
