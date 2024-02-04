import { Container } from "react-bootstrap";
// import { WalletContextProvider } from "../../components/wallet-context-provider";
// import { PingButton } from "../../components/ping-button";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
  Signer,
} from "@solana/web3.js";
import {
  Program,
  AnchorProvider,
  web3,
  utils,
  BN,
} from "@project-serum/anchor";

import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import idl_crowdfunding from "../../idl/crowdfunding.json";
import { useState } from "react";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

const opts = {
  preflightCommitment: "processed",
  commitment: "processed",
};
const { SystemProgram } = web3;
const program_ID = new PublicKey(idl_crowdfunding.metadata.address);

const Homepage = () => {
  const network = clusterApiUrl("devnet");

  const [walletAddress, setWalletAddress] = useState(null);

  const programID = new PublicKey(idl_crowdfunding.metadata.address);

  const connectWallet = async () => {
    const { solana } = window;
    if (solana) {
      const response = await solana.connect();
      setWalletAddress(response.publicKey.toString());
      console.log("Connected with public key: ", response.publicKey.toString());
    }
  };
  const renderNotConnectedContainer = () => (
    <button
      className="cta-button connect-wallet-button"
      onClick={connectWallet}
    >
      Connect Wallet
    </button>
  );

  const getProvider = () => {
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new AnchorProvider(
      connection,
      window.solana,
      opts.preflightCommitment
    );
    return provider;
  };
  const renderConnectedContainer = () => {
    if (walletAddress !== null) {
      return (
        <div className="connected-container">
          <button
            type="submit"
            className="cta-button submit-gif-button"
            onClick={joinIDO}
          >
            Test JoinIdo
          </button>
        </div>
      );
    }
  };
  const joinIDO = async () => {
    const ido_Address = "ARRwPx2wrkn1MHvicoSam1tRFFkXxRKHQcqTgBBYsaut";
    const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
    const to_account_test = "9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS";
    let toWalletPub = new PublicKey(to_account_test);

   const IDO_PUB =  new PublicKey(ido_Address);
    try {

      const provider = getProvider();

      const program = new Program(idl_crowdfunding, program_ID, provider);

      const token_mint = new PublicKey(token_raise);


      const tokenAccount =  getAssociatedTokenAddressSync(token_mint,token_mint);

      
         let tx = await program.methods.createTokenAccount().accounts({
          payer: provider.publicKey,
          tokenMint: token_mint,
          associatedToken: tokenAccount,
          authority: IDO_PUB,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId
        }).rpc();

    } catch (error) {
      console.log(error);
    }
  };

  const testTransferToken = async () => {
    // const ido_Address = "2oQX3YwodPTp4vK6qeaUFZEnmsfswcNQAaeNkaadZnsG";
    const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
    const to_account_test = "9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS";
    const toWalletPub = new PublicKey(to_account_test);
    try {

      const provider = getProvider();

      const program = new Program(idl_crowdfunding, program_ID, provider);

      const amount = new BN(1 * LAMPORTS_PER_SOL);
      const token_mint = new PublicKey(token_raise);

      const sourceAccount =  getAssociatedTokenAddressSync( token_mint, provider.wallet.publicKey );
      const desAccount =  getAssociatedTokenAddressSync(token_mint,toWalletPub);


      let tx = await program.methods
        .transferSplTokens(amount)
        .accounts({
          from: provider.publicKey,
          fromAta: sourceAccount,
          toAta: desAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
      console.log("joinIDO success at tx: ", tx);
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <div className="hb-not-found py-6">
      <Container>
        <h1>Homepage</h1>
        <p>Click the button below to connect your wallet</p>

        <br></br>
        <div>
          {!walletAddress
            ? renderNotConnectedContainer()
            : renderConnectedContainer()}
        </div>
      </Container>
    </div>
  );
};
export default Homepage;
