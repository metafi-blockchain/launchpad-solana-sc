import * as anchor from "@coral-xyz/anchor";
import { Crowdfunding } from "../target/types/crowdfunding";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SOLANA_SCHEMA, Signer, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, web3, utils, BN } from '@project-serum/anchor';
import { assert, expect } from "chai";
import moment from 'moment'
import {
  getAssociatedTokenAddress,
  createMint,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token"
import { getAllocationRemaining } from "./ido.ultil";
import { IdoAccount } from "./ido_type";
import { describe } from "mocha";
// let IDO_TEST = "ARRwPx2wrkn1MHvicoSam1tRFFkXxRKHQcqTgBBYsaut";

const AUTHORITY_IDO = "IDO_ONE_PAD";
const AUTHORITY_USER = "USER_ONE_PAD";
const ADMIN_ROLE = "ADMIN_ROLE";
const OPERATOR_ROLE = "OPERATOR_ROLE";
const ONEPAD = "ONE_PAD";
const USDC_TEST = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW")

const getInfoIdoAccount = async (program: any, idoAccountAddress: String) => {
  const idoAccountPub = new PublicKey(idoAccountAddress)
  let ido_info = await program.account.idoAccount.fetch(idoAccountPub);
  return ido_info
}

const getOnepadPda = (programId: PublicKey) => {
  const [mint, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(ONEPAD)],
    programId
  );
  console.log("OnePad PDA:", mint.toString());
  return mint;

}


const getPdaIdo = (programId: PublicKey, ido_id: number) => {
  let idoIdBuff = new anchor.BN(ido_id);
  const [idoPDAs, _] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(AUTHORITY_IDO),
      idoIdBuff.toBuffer("le", 8),
    ],
    programId);
  return idoPDAs;
}

const getAdminRolePda = (programId: PublicKey, user: PublicKey) => {
  const [mint, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(ADMIN_ROLE), user.toBuffer()],
    programId
  );
  console.log("Admin PDA:", mint.toString());
  return mint;
};
const  getOperatorRolePda = (programId: PublicKey, user: PublicKey) => {
  const [mint, _] = PublicKey.findProgramAddressSync(
    [Buffer.from(OPERATOR_ROLE), user.toBuffer()],
    programId
  );
  console.log("operator PDA:", mint.toString());
  return mint;
};


const getPdaUser = (programId: PublicKey, idoPDA: PublicKey, user: PublicKey) => {

  const [idoPDAs, _] = PublicKey.findProgramAddressSync(
    [
      utils.bytes.utf8.encode(AUTHORITY_USER),
      idoPDA.toBuffer(),
      user.toBuffer(),

    ],
    programId);
  return idoPDAs;
}

const getTokenInfoByTokenAccount = async (connection: Connection, token_mint: String, toWalletPub: String) => {

  try {
    const mint = new PublicKey(token_mint);
    const tokenAcc = new PublicKey(toWalletPub);
    const tokenAccount = getAssociatedTokenAddressSync(mint, tokenAcc);

    const tokenAccountInfo = await connection.getParsedAccountInfo(tokenAccount);

    console.log(tokenAccountInfo.value?.data);
    return tokenAccountInfo
  }
  catch (error) {
    console.log(error);
  }
}

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

const raise_token_test = "8xRoWyiPKGzqWPwh81HAGaytxzy6bEgN58Uh7LiHvMru";
const ido_id = 1;






