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
  sendAndConfirmRawTransaction,
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
            onClick={testTransferTokenToIdoPDA}
          >
            join Ido
          </button>
          <br></br>
          <button
            type="submit"
            className="cta-button submit-gif-button"
            onClick={withdrawSpl}
          >
            withdraw spl
          </button>
        </div>
      );
    }
  };
  const joinIDO = async () => {
    const ido_Address = "35kXkS3PNoBVWa9K1M3h84d5GjHCM595sx5vXyHzKQtD";
    const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
    const to_account_test = "Dm1sTcsXcWv71ePpNmZYQZm1oDe7KGQSKMKG5wCLr8vd";
    let toWalletPub = new PublicKey(to_account_test);

    const IDO_PUB = new PublicKey(ido_Address);
    try {
      const provider = getProvider();

      const program = new Program(idl_crowdfunding, program_ID, provider);

      const token_mint = new PublicKey(token_raise);

      const tokenAccount = getAssociatedTokenAddressSync(
        token_mint,
        toWalletPub
      );

      //   const tokenAccountInfo = await provider.connection.getParsedAccountInfo(tokenAccount);

      //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
      debugger;
      let tx = await program.methods
        .createTokenAccount()
        .accounts({
          payer: provider.publicKey,
          tokenAccount: tokenAccount,
          mint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
          rent: web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();
      console.log("joinIDO success at tx: ", tx);
    } catch (error) {
      console.log(error);
    }
  };

  const withdrawSpl = async () => {
    const ido_Address = "EW1LDGKJM6NTPsPR7e9rmQBmFUapLym3DFPXDsmpb5pr";
    const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";

    const IDO_PUB = new PublicKey(ido_Address);
    try {
      const provider = getProvider();

      const program = new Program(idl_crowdfunding, program_ID, provider);

      const token_mint = new PublicKey(token_raise);

      const fromAtaAccount = getAssociatedTokenAddressSync(
        token_mint,
        IDO_PUB,
        true
      );
      const toAtaAccount = getAssociatedTokenAddressSync(
        token_mint,
        provider.publicKey,
        true
      );

      console.log("fromAtaAccount: ", fromAtaAccount.toString());

      //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
      let amount = new BN(0.1 * LAMPORTS_PER_SOL);
      let tx = await program.methods
        .transferSplTokenFromPda(amount)
        .accounts({
          idoAccount: new PublicKey(ido_Address),
          payer: provider.publicKey,
          fromAta: fromAtaAccount,
          toAta: toAtaAccount,
          mint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
      console.log("joinIDO success at tx: ", tx);
    } catch (error) {
      console.log(error);
    }
  };

  const testTransferTokenToIdoPDA = async () => {
    const ido_Address = "EW1LDGKJM6NTPsPR7e9rmQBmFUapLym3DFPXDsmpb5pr";
    const token_raise = "3uWjtg9ZRjGbSzxYx4NgDLBwdFxhPLi9aArN9tiu6m8b";
    const to_account_test = "9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS";
    const toWalletPub = new PublicKey(ido_Address);
    try {
      const provider = getProvider();

      const program = new Program(idl_crowdfunding, program_ID, provider);

      const amount = new BN(1 * LAMPORTS_PER_SOL);
      const token_mint = new PublicKey(token_raise);

      const sourceAccount = getAssociatedTokenAddressSync(
        token_mint,
        provider.wallet.publicKey,
        true
      );

      const desAccount = getAssociatedTokenAddressSync(
        token_mint,
        toWalletPub,
        true
      ); //5uyDVW7MvLpjoqARJ8u6w8uRt39zjoh1yqHws8f8Vxve

      console.log("desAccount:", desAccount.toString());

      let tx = await program.methods
        .transferSplToken(amount)
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

  const testClaimToken = async () => {
    const provider = getProvider();

    const program = new Program(idl_crowdfunding, program_ID, provider);

    const release_token_test = "Hv6634qu7ucXkaHDgcH3H5fUH1grmSNwpspYdCkSG7hK";

    const IDO_PUB = new PublicKey(
      "EW1LDGKJM6NTPsPR7e9rmQBmFUapLym3DFPXDsmpb5pr"
    );
    const token_mint = new PublicKey(release_token_test);

    try {
      const idoAtaAccount = getAssociatedTokenAddressSync(
        token_mint,
        IDO_PUB,
        true
      );
      const toAtaAccount = getAssociatedTokenAddressSync(
        token_mint,
        provider.publicKey,
        true
      );

      console.log("idoAtaAccount: ", idoAtaAccount.toString());
      console.log("toAtaAccount: ", toAtaAccount.toString());

      //   console.log((tokenAccountInfo.value?.data).parsed.info.tokenAmount.amount);
      let amount = new BN(0.1 * LAMPORTS_PER_SOL);
      let tx = await program.methods
        .withdrawTokenFromPda(amount)
        .accounts({
          idoAccount: IDO_PUB,
          payer: provider.publicKey,
          fromAta: idoAtaAccount,
          toAta: toAtaAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
      console.log("joinIDO success at tx: ", tx);
    } catch (error) {
      console.log(error);
      // assert.equal(false, true, "transfer_spl_token_from_pda error");
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
