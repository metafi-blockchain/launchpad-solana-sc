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
import { getAllocationRemaining, getPdaUser } from "./ido.ultil";
import { IdoAccount } from "./ido_type";
// let IDO_TEST = "ARRwPx2wrkn1MHvicoSam1tRFFkXxRKHQcqTgBBYsaut";


describe("crowd funding testing", () => {
  let programId = new PublicKey("6KMVQWmTXpd36ryMi7i91yeLsgM6S4BiaTX3UczEkvqq");

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

  const getInfoIdoAccount = async (program: any, idoAccountAddress: String) => {
    const idoAccountPub = new PublicKey(idoAccountAddress)
    let ido_info = await program.account.idoAccount.fetch(idoAccountPub);
    return ido_info
  }

  const getPdaIdo =  (program: any, ido_id: number, userAdmin: PublicKey) => {
    const idoIdBuff = Buffer.alloc(4);
    idoIdBuff.writeUInt32LE(ido_id, 0)
    const [idoPDAs, _] = PublicKey.findProgramAddressSync(
      [
        utils.bytes.utf8.encode("ido_pad"),
        userAdmin.toBuffer(),
        idoIdBuff,
      ],
      program.programId);
    return idoPDAs;
  }
  const getPdaAdmin =  (program: any, ido_id: number) => {
    const idoIdBuff = Buffer.alloc(4);
    idoIdBuff.writeUInt32LE(ido_id, 0)
    const [idoPDAs, _] = PublicKey.findProgramAddressSync(
      [
        utils.bytes.utf8.encode("admin_ido"),
        SystemProgram.programId.toBuffer(),
        idoIdBuff,
      ],
      program.programId);
    return idoPDAs;
  }
  // const getPdaUser =  (programId: PublicKey, idoPDA: PublicKey, user: PublicKey,seed: string) => {
  //   const idoIdBuff = Buffer.alloc(4);
  //   idoIdBuff.writeUInt32LE(ido_id, 0)
  //   const [idoPDAs, _] = PublicKey.findProgramAddressSync(
  //     [
  //       utils.bytes.utf8.encode(seed),
  //       user.toBuffer(),
  //       idoPDA.toBuffer(),
  //     ],
  //     programId);
  //   return idoPDAs;
  // }
  // const idoAccountTest = new PublicKey(IDO_TEST);

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

  const raise_token_test = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
  const release_token = "Hv6634qu7ucXkaHDgcH3H5fUH1grmSNwpspYdCkSG7hK";
  const ido_id = 99;

  let adminPda = getPdaAdmin(program,ido_id )

  let idoPDAs =  getPdaIdo(program, ido_id, adminPda);

  it("initialize Ido program", async () => {

    const rate =  1000;
    const openTimestamp = convertTimeTimeTo("2024/02/20 15:45:00");
    const allocationDuration =  12*60*60*60;
    const fcfsDuration =  12*3600*60;
    const cap = new BN(10*LAMPORTS_PER_SOL);




    const releaseToken ="DG9UcawWuzsnRpMDaY67kDn3SHbbQQnKEW4to3UQWLJC";

    let token_mint = new PublicKey("3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b")
    const raiseTokenAccount =  getAssociatedTokenAddressSync(token_mint, idoPDAs, true);


    console.log("associatedToken: ", raiseTokenAccount.toString());
    console.log(provider.wallet.publicKey.toString());
    console.log("idoPDAs: " ,idoPDAs.toString());
    console.log("adminPda: ", adminPda.toString());
    console.log("token_mint: ",token_mint.toString());

    try {
      await program.methods.initialize( raise_token_test, rate, openTimestamp, allocationDuration, fcfsDuration ,cap, releaseToken, ido_id).accounts({
        idoAccount: idoPDAs,
        idoAdminAccount: adminPda,
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
    let adminPdaInfo = await program.account.adminAccount.fetch(adminPda);
    console.log(JSON.stringify(idoInfo));
    console.log("==========");
    console.log(JSON.stringify(adminPdaInfo));



    assert.equal(idoInfo.authority.toString(), adminPda.toString(), "Owner is user create ido account")
    assert.equal(adminPdaInfo.authority.toString(), provider.publicKey.toString(), "Owner is user create ido account")
  });



  // it("update_admin_ido", async () => {

  //   let adminPda = getPdaAdmin(program,ido_id )

  //   let idoPDAs =  getPdaIdo(program, ido_id, adminPda);

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
  
  it("set_cap", async () => {
    try {
      let adminPda = getPdaAdmin(program,ido_id )

      let idoPDAs =  getPdaIdo(program, ido_id, adminPda);
      const cap = new BN(10*LAMPORTS_PER_SOL);
      await program.methods.setCap(cap).accounts({
        idoAccount: idoPDAs,
        adminWallet: adminPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
  
      const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
      const _cap = idoInfo._cap;
      assert.equal(idoInfo._cap, _cap, "cap  is setup");
    } catch (error) {
      console.log(error);
      
    }


  })
  it("set rate", async () => {
    const rate = 10000;

    await program.methods.setRate(rate).accounts({
      idoAccount: idoPDAs,
      adminWallet: adminPda,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
    console.log(JSON.stringify(idoInfo));
    

    const _rate = idoInfo._rate;
    assert.equal(idoInfo._rate, _rate, "_rate  is setup");

  })


  // it("set open timestamp", async () => {


  //   const timestamp = convertTimeTimeTo("2024/02/21 15:15:00");

  //   await program.methods.setOpenTimestamp(timestamp).accounts({
  //     idoAccount: idoPDAs,
  //     adminWallet: adminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     }).rpc();


  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   const _open_timestamp = idoInfo._open_timestamp;
  //   assert.equal(idoInfo._open_timestamp, _open_timestamp, "_open_timestamp  is setup");

  // })


  // it("modify_rounds", async () => {



  //   const nameList = ["Test round1", "Test prepare", "Test fsfs",] ;
  //   const durationSeconds = [36000, 15000, 90000];

  //   //check lai logic cho round class
  //  const classList = [{allocation:{}},  {fcfsPrepare:{}},  {fcfs:{}} ]

  //   await program.methods.modifyRounds( nameList , durationSeconds , classList)
  //   .accounts({
  //      idoAccount: idoPDAs,
  //      adminWallet: adminPda,
  //      authority: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc();  
  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());

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
  //   const durationSeconds = 60000;

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






  // it("modify_round_allocations", async () => {

  //   let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");

  //   const round_index = 0;
  //   const tierAllocations = [new BN(2 * LAMPORTS_PER_SOL), new BN(3 * LAMPORTS_PER_SOL) , new BN(4 * LAMPORTS_PER_SOL)];

  //   try {
  //     await program.methods.modifyRoundAllocations(round_index, tierAllocations).accounts( {
  //       idoAccount: idoPDA,
  //       authority: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }).rpc();
  
  //    const idoInfo = await getInfoIdoAccount(program, idoPDA.toString());
  //    console.log(JSON.stringify(idoInfo));
  //    const  roundAllocations = idoInfo.rounds[round_index].tierAllocations;
  //    for (let i = 0; i < roundAllocations.length; i++) {
  //       const tierAl = roundAllocations[i];
  //       assert.equal(tierAl.toString(), tierAllocations[i].toString(), "tier allocation is amount setup");
  //    }
  //   } catch (error) {
  //     console.log(error);
      
  //   }
  
  // });


  // it("modify_tier_allocated_one", async () => {
  //   const add1 = "B4Sho4nv3f7kJNo33a3cmYEKCUetCm6tgHqatkcxiaA8";
  //   let user1 = new PublicKey(add1)
  //   let userPDA =  getPdaUser(program.programId,  idoPDAs, ido_id, user1);
  //   const tier = 1;
   
  //   console.log("userPDA: ", userPDA.toString());
  //   console.log("idoPDA: ", idoPDAs.toString());
   
  //  const remove = false;
  //  try {
  //   await program.methods.modifyTierAllocatedOne(tier, user1, remove).accounts({
      
  //     idoAccount: idoPDAs,
  //     authority: provider.wallet.publicKey,
  //     adminWallet: adminPda,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     userIdoAccount: userPDA
  //   }).rpc();

  //   let userInfo = await program.account.pdaUserStats.fetch(userPDA);

  //   // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
  //   console.log(JSON.stringify(userInfo));
    
  //   assert.equal(userInfo.tierIndex, tier, `${user1} is add in tier ${tier}`);
 
  //   assert.equal(userInfo.allocated, !remove, `address has allocated change: ${!remove}`);
  //   assert.equal(userInfo.address.toString(), user1.toString(), `${user1} is add white list`);
  //  } catch (error) {
  //   console.log(error);
    
  //  }
  // })

  // it("joinIdo", async () => {
  //   // const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";

  //   let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");


  //   const token_mint = new PublicKey(raise_token_test);



  //   const desAccount = getAssociatedTokenAddressSync(token_mint, idoPDA, true);

  //   console.log("desAccount: " , desAccount.toString());
    
  //   const sourceAccount = getAssociatedTokenAddressSync(token_mint, provider.publicKey, true);

  //   let userPDA =  getPdaUser(program.programId,  idoPDA, ido_id, provider.publicKey);

  //     try {
  //       let idoInfo = await program.account.idoAccount.fetch(idoPDA);
  //       let userInfo = await program.account.pdaUserStats.fetch(userPDA);

  //       const infoWallet = getAllocationRemaining(1, userInfo.tierIndex, <IdoAccount><unknown>idoInfo ,  userInfo)

  //       console.log(JSON.stringify(infoWallet));
        

  //       //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
  //       let amount = new BN(0.1 * LAMPORTS_PER_SOL);
  //       let tx = await program.methods
  //         .participate(amount).accounts({
  //           idoAccount: idoPDA,
  //           userPdaAccount: userPDA,
  //           user: provider.publicKey,
  //           depositTokenAccount: sourceAccount,
  //           receiveTokenAccount: desAccount,
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           systemProgram: anchor.web3.SystemProgram.programId
  //         })
  //         .rpc();
  //       console.log("joinIDO success at tx: ", tx);
  //     } catch (error) {
  //       console.log(error);
        
        
  //     }
  //     let _userInfo = await program.account.pdaUserStats.fetch(userPDA);
  //     const _idoInfo = await program.account.idoAccount.fetch(idoPDA);

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
  //   const pair_release_token = "5yAX4HZEq9X2DumUkotrmPLPuFGVuMkWphUF2EcmtyBS";
 
  //   const token_mint = new PublicKey(release_token);

  //   console.log("idoPDA: ", idoPDAs.toString());
    
  //   try {
  //     const releaseAtaAccount = getAssociatedTokenAddressSync(token_mint, idoPDAs, true);

  //     console.log("releaseAtaAccount:", releaseAtaAccount.toString());
  //     await program.methods.setupReleaseToken(release_token, pair_release_token).accounts({
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
  //   const _releaseTokenPair = idoInfo.releaseTokenPair;
  //   assert.equal(_releaseTokenPair.toString(), pair_release_token.toString(), "release token pair is pair setup");
  // });

  // it("setup_releases", async () => {
  //   let idoPDAs =  getPdaIdo(program, ido_id,"ido_pad");
  //  const from_timestamps = [1705514720, 1705536720 , 1705574720]
  //  const to_timestamps = [1705734740, 1705834720, 1705934720]
  //  const percents = [40, 30, 30];
  //  await program.methods.setupReleases(from_timestamps, to_timestamps, percents).accounts({
  //   idoAccount: idoPDAs,
  //   authority: provider.wallet.publicKey,
  //   systemProgram: anchor.web3.SystemProgram.programId,
  //  }).rpc();

  //   const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   const releases = idoInfo.releases;
  //   assert.equal(releases.length, 3, "releases length is 3");
  //   // assert.equal(_tier.name, name, "tier name is name change");

  // })

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



  //  it("modify_tier_allocated_multi", async () => {
  //   const add1 = "CjZ4nLk8RLmk89hhFZhJT6QNRUUcgGPqMgBMZ5x3re67";
  //   let user1 = new PublicKey(add1)
  //   let idoPDA =  getPdaIdo(program, ido_id,"ido_pad");
  //   // let userPDA =  getPdaUser(program.programId, idoPDA, ido_id, user1);
  //   const index = 0;
   
  //   const add2 = "9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS";
  //   const add3 = "HwzR86jCMDsddsNY6xYNk6qC8kSvTaEMFSQmemCWsyxS";
  //   const add4 = "Bf2VHp1uBLAUvuWDVLSdYUeJ5dZcJonBT93kjHgEznoQ";
  //   // let idoAccountTest = new PublicKey('Fs2deA3RCKoeT8NMfUb6KdRyx5brwnwWQJfycWnwQw5V');

  //   const addresses = [add1,add2, add3, add4]

  //   // console.log("userPDA: ", userPDA.toString());
  //   // console.log("idoPDA: ", idoPDA.toString());
   
  //  const remove = true;
  //  try {
  //   await program.methods.modifyTierAllocatedMulti(index, addresses, remove).accounts({
      
  //     idoAccount: idoPDA,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   // let userInfo = await program.account.pdaUserStats.fetch(userPDA);

  //   // const userInfo = await getInfoIdoAccount(program, userPDA.toString());
  //   // console.log(JSON.stringify(userInfo));
    
 
  //   // assert.equal(userInfo.allocated, !remove, `address has allocated change: ${!remove}`);
  //   // assert.equal(userInfo.address.toString(), user1.toString(), `${user1} is add white list`);
  //  } catch (error) {
  //   console.log(error);
    
  //  }
  // })


  //   // const idoInfo = await getInfoIdoAccount(program, idoPDAs.toString());
  //   // console.log(JSON.stringify(idoInfo));


  //   //   const tier = idoInfo.tiers[index];

  //   //   for (let i = 0; i < addresses.length; i++) {
  //   //     const al = tier.allocated.find(al => al.address.toString() == addresses[i]);
  //   //     assert.equal(al.allocated, !remove, "address has allocated change");


  //   //   }
  // })





  


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

const convertTimeTimeTo  = (str_date: string): number=> {
  const date  = moment(str_date,'yyyy/MM/dd HH:mm:ss')
  const srt = moment(date).format("X");  
  return Number(srt) || 0
}