describe("crowd funding testing", () => {

  let idoPDAs = getPdaIdo(program.programId, ido_id);
  const operator_wallet = new PublicKey("9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS");


  // it("initialize onepad program", async () => {
  //   const onepadPda = getOnepadPda(program.programId);
  //   const adminRolePda = getAdminRolePda(program.programId, provider.publicKey);

  //   const tx = await program.methods.initializeOnepad(operator_wallet).accounts({
  //     onepadPda: onepadPda,
  //     adminRolePda: adminRolePda,
  //     authority: provider.publicKey,
  //     systemProgram: SystemProgram.programId,
  //   }).rpc();
  //   console.log("transactions: ", tx);
  // })

  it("add operator onepad program", async () => {
    const operator = new PublicKey("BF8uL2SazBGZBckXtgN5kRgcjChhztGgJdAJW2kC8fgQ")
    const onepadPda = getOnepadPda(program.programId);
    const adminPda = getAdminRolePda(program.programId, provider.publicKey);
    const operatorPda = getOperatorRolePda(program.programId,operator );

    const tx = await program.methods.adminAddOperator(operator).accounts({
      onepadPda: onepadPda,
      adminPda: adminPda,
      operatorPda: operatorPda,
      authority: provider.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("transactions: ", tx);
  })





  let raise_token = USDC_TEST
  it("initialize Ido program", async () => {

    const rate =  1000000;
    const openTimestamp = convertTimeTimeTo("2024/06/24 09:15:00");
    const allocationDuration =  2*60*60;
    const fcfsDuration =  120*60;

    const cap = new BN(100*10**6);


    let token_mint = USDC_TEST;

    const raiseTokenAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);

    console.log("associatedToken: ", raiseTokenAccount.toString());
    console.log(provider.wallet.publicKey.toString());
    console.log("idoPDAs: " ,idoPDAs.toString());
    console.log("token_mint: ",token_mint.toString());

    
    const initIdoParams = {
      raiseToken: USDC_TEST,
      rate: rate,
      openTimestamp: openTimestamp,
      allocationDuration: allocationDuration,
      fcfsDuration: fcfsDuration,
      cap: cap,
      idoId: new BN(ido_id),
    }
    const operator = new PublicKey("BF8uL2SazBGZBckXtgN5kRgcjChhztGgJdAJW2kC8fgQ")
    const operatorPda = getOperatorRolePda(program.programId,operator );

    try {
      await program.methods.initializeIdo(initIdoParams).accounts({
        onepadPda: getOnepadPda(program.programId),
        idoAccount: idoPDAs,
        operatorPda: operatorPda,
        authority: provider.publicKey,
        tokenMint: token_mint,
        tokenAccount: raiseTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    }).rpc() ;
    } catch (error) {
      console.log(error);

    }

    // console.log("IDO Account", idoPDAs.publicKey.toString());

    let idoInfo = await program.account.idoAccount.fetch(idoPDAs);
    console.log(JSON.stringify(idoInfo));
    assert.equal(idoInfo.authority.toString(), operatorPda.toString(), "Owner is user create ido account")
  });



  // it("update_admin_ido", async () => {
  //   const newAdmin = new PublicKey("3Mr13g5w5NFyh8GVfXMvZv2diZSMKDBASzQ2YwBNHayP");
  //   await program.methods.updateAdminIdo(newAdmin).accounts({
  //     idoAccount: idoPDAs,
  //     adminWallet: adminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   let adminPdaInfo = await program.account.adminAccount.fetch(adminPda);
  //   console.log(JSON.stringify(adminPdaInfo));



  // })

  // it("set_cap", async () => {
  //   try {

  //     const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

  
  //     const cap = new BN(100).mul(new BN(LAMPORTS_PER_SOL));
  //     await program.methods.setCap(cap).accounts({
  //       idoAccount: idoPDAs,
  //       operatorPda: operatorPda,
  //       authority: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }).rpc();

  //     const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //     const _cap = idoInfo._cap;
  //     assert.equal(idoInfo._cap, _cap, "cap  is setup");
  //   } catch (error) {
  //     console.log(error);

  //   }
  // })

  // it("set rate", async () => {
  //   const rate = 100000000;
  //   const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

  //   await program.methods.setRate(rate).accounts({
  //     idoAccount: idoPDAs,
  //     operatorPda: operatorPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   console.log(JSON.stringify(idoInfo));


  //   const _rate = idoInfo._rate;
  //   assert.equal(idoInfo._rate, _rate, "_rate  is setup");

  // })


  it("set open timestamp", async () => {


    const timestamp = convertTimeTimeTo("2024/06/24 14:30:00");
    console.log("timestamp", timestamp.toString() );
    const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );
    await program.methods.setOpenTimestamp(timestamp).accounts({
      idoAccount: idoPDAs,
      operatorPda: operatorPda,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();


    const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
    const _open_timestamp = idoInfo._open_timestamp;
    assert.equal(idoInfo._open_timestamp, _open_timestamp, "_open_timestamp  is setup");

  })


  // it("modify_rounds", async () => {

  //   const nameList = ["Allocation", "FSCS prepare", " FCFS",] ;
  //   const durationSeconds = [60*60, 1800, 3600];

  //   //check lai logic cho round class
  //  const classList = [{allocation:{}},  {fcfsPrepare:{}},  {fcfs:{}} ]

  //  const param = {
  //     nameList: nameList,
  //     durationList: durationSeconds,
  //     classList: classList
  //  }

  //  const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

  //   const tx  = await program.methods.modifyRounds(param)
  //   .accounts({
  //      idoAccount: idoPDAs,
  //      operatorPda: operatorPda,
  //      authority: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc();  
  //    console.log(tx);
  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //    console.log(JSON.stringify(idoInfo));


  //   const rounds = idoInfo.rounds;
  //   for (let i = 0; i < rounds.length; i++) {
  //       const r = rounds[i];
  //       assert.equal(r.name, nameList[i], "modify round name");
  //       assert.equal(r.durationSeconds, durationSeconds[i], "modify duration");
  //       // assert.equal(JSON.stringify(r.class), JSON.stringify(classList[i]), "modify class");
  //   }
  // });

  // it("modify_round", async () => {


  //   const index = 1;
  //   const name = "Test round1";
  //   const durationSeconds = 60;

  //   const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

  //  const param = {
  //     roundIndex: index ,
  //     name: name,
  //     durationSeconds: durationSeconds,
  //     class:  { allocation: {} }
  //  }
  //   await program.methods.modifyRound(param).accounts({
  //     idoAccount: idoPDAs,
  //     operatorPda: operatorPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //  const round = idoInfo.rounds[index];



  //   assert.equal(round.name, name, "modify round name");

  //   assert.equal(round.durationSeconds, durationSeconds, "modify duration");
  //   // assert.equal(JSON.stringify(round.class), JSON.stringify(_class), "modify class");
  // });






  it("modify_round_allocations", async () => {

    // let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");

    const round_index = 3;
    const tierAllocations = [new BN(10 * 10**6), new BN(20 * 10**6) , new BN(30 * 10**6)];
    // const tierAllocations = [new BN(0), new BN(0) , new BN(0)];
    const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

    const param ={
      roundIndex: round_index,
      tierAllocations: tierAllocations
    }
    try {
      await program.methods.modifyRoundAllocations(param).accounts( {
        idoAccount: idoPDAs,
        operatorPda: operatorPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();

     const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
     console.log(JSON.stringify(idoInfo));
     const  roundAllocations = idoInfo.rounds[round_index].tierAllocations;
     for (let i = 0; i < roundAllocations.length; i++) {
        const tierAl = roundAllocations[i];
        assert.equal(tierAl.toString(), tierAllocations[i].toString(), "tier allocation is amount setup");
     }
    } catch (error) {
      console.log(error);

    }

  });


  // it("modify_tier_allocated", async () => {
  //   const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   console.log(JSON.stringify(IdoInfo));


  //   const add1 = "2coCGfyERKREmvFnSezrgUZr8w1eWbZnA8LvZfPpjthy";
  //   let user1 = new PublicKey(add1)
  //   let userPDA = getPdaUser(program.programId, idoPDAs, user1);
  //   const tier = 1;

  //   console.log("userPDA: ", userPDA.toString());
  //   console.log("idoPDA: ", idoPDAs.toString());
  //   const operatorPda = getOperatorRolePda(program.programId,provider.publicKey );

  //   let remove = false
  //   try {
  //     const data = {
  //       tier: tier,
  //       address:  user1,
  //       remove: remove,
  //     }
  //     let tx = await program.methods.modifyTierAllocated(data).accounts({
  //       idoAccount: idoPDAs,
  //       authority: provider.wallet.publicKey,
  //       operatorPda: operatorPda,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       userIdoAccount: userPDA
  //     }).rpc();

  //     console.log("transactions: ", tx);

  //     let userInfo = await program.account.pdaUserStats.fetch(userPDA);

  //     // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
  //     console.log(JSON.stringify(userInfo));

  //     assert.equal(userInfo.tierIndex, tier, `${user1} is add in tier ${tier}`);

  //     assert.equal(userInfo.allocated, !remove, `address has allocated change: ${!remove}`);
  //     assert.equal(userInfo.address.toString(), user1.toString(), `${user1} is add white list`);
  //   } catch (error) {
  //     console.log(error);

  //   }
  // })
  

  // it("joinIdo", async () => {
  //   // const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";

  //   // let idoPDA =  getPdaIdo(program, ido_id);


  //   const token_mint = new PublicKey(raise_token_test);

  //   let userPDA =  getPdaUser(program.programId,  idoPDAs, provider.publicKey);

  //   const desAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);

  //   console.log("desAccount: " , desAccount.toString());

  //   const sourceAccount = getAssociatedTokenAddressSync(token_mint, provider.publicKey, true);

  //   // let userPDA =  getPdaUser(program.programId,  idoPDAs, ido_id, provider.publicKey);

  //     try {
  //       let idoInfo = await program.account.idoAccount.fetch(idoPDAs);
  //       let userInfo = await program.account.pdaUserStats.fetch(userPDA);

  //       // const infoWallet = getAllocationRemaining(1, userInfo.tierIndex, <IdoAccount><unknown>idoInfo ,  userInfo)

  //       // console.log(JSON.stringify(infoWallet));


  //       //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
  //       let amount = new BN(3 * LAMPORTS_PER_SOL);
  //       let tx = await program.methods
  //         .participate(amount).accounts({
  //           idoAccount: idoPDAs,
  //           userPdaAccount: userPDA,
  //           user: provider.publicKey,
  //           userTokenAccount: sourceAccount,
  //           idoTokenAccount: desAccount,
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           systemProgram: anchor.web3.SystemProgram.programId
  //         })
  //         .rpc();
  //       console.log("joinIDO success at tx: ", tx);
  //     } catch (error) {
  //       console.log(error);


  //     }
  //     let _userInfo = await program.account.pdaUserStats.fetch(userPDA);
  //     const _idoInfo = await program.account.idoAccount.fetch(idoPDAs);

  //     // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
  //     console.log(JSON.stringify(_userInfo));
  //     console.log(JSON.stringify(_idoInfo));

  // });

  // test modify tier
  // it("modify tier", async () => {
  //   const index = 1;
  //   const name = 'Lottery Winners Test'; 
  //   await program.methods.modifyTier(index, name).accounts({
  //     idoInfo: idoAccountTest,
  //     user: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   const idoInfo = await getInfoIdoAccount(program,IDO_TEST);

  //   console.log(JSON.stringify(idoInfo));

  //   const _tier = idoInfo.tiers[index];
  //   assert.equal(_tier.name, name, "tier name is changed");

  // })



  // it("modify_tiers", async () => {
  //   let idoPDAs =  getPdaIdo(program, ido_id);
  //   const names = ["Tier 1", "Tier 2","Tier 3", "Tier 4", "Tier 5", "Tier 6"]
  //   await program.methods.modifyTiers(names).accounts({
  //     idoAccount: idoPDAs,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();
  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   const tiers = idoInfo.tiers;
  //     for (let i = 0; i < tiers.length; i++) {
  //       const name = tiers[i].name;
  //       assert.equal(name, names[i], "tier name is changed");  
  //     }
  // })

  // it("modify_tier", async () => {

  //   const index = 0;
  //   const name = "Lottery Winners";

  //   let idoPDAs =  getPdaIdo(program, ido_id);

  //   const param ={
  //     tierIndex: index,
  //     name: name
  //   }
  //   await program.methods.modifyTier(param).accounts({
  //      idoAccount: idoPDAs,
  //      authority: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //  const tier = idoInfo.tiers[index];
  //   console.log(JSON.stringify(idoInfo));


  //   assert.equal(tier.name, name, "modify tier name");

  });

  // it("setup_release_token", async () => {

  //   const release_token = "Hv6634qu7ucXkaHDgcH3H5fUH1grmSNwpspYdCkSG7hK";

  //   const token_mint = new PublicKey(release_token);

  //   console.log("idoPDA: ", idoPDAs.toString());

  //   try {
  //     const releaseAtaAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);

  //     console.log("releaseAtaAccount:", releaseAtaAccount.toString());
  //     await program.methods.setupReleaseToken(token_mint).accounts({
  //       idoAccount: idoPDAs,
  //       adminAccount: adminPda,
  //       releaseTokenAccount: releaseAtaAccount,
  //       tokenMint: token_mint, 
  //       authority: provider.wallet.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram: web3.SystemProgram.programId,
  //     }).rpc()

  //   } catch (error) {
  //     console.log(error);

  //   }

  //  const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //  console.log(JSON.stringify(idoInfo));


  //   const _releaseToken = idoInfo.releaseToken;
  //   assert.equal(_releaseToken.toString(), release_token.toString(), "release token is token setup");
  //   // const _releaseTokenPair = idoInfo.releaseTokenPair;
  //   // assert.equal(_releaseTokenPair.toString(), pair_release_token.toString(), "release token pair is pair setup");
  // });

  

  // it("modify_tier_list", async () => {

  //   const nameList = ["Test1","Test2", "Test3"]

  //  await program.methods.modifyTiers(nameList, {
  //     accounts: {
  //       idoInfo: idoAccount.publicKey,
  //       user: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }
  //   });  


  //   const idoInfo = await getInfoIdoAccount(program, idoAccount.publicKey.toString());

  //   const _tiers = idoInfo.tiers;
  //   assert.equal(_tiers.length, nameList.length, "change tier");
  //  for (let index = 0; index < _tiers.length; index++) {
  //   const tier = _tiers[index];
  //   assert.equal(tier.name, nameList[index], "tier name is changed");
  //  }

  // })



  // 








  // it("withdraw_token_from_pda", async () => {

  //   let idoPDAs =  getPdaIdo(program, ido_id,"ido_pad");

  //   const token_mint = new PublicKey(raise_token_test);

  //   try {

  //     const idoAtaAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);
  //     const toAtaAccount = getAssociatedTokenAddressSync(token_mint, provider.publicKey, true);

  //     console.log("idoAtaAccount: ", idoAtaAccount.toString());
  //     console.log("toAtaAccount: ", toAtaAccount.toString());


  //     //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
  //     let amount = new BN(1.4 * LAMPORTS_PER_SOL);
  //     let tx = await program.methods.withdrawTokenFromPda(amount).accounts({
  //       tokenMint: token_mint,
  //       idoAccount: idoPDAs,
  //       authority: provider.publicKey,
  //       fromAta: idoAtaAccount,
  //       toAta: toAtaAccount,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       systemProgram: web3.SystemProgram.programId,
  //     }).rpc();
  //     console.log("joinIDO success at tx: ", tx);
  //   } catch (error) {
  //     console.log(error);
  //     // assert.equal(false, true, "transfer_spl_token_from_pda error");

  //   }
  // });


  // it("claim_token", async () => {
  //   // const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";

  //   let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");

  //   const token_mint = new PublicKey(release_token);

  //   const idoTokenReleaseAccount = getAssociatedTokenAddressSync(token_mint, idoPDA, true);
  //   const userTokenAccount = getAssociatedTokenAddressSync(token_mint, provider.publicKey, true);

  //   let userPDA =  getPdaUser(program.programId,  idoPDA, ido_id, provider.publicKey);

  //     try {
  //       // let idoInfo = await program.account.idoAccount.fetch(idoPDA);
  //       // let userInfo = await program.account.pdaUserStats.fetch(userPDA);       

  //       let index  = 3;
  //       let tx = await program.methods
  //         .claim(index).accounts({
  //           idoAccount: idoPDA,

  //           userPdaAccount: userPDA,
  //           user: provider.publicKey,
  //           userTokenAccount: userTokenAccount,
  //           idoTokenAccount: idoTokenReleaseAccount,
  //           tokenMint: token_mint,
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           associatedTokenProgram:  ASSOCIATED_TOKEN_PROGRAM_ID,
  //           systemProgram: anchor.web3.SystemProgram.programId
  //         })
  //         .rpc();
  //       console.log("claim success at tx: ", tx);
  //     } catch (error) {
  //       console.log(error);


  //     }
  //     let _userInfo = await program.account.pdaUserStats.fetch(userPDA);
  //     const _idoInfo = await program.account.idoAccount.fetch(idoPDA);

  //     // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
  //     console.log(JSON.stringify(_userInfo));
  //     console.log(JSON.stringify(_idoInfo));

  // });


// });

// describe("crowd funding testing release", () => {
//   it("setup_release_token", async () => {

//     const release_token_mint =  new PublicKey("Hv6634qu7ucXkaHDgcH3H5fUH1grmSNwpspYdCkSG7hK");
//     const token_account_release = getAssociatedTokenAddressSync(release_token_mint, idoPDAs, true);
//     console.log("token_account_release", token_account_release.toString() );


//     try {
//       let tx = await program.methods.setupReleaseToken(release_token_mint).accounts({

//         idoAccount: idoPDAs,
//         authority: provider.wallet.publicKey,
//         adminAccount: adminPda,
//         releaseTokenAccount: token_account_release,
//         tokenMint: release_token_mint,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//         systemProgram: SystemProgram.programId,

//       }).rpc();

//       console.log("transactions: ", tx);

//       const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
//       console.log(JSON.stringify(IdoInfo));

//       assert(IdoInfo.releaseToken.toString() == release_token_mint, "release token is setup");


//     } catch (error) {
//       console.log(error);

//     }
//   })
//   it('setup release', async () => {

//     const f1 = convertTimeTimeTo("2024/04/4 11:40:00");
//     const t1 = convertTimeTimeTo("2024/04/4 11:40:00");
//     const f2 = convertTimeTimeTo("2024/04/4 20:00:00");
//     const t2 = convertTimeTimeTo("2024/04/30 22:00:00");

//     const from_timestamps = [f1, f2];
//     const to_timestamps = [t1, t2];
//     const percents = [3333, 6664];

//     const params = {
//       fromTimestamps: from_timestamps,
//       toTimestamps: to_timestamps,
//       percents: percents
//     }
//     try {
//       let tx = await program.methods.setupReleases(params).accounts({
//         idoAccount: idoPDAs,
//         authority: provider.wallet.publicKey,
//         adminWallet: adminPda,
//         systemProgram: SystemProgram.programId,

//       }).rpc();

//       console.log("transactions: ", tx);

//       const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
//       console.log(JSON.stringify(IdoInfo));

//     } catch (error) {
//       console.log(error);

//     }
//   })

// })

const convertTimeTimeTo = (str_date: string): BN => {
  const date = moment(str_date, 'YYYY/MM/DD HH:mm:ss')
  const srt = moment(date).format("X");
  return new BN(srt || 0)
}

const convertTimeTimeToNumber = (str_date: string): number => {
  const date = moment(str_date, 'YYYY/MM/DD HH:mm:ss')
  const srt = moment(date).format("X");
  return Number(srt.toString())
}

