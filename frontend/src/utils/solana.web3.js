import * as solanaWeb3 from "@solana/web3.js";

const DEV_NET = solanaWeb3.clusterApiUrl("devnet");
const solanaConnection = new solanaWeb3.Connection(DEV_NET);

export const getAddressInfo = async (address) => {
  const pubKey = new solanaWeb3.PublicKey(address);
  const accountBalance = await solanaConnection.getBalance(pubKey);

  return accountBalance;
};
