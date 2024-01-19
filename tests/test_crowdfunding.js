const anchor = require("@coral-xyz/anchor");
const { Connection, clusterApiUrl, Keypair, LAMPORTS_PER_SOL, StakeProgram, Authorized, Lockup, sendAndConfirmTransaction, PublicKey } = require("@solana/web3.js");

const {Program, AnchorProvider, web3, utils, BN} =require('@project-serum/anchor');
const { getInfoIdoAccount } = require("./getInfoIdo");

let idoAccountString = ""

const main = async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const idoAccount = Keypair.generate();
  if (!idoAccount) return;
  const raiseToken = new PublicKey("3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b");
  const rate = new BN(1000);
  const openTimestamp = new BN(1705534720);
  const allocationDuration = new BN(1705544720);
  const fcfsDuration = new BN(1705545720);
  const cap = new BN(1 * LAMPORTS_PER_SOL);
  const releaseToken = new PublicKey("3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b");

  const program = anchor.workspace.Crowdfunding;

  const tx = await program.rpc.createIdoAccount(raiseToken, rate, openTimestamp, allocationDuration, fcfsDuration, cap, releaseToken, {
    accounts: {
      idoInfo: idoAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [idoAccount],
  });

  idoAccountString = idoAccount.publicKey.toString()
  

 const token = new PublicKey("GdgCpzyFdcZqvtwyX1phzNH8Q32vcNk47AqrZTSsciLs");
 const pair = new PublicKey("5yAX4HZEq9X2DumUkotrmPLPuFGVuMkWphUF2EcmtyBS");

 //test setupReleaseToken  -> OK
 await program.rpc.setupReleaseToken(token, pair, {
  accounts: {
    idoInfo: idoAccount.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  }
});

console.log("Your transaction signature", tx);
console.log("Your idoAccount", idoAccount.publicKey.toString());

const idoInfo = await getInfoIdoAccount(program, idoAccount.publicKey.toString());
console.log(JSON.stringify(idoInfo));

}


const runMain = async () => {
  try {
    await main();
  } catch (error) {
    console.log(error);
    process.exit(1);
  }
}

runMain()