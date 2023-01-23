# Mate Protocol

This is the Repo for the Solana Program (aka Smart Contract)
For the Front End project go to: [https://github.com/cooperativamental/MATE-app](https://github.com/cooperativamental/MATE-app)

### Steps to run this project

1. Follow [this guide](https://book.anchor-lang.com/getting_started/installation.html) to install all the needed software
2. Clone this repo:
    `git clone https://github.com/cooperativamental/mate.git`
3. Run a local validator
    `solana-test-validator`
4. Open another terminal, cd into the project's folder and install dependencies
    `cd mate && yarn`
5. Build the program
    `anchor build`
6. Set correct program id
    `anchor keys list`
    replace all ocurrences of Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS with the resulting key and save the files
7. Set localhost as target
    `solana config set --url http://localhost:8899`
8. Create a dev CLI wallet
    `solana-keygen new -o dev_account.json`
9. Airdrop some fake SOL to that wallet
    `solana airdrop 10 dev_account.json`
10. Deploy and run tests
    `anchor test --skip-local-validator`
    If all test pass you are all set up

### PDAs and SEEDs

| PDA | Relevant Keys | Seeds |
| --- | ------------- | ----- |
| group | name: String | "group", name |
| project | group: String, name: String, milestones: u8 | "project", name, group |
| milestone(\*) | - | "milestone", (project)name, (group)name, order: u8 |

*(\*) in This first version the milestone PDA isn´t implented, there is only one milestone per project, in the next iteration the milestone feature will be implemented.*

### Aditional documentation


Running `cargo doc` in a terminal will outputs a folder named `target/doc` wich contains very detailed documentation of the project and his depencies,
We recommend `/target/doc/mate/index.html` as an entrypoint.
