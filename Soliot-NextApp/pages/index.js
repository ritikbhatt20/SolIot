import { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
  BackpackWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import Header from "../components/Header";
// import style from "../styles/Home.module.css";
import "@solana/wallet-adapter-react-ui/styles.css";
import { AppProvider } from "../context/context";
import InitializeRegistry from "../components/InitializeRegistry";
import RegisterNode from "../components/RegisterNode";
import UpdateNode from "../components/UpdateNode";
import NodeStatusDetails from "../components/NodeStatusDetails";
import { clusterApiUrl } from "@solana/web3.js";
// import "../styles/globals.css";

export default function Home() {
  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SolflareWalletAdapter(),
      new BackpackWalletAdapter(),
    ],
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect={true}>
        <WalletModalProvider>
          <AppProvider>
            <div className="min-h-screen bg-slate-800">
              <Header />
              {/* <div> */}
              {/* <InitializeRegistry />  */}
              {/* <RegisterNode/> */}
              {/* <UpdateNode/> */}
              <NodeStatusDetails />
              {/* </div> */}
            </div>
          </AppProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
