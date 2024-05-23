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
import style from "../styles/Home.module.css";
import "@solana/wallet-adapter-react-ui/styles.css";
import { AppProvider } from "../context/context";
import InitializeRegistry from "../components/InitializeRegistry";
import RegisterNode from "../components/RegisterNode";
import UpdateNode from "../components/UpdateNode";
import NodeStatusDetails from "../components/NodeStatusDetails";

export default function Home() {
  const endpoint =
    "https://fluent-little-sun.solana-devnet.quiknode.pro/371c43da2c0c0ea39a2d68d940b83d1c7a0a05b5/";

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
            <Header />
            <div className={style.container}>
              {/* <InitializeRegistry />  */}
              {/* <RegisterNode/> */}
              {/* <UpdateNode/> */}
              <NodeStatusDetails/>
            </div>
          </AppProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
