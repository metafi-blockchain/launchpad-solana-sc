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
const AUTHORITY_IDO = "ido_pad";
const AUTHORITY_ADMIN = "admin_ido";
const AUTHORITY_USER = "wl_ido_pad";

const getInfoIdoAccount = async (program: any, idoAccountAddress: String) => {
  const idoAccountPub = new PublicKey(idoAccountAddress)
  let ido_info = await program.account.idoAccount.fetch(idoAccountPub);
  return ido_info
}

const getPdaIdo = (program: any, ido_id: number) => {
  let idoIdBuff = new anchor.BN(ido_id);
  const [idoPDAs, _] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(AUTHORITY_IDO),
      idoIdBuff.toBuffer("le", 8),
    ],
    program.programId);
  return idoPDAs;
}
const getPdaAdmin = (program: any, ido_pda: PublicKey) => {

  const [idoPDAs, _] = PublicKey.findProgramAddressSync(
    [
      utils.bytes.utf8.encode(AUTHORITY_ADMIN),
      ido_pda.toBuffer(),
    ],
    program.programId);
  return idoPDAs;
}
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
const ido_id = 7;


let idoPDAs = getPdaIdo(program, ido_id);
let adminPda = getPdaAdmin(program, idoPDAs)



describe("crowd funding testing", () => {



  // const idoAccountTest = new PublicKey(IDO_TEST);

  // Configure the client to use the local cluster.


  console.log("idoPDAs:", idoPDAs.toString());

  let raise_token = raise_token_test
  // it("initialize Ido program", async () => {

  //   const rate =  1000000;
  //   const openTimestamp = convertTimeTimeTo("2024/03/28 12:45:00");
  //   const allocationDuration =  12*60*60;
  //   const fcfsDuration =  120*60;

  //   const cap = new BN(10*LAMPORTS_PER_SOL);

  //   const releaseToken ="DG9UcawWuzsnRpMDaY67kDn3SHbbQQnKEW4to3UQWLJC";

  //   let token_mint = new PublicKey("8xRoWyiPKGzqWPwh81HAGaytxzy6bEgN58Uh7LiHvMru")
  //   const raiseTokenAccount =  getAssociatedTokenAddressSync(token_mint, idoPDAs, true);


  //   console.log("associatedToken: ", raiseTokenAccount.toString());
  //   console.log(provider.wallet.publicKey.toString());
  //   console.log("idoPDAs: " ,idoPDAs.toString());
  //   console.log("adminPda: ", adminPda.toString());
  //   console.log("token_mint: ",token_mint.toString());

  //   try {
  //     await program.methods.initialize(raise_token_test, rate, openTimestamp, allocationDuration, fcfsDuration ,cap, releaseToken, new BN(ido_id)).accounts({
  //       idoAccount: idoPDAs,
  //       idoAdminAccount: adminPda,
  //       authority: provider.publicKey,
  //       tokenMint: token_mint,
  //       tokenAccount: raiseTokenAccount,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //   }).rpc() ;
  //   } catch (error) {
  //     console.log(error);

  //   }

  //   // console.log("IDO Account", idoPDAs.publicKey.toString());

  //   let idoInfo = await program.account.idoAccount.fetch(idoPDAs);
  //   let adminPdaInfo = await program.account.adminAccount.fetch(adminPda);
  //   console.log(JSON.stringify(idoInfo));
  //   console.log("==========");
  //   console.log(JSON.stringify(adminPdaInfo));



  //   assert.equal(idoInfo.authority.toString(), adminPda.toString(), "Owner is user create ido account")
  //   assert.equal(adminPdaInfo.authority.toString(), provider.publicKey.toString(), "Owner is user create ido account")
  // });



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


  
  //     const cap = new BN(100).mul(new BN(LAMPORTS_PER_SOL));
  //     await program.methods.setCap(cap).accounts({
  //       idoAccount: idoPDAs,
  //       adminWallet: adminPda,
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

  //   await program.methods.setRate(rate).accounts({
  //     idoAccount: idoPDAs,
  //     adminWallet: adminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   console.log(JSON.stringify(idoInfo));


  //   const _rate = idoInfo._rate;
  //   assert.equal(idoInfo._rate, _rate, "_rate  is setup");

  // })


  it("set open timestamp", async () => {


    const timestamp = convertTimeTimeTo("2024/04/1 21:00:00");
    console.log("timestamp", timestamp.toString() );


    await program.methods.setOpenTimestamp(timestamp).accounts({
      idoAccount: idoPDAs,
      adminWallet: adminPda,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();


    const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
    const _open_timestamp = idoInfo._open_timestamp;
    assert.equal(idoInfo._open_timestamp, _open_timestamp, "_open_timestamp  is setup");

  })


  it("modify_rounds", async () => {

    const nameList = ["Allocation", "FSCS prepare", " FCFS",] ;
    const durationSeconds = [15*60, 60, 60*10];

    //check lai logic cho round class
   const classList = [{allocation:{}},  {fcfsPrepare:{}},  {fcfs:{}} ]

    const tx  = await program.methods.modifyRounds( nameList , durationSeconds , classList)
    .accounts({
       idoAccount: idoPDAs,
       adminWallet: adminPda,
       authority: provider.wallet.publicKey,
       systemProgram: anchor.web3.SystemProgram.programId,
     }).rpc();  
     console.log(tx);
    const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
     console.log(JSON.stringify(idoInfo));


    const rounds = idoInfo.rounds;
    for (let i = 0; i < rounds.length; i++) {
        const r = rounds[i];
        assert.equal(r.name, nameList[i], "modify round name");
        assert.equal(r.durationSeconds, durationSeconds[i], "modify duration");
        // assert.equal(JSON.stringify(r.class), JSON.stringify(classList[i]), "modify class");
    }
  });

  // it("modify_round", async () => {


  //   const index = 1;
  //   const name = "Test round1";
  //   const durationSeconds = 60;

  //   //check lai logic cho round class
  //  const _class = { allocation: {} }
  //   await program.methods.modifyRound(index, name, durationSeconds , _class).accounts({
  //     idoAccount: idoPDAs,
  //     adminWallet: adminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //  const round = idoInfo.rounds[index];



  //   assert.equal(round.name, name, "modify round name");

  //   assert.equal(round.durationSeconds, durationSeconds, "modify duration");
  //   assert.equal(JSON.stringify(round.class), JSON.stringify(_class), "modify class");
  // });






  it("modify_round_allocations", async () => {

    // let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");

    const round_index = 0;
    const tierAllocations = [new BN(2 * LAMPORTS_PER_SOL), new BN(2 * LAMPORTS_PER_SOL) , new BN(5 * LAMPORTS_PER_SOL)];
    // const tierAllocations = [new BN(0), new BN(0) , new BN(0)];

    try {
      await program.methods.modifyRoundAllocations(round_index, tierAllocations).accounts( {
        idoAccount: idoPDAs,
        adminWallet: adminPda,
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


  it("modify_tier_allocated_one", async () => {
    const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
    console.log(JSON.stringify(IdoInfo));


    const add1 = "G8e8xCUub4eS2s8QfMihLr8uhMdzkUfFPiMkqfpMoKFD";
    let user1 = new PublicKey(add1)
    let userPDA = getPdaUser(program.programId, idoPDAs, user1);
    const tier = 1;

    console.log("userPDA: ", userPDA.toString());
    console.log("idoPDA: ", idoPDAs.toString());

    const remove = false;
    try {
      let tx = await program.methods.modifyTierAllocatedOne(tier, user1, remove).accounts({

        idoAccount: idoPDAs,
        authority: provider.wallet.publicKey,
        adminWallet: adminPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        userIdoAccount: userPDA
      }).rpc();

      console.log("transactions: ", tx);

      let userInfo = await program.account.pdaUserStats.fetch(userPDA);

      // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
      console.log(JSON.stringify(userInfo));

      assert.equal(userInfo.tierIndex, tier, `${user1} is add in tier ${tier}`);

      assert.equal(userInfo.allocated, !remove, `address has allocated change: ${!remove}`);
      assert.equal(userInfo.address.toString(), user1.toString(), `${user1} is add white list`);
    } catch (error) {
      console.log(error);

    }
  })
  // it("modify_tier_allocated_multi", async () => {
  //     const add1 = "B4Sho4nv3f7kJNo33a3cmYEKCUetCm6tgHqatkcxiaA8";
  //     const add2 = "Dm1sTcsXcWv71ePpNmZYQZm1oDe7KGQSKMKG5wCLr8vd";

  //     let user1 = new PublicKey(add1)
  //     let user2 = new PublicKey(add2)
  //     let userPDA1 =  getPdaUser(program.programId,  idoPDAs, user1);
  //     let userPDA2 =  getPdaUser(program.programId,  idoPDAs, user2);
  //     const tier = 2;

  //     console.log("userPDA: ", userPDA1.toString());
  //     console.log("idoPDA: ", idoPDAs.toString());

  //    const remove = true;
  //    try {
  //     let tx = await program.methods.modifyTierAllocatedMulti(tier,[ user1, user2], remove).accounts({

  //       idoAccount: idoPDAs,
  //       authority: provider.wallet.publicKey,
  //       adminWallet: adminPda,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }
  //     ).remainingAccounts([{
  //       pubkey: userPDA1,
  //       isWritable: true,
  //       isSigner: false
  //     },
  //     {
  //       pubkey: userPDA2,
  //       isWritable: true,
  //       isSigner: false
  //     }]).rpc();

  //     console.log("transaction: ", tx);

  //     let userInfo = await program.account.pdaUserStats.fetch(userPDA1);
  //     console.log(JSON.stringify(userInfo));

  //     // assert.equal(userInfo.tierIndex, tier, `${user1} is add in tier ${tier}`);

  //     // assert.equal(userInfo.allocated, !remove, `address has allocated change: ${!remove}`);
  //     // assert.equal(userInfo.address.toString(), user1.toString(), `${user1} is add white list`);
  //    } catch (error) {
  //     console.log(error);

  //    }
  //   })

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
  //   let idoPDAs =  getPdaIdo(program, ido_id,"ido_pad");
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

  //   let idoPDAs =  getPdaIdo(program, ido_id,"ido_pad");

  //   await program.methods.modifyTier(index, name).accounts({
  //      idoAccount: idoPDAs,
  //      authority: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //  const tier = idoInfo.tiers[index];
  //   console.log(JSON.stringify(idoInfo));


  //   assert.equal(tier.name, name, "modify tier name");

  // });

  // it("setup_release_token", async () => {

  //   const release_token = "Hv6634qu7ucXkaHDgcH3H5fUH1grmSNwpspYdCkSG7hK";

  //   const token_mint = new PublicKey(release_token);

  //   console.log("idoPDA: ", idoPDAs.toString());

  //   try {
  //     const releaseAtaAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);

  //     console.log("releaseAtaAccount:", releaseAtaAccount.toString());
  //     await program.methods.setupReleaseToken(token_mint).accounts({
  //       idoAccount: idoPDAs,
  //       adminWallet: adminPda,
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

  //  await program.rpc.modifyTiers(nameList, {
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


});

describe("crowd funding testing release", () => {
  // it("setup_release_token", async () => {

  //   const release_token_mint =  new PublicKey(release_token);
  //   const token_account_release = getAssociatedTokenAddressSync(release_token_mint, idoPDAs, true);
  //   console.log("token_account_release", token_account_release.toString() );


  //   try {
  //     let tx = await program.methods.setupReleaseToken(release_token_mint).accounts({

  //       idoAccount: idoPDAs,
  //       authority: provider.wallet.publicKey,
  //       adminWallet: adminPda,
  //       releaseTokenAccount: token_account_release,
  //       tokenMint: release_token_mint,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram: SystemProgram.programId,

  //     }).rpc();

  //     console.log("transactions: ", tx);

  //     const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //     console.log(JSON.stringify(IdoInfo));

  //     assert(IdoInfo.releaseToken.toString() == release_token_mint, "release token is setup");


  //   } catch (error) {
  //     console.log(error);

  //   }
  // })
  it('setup release', async () => {

    const f1 = convertTimeTimeTo("2024/03/31 15:15:00");
    const t1 = convertTimeTimeTo("2024/03/31 15:25:00");
    const f2 = convertTimeTimeTo("2024/04/1 20:00:00");
    const t2 = convertTimeTimeTo("2024/04/30 22:00:00");

    const from_timestamps = [f1, f2];
    const to_timestamps = [t1, t2];
    const percents = [3333, 6664];
    try {
      let tx = await program.methods.setupReleases(from_timestamps, to_timestamps, percents).accounts({
        idoAccount: idoPDAs,
        authority: provider.wallet.publicKey,
        adminWallet: adminPda,
        systemProgram: SystemProgram.programId,

      }).rpc();

      console.log("transactions: ", tx);

      const IdoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
      console.log(JSON.stringify(IdoInfo));

    } catch (error) {
      console.log(error);

    }
  })

})


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

