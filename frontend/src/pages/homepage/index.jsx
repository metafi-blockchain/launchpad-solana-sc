import { Button, Container } from "react-bootstrap";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useEffect, useState } from "react";
import { getAddressInfo } from "../../utils/solana.web3";

const Homepage = () => {
  const { select, wallets, publicKey, disconnect } = useWallet();

  const [balance, setBalance] = useState(0);

  const getBalanceOfWallet = async () => {
    try {
      const res = await getAddressInfo(publicKey);
      console.log("11111", res);
      setBalance(res);
    } catch (error) {}
  };

  useEffect(() => {
    if (publicKey) {
      getBalanceOfWallet(publicKey);
    } else {
      setBalance(0);
    }
  }, [publicKey]);

  return (
    <div className="hb-not-found py-6">
      <Container>
        <h1>Homepage</h1>
        {/* default connect button */}
        <WalletMultiButton />
        {/* custom connect button */}
        {!publicKey ? (
          wallets.filter((wallet) => wallet.readyState === "Installed").length >
          0 ? (
            wallets
              .filter((wallet) => wallet.readyState === "Installed")
              .map((wallet) => (
                <Button
                  key={wallet.adapter.name}
                  onClick={() => select(wallet.adapter.name)}
                  w="64"
                  size="lg"
                  fontSize="md"
                  // leftIcon={
                  //   <Image
                  //     src={wallet.adapter.icon}
                  //     alt={wallet.adapter.name}
                  //     h={6}
                  //     w={6}
                  //   />
                  // }
                >
                  {wallet.adapter.name}
                </Button>
              ))
          ) : (
            <h3>No wallet found. Please download a supported Solana wallet</h3>
          )
        ) : (
          <div>
            <h2>{publicKey.toBase58()}</h2>
            <Button onClick={disconnect}>Disconnect wallet</Button>
          </div>
        )}
      </Container>
    </div>
  );
};
export default Homepage;
