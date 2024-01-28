import * as anchor from "@coral-xyz/anchor";
import { Crowdfunding } from "../target/types/crowdfunding";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SOLANA_SCHEMA } from "@solana/web3.js";
import {Program, AnchorProvider, web3, utils, BN} from '@project-serum/anchor';
import { assert, expect } from "chai";

 const idoAccount = Keypair.generate();
  let IDO_TEST = "xqWbfdzkM5c91P72mU85qC74b5ME5qe83jNvFxpWW59";

describe("crowd funding testing", () => {

  const getInfoIdoAccount = async (program: any, idoAccountAddress: String)=>{
    const idoAccountPub  = new PublicKey(idoAccountAddress)
    let ido_info = await program.account.idoAccountInfo.fetch(idoAccountPub);
    return ido_info
  }
  const idoAccountTest = new PublicKey(IDO_TEST);

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;



  // it("initialize Ido program", async () => {

  //   const rate =  1000;
  //   const openTimestamp = 1705534720;
  //   const allocationDuration = 1705544720;
  //   const fcfsDuration = 1705545720;
  //   const cap = new BN(10*LAMPORTS_PER_SOL);

  //   const raiseToken = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
  //   const releaseToken ="3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";


  //  await program.methods.initialize(raiseToken, rate, openTimestamp, allocationDuration, fcfsDuration ,cap, releaseToken).accounts({

  //       idoInfo: idoAccount.publicKey,
  //       user: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //  }). signers([idoAccount]).rpc() ;


  //   console.log("IDO Account", idoAccount.publicKey.toString());
    
  //   const idoInfo = await getInfoIdoAccount(program, idoAccount.publicKey.toString());

  //   const _owner = idoInfo.owner;

  //   assert.equal(_owner.toString(), provider.wallet.publicKey.toString(), "Owner is user create ido account")

  // });
//test modify tier
// it("set_cap", async () => {
//   const cap = new BN(10*LAMPORTS_PER_SOL);
//   await program.methods.setCap(cap).accounts({
//     idoInfo: idoAccountTest,
//     user: provider.wallet.publicKey,
//     systemProgram: anchor.web3.SystemProgram.programId,
//   }).rpc();

//   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
//   const _cap = idoInfo._cap;
//   assert.equal(idoInfo._cap, _cap, "cap  is setup");

// })
// it("set rate", async () => {
//   const rate = 10000;

//   await program.methods.setRate(rate).accounts({
//     idoInfo: idoAccountTest,
//     user: provider.wallet.publicKey,
//     systemProgram: anchor.web3.SystemProgram.programId,
//   }).rpc();
  
//   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
  
//   const _rate = idoInfo._rate;
//   assert.equal(idoInfo._rate, _rate, "_rate  is setup");

// })


// it("set open timestamp", async () => {
//   const timestamp = (new Date().getTime() + 60*100*60) /1000;
//   console.log("timestamp=>", timestamp);
  
//   await program.methods.setOpenTimestamp(timestamp).accounts({
//     idoInfo: idoAccountTest,
//     user: provider.wallet.publicKey,
//     systemProgram: anchor.web3.SystemProgram.programId,
//     }).rpc();
  
  
//   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);

//   console.log(JSON.stringify(idoInfo));
  
//   const _open_timestamp = idoInfo._open_timestamp;
//   assert.equal(idoInfo._open_timestamp, _open_timestamp, "_open_timestamp  is setup");

// })
  
  
  // it("modify_rounds", async () => {
  //   if(!idoAccount) return;

  //   const nameList = ["Test round1", "Test prepare", "Test fsfs",] ;
  //   const durationSeconds = [3600, 1500, 9000];

  //   //check lai logic cho round class
  //  const classList = [{Allocation:{}},  {fcfsPrepare:{}},  {Fcfs:{}} ] 

  //   await program.methods.modifyRounds( nameList , durationSeconds , classList)
  //   .accounts({
  //      idoInfo: idoAccountTest,
  //      user: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc();  
  //   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
 
  //   const rounds = idoInfo.rounds;
  //   for (let i = 0; i < rounds.length; i++) {
  //       const r = rounds[i];
  //       assert.equal(r.name, nameList[i], "modify round name");
  //       assert.equal(r.durationSeconds, durationSeconds[i], "modify duration");
  //       // assert.equal(JSON.stringify(r.class), JSON.stringify(classList[i]), "modify class");
  //   }
  // });

  // it("modify_round", async () => {
  //   if(!idoAccount) return;
  //   const index = 0;
  //   const name = "Test round1";
  //   const durationSeconds = 600;

  //   //check lai logic cho round class
  //  const _class = {fcfsPrepare:{}}
  //   await program.methods.modifyRound(index, name, durationSeconds , _class).accounts({
  //      idoInfo: idoAccountTest,
  //      user: provider.wallet.publicKey,
  //      systemProgram: anchor.web3.SystemProgram.programId,
  //    }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
  //  const round = idoInfo.rounds[index];


   
  //   assert.equal(round.name, name, "modify round name");
 
  //   assert.equal(round.durationSeconds, durationSeconds, "modify duration");
  //   assert.equal(JSON.stringify(round.class), JSON.stringify(_class), "modify class");
  // });

  // it("setup release token", async () => {
  //   if(!idoAccount) return;
  //   const token = "GdgCpzyFdcZqvtwyX1phzNH8Q32vcNk47AqrZTSsciLs";
  //   const pair = "5yAX4HZEq9X2DumUkotrmPLPuFGVuMkWphUF2EcmtyBS";
   
  //   //test setupReleaseToken  -> OK
  //   await program.methods.setupReleaseToken(token, pair).accounts({
  //     idoInfo: idoAccountTest,
  //     user: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc()

  //  const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
  //  console.log(JSON.stringify(idoInfo));
   
  
  //   const _releaseToken = idoInfo.releaseToken;
  //   assert.equal(_releaseToken.toString(), token.toString(), "release token is token setup");
  //   const _releaseTokenPair = idoInfo.releaseTokenPair;
  //   assert.equal(_releaseTokenPair.toString(), pair.toString(), "release token pair is pair setup");
  // });


  // it("modify_round_allocations", async () => {
  //   if(!idoAccount) return;
  //   const index = 0;
  //   const tierAllocations = [new BN(0.1 * LAMPORTS_PER_SOL), new BN(0.02 * LAMPORTS_PER_SOL) ];
   
  //   //test setupReleaseToken  -> OK
  //   await program.methods.modifyRoundAllocations(index, tierAllocations).accounts( {
  //     idoInfo: idoAccountTest,
  //     user: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();
      
  //  const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
  //  const  roundAllocations = idoInfo.rounds[index].tierAllocations;
  //  for (let i = 0; i < roundAllocations.length; i++) {
  //     const tierAl = roundAllocations[i];
  //     assert.equal(tierAl.toString(), tierAllocations[i].toString(), "tier allocation is amount setup");
  //  }

  // });

  //test modify tier
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

  //   const names = ["Tier 1", "Tier 2","Tier 3", "Tier 4", "Tier 5", "Tier 6"]
  //   await program.methods.modifyTiers(names).accounts({
  //     idoInfo: idoAccountTest,
  //     user: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();
  //   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
  //   const tiers = idoInfo.tiers;
  //     for (let i = 0; i < tiers.length; i++) {
  //       const name = tiers[i].name;
  //       assert.equal(name, names[i], "tier name is changed");  
  //     }
  // })

  // it("setup_releases", async () => {

  //  const from_timestamps = [1705514720, 1705536720 , 1705574720]
  //  const to_timestamps = [1705734740, 1705834720, 1705934720]
  //  const percents = [40, 30, 30];
  //  await program.methods.setupReleases(from_timestamps, to_timestamps, percents).accounts({
  //   idoInfo: idoAccountTest,
  //   user: provider.wallet.publicKey,
  //   systemProgram: anchor.web3.SystemProgram.programId,
  //  }).rpc();

  //   const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
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

  it("modify_tier_allocated", async () => {
    const index = 2;
    const add1 = "CjZ4nLk8RLmk89hhFZhJT6QNRUUcgGPqMgBMZ5x3re67";
    const add2 = "9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS";
    const add3 = "HwzR86jCMDsddsNY6xYNk6qC8kSvTaEMFSQmemCWsyxS";
    const add4 = "Bf2VHp1uBLAUvuWDVLSdYUeJ5dZcJonBT93kjHgEznoQ";

    const addresses = [add1,add2, add3, add4]
   const remove = false;
    await program.methods.modifyTierAllocated(index, addresses, remove).accounts({
      idoInfo: idoAccountTest,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
  
    // await program.methods.modifyTierAllocated(index, [add4], true).accounts({
    //   idoInfo: idoAccountTest,
    //   user: provider.wallet.publicKey,
    //   systemProgram: anchor.web3.SystemProgram.programId,
    // }).rpc();
 
    const idoInfo = await getInfoIdoAccount(program, IDO_TEST);
    console.log(JSON.stringify(idoInfo));
    
    
      const tier = idoInfo.tiers[index];
    
      for (let i = 0; i < addresses.length; i++) {
        const al = tier.allocated.find(al => al.address.toString() == addresses[i]);
        assert.equal(al.allocated, !remove, "address has allocated change");

        
      }
  })




  // it("set_closed", async () => {
  //   const closed = true;
  //   await program.rpc.setClosed(closed, {
  //     accounts: {
  //       idoInfo: idoAccount.publicKey,
  //       user: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }
  //   });  
 
  //   const idoInfo = await getInfoIdoAccount(program, idoAccount.publicKey.toString());
  //   assert.equal(idoInfo.closed, closed, "state project change");
   
  // })



  
});
