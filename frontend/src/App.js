
import './App.css';
import { useEffect, useState } from 'react';
import idl from './idl/crowdfunding.json';
import {Connection, PublicKey, clusterApiUrl} from '@solana/web3.js';
import {Program, AnchorProvider, web3, utils, BN} from '@project-serum/anchor';
import {Buffer} from 'buffer';
window.Buffer = Buffer;

const programID = new PublicKey(idl.metadata.address)
const network = clusterApiUrl('devnet');
const opts = {
  preflightCommitment: "processed",
  commitment: "processed",
}
const {SystemProgram} = web3

const App = ()=> {

  const [walletAddress, setWalletAddress] = useState(null);
  const [campaigns, setCampaigns] = useState([]);
  const getProvider = () =>{
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new AnchorProvider(connection, window.solana, opts.preflightCommitment);
    return provider
  }

  const createCampaign = async()=>{
    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      const [campaign] = await PublicKey.findProgramAddressSync(
        [
          utils.bytes.utf8.encode('CAMPAIGN_DEMO'), 
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId)
        console.log("Campaign address: ", campaign);
        await program.rpc.create('campaign name', 'description', {
          accounts:{
            campaign: campaign,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId
          }
        })
        console.log('Create campaign with address: ', campaign.toString());
    } catch (error) {
        console.log("ERROR creating campaign account:", error);
    }
  }

  const getCampaign = async () =>{
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = getProvider();
    const program = new Program(idl, programID, provider)

    Promise.all((await connection.getProgramAccounts(programID)).map(async (campaign) =>({
  
      ...(await program.account.camPaign.fetch(campaign.pubkey)),
      pubkey: campaign.pubkey
    }))).then(campaigns =>(setCampaigns(campaigns))).catch(err=>{
      console.log(err)
    })
  }

  const donate = async (campaignAddress)=>{
    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      await program.rpc.donate( new BN(0.01* web3.LAMPORTS_PER_SOL), {
        accounts:{
          campaign: new PublicKey(campaignAddress),
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId
        }
      })
      console.log("Donate success:" , campaignAddress.toString());
      getCampaign()
    } catch (error) {
      console.log(error);
    }
  }

  const withdraw = async (campaignAddress)=>{
    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      await program.rpc.withdraw(new BN(0.01 * web3.LAMPORTS_PER_SOL), {
        accounts:{
          campaign: campaignAddress,
          user: provider.wallet.publicKey
        }
      })
      console.log("Withdraw success:" , campaignAddress.toString());
      getCampaign()
    } catch (error) {
      console.log(error);
    }

  }

  const checkIfWalletIsConnected = async ()=> {
    try {
        const {solana} = window;
 
        if(solana.isPhantom){
     
    
          const response = await solana.connect({onlyIfTrusted: true})

          console.log("Connected with public key: ", response.publicKey.toString());
          setWalletAddress(response.publicKey.toString());

        }else{
          alert("Sonala wallet is not found!")
        }
    } catch (error) {
      console.log(error)
    }
  };

  const connectWallet = async () =>{
    const {solana} = window;
    if(solana){
      const response = await solana.connect();
      setWalletAddress(response.publicKey.toString());
      console.log("Connected with public key: ", response.publicKey.toString());
    }
  }

  const renderNotConnectedContainer = ()=>(<button onClick={connectWallet}>Connect Wallet</button>)
  
  const renderConnectedContainer = ()=>(
    <>
    <button onClick={createCampaign}>Create Campaign</button>
    <button onClick={getCampaign}>Get Campaign</button>
    <br/>
    {campaigns.map(cap =>{
      return (
        <div key={cap.pubkey.toString()}>
      
          <h3>Campaign ID: {cap.pubkey.toString()}</h3>
          <p>Total Donate: {cap.totalDonated / web3.LAMPORTS_PER_SOL}</p>
          <p>{cap.name}</p>
          <p>{cap.description}</p>
          <p>admin: {cap.admin.toString()}</p>
          <button onClick={()=>donate(cap.pubkey)}>Donate</button>

          <button onClick={()=>withdraw(cap.pubkey)}>Withdraw</button>
        </div>
      )
    })}
    </>
  
  )




	useEffect(() => {
		const onLoad = async () => {
			await checkIfWalletIsConnected();
		};
		window.addEventListener("load", onLoad);
		return () => window.removeEventListener("load", onLoad);
	}, []);

  return <div className='App'> {walletAddress ? renderConnectedContainer() :renderNotConnectedContainer() }</div>

}

export default App;
