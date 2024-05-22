// WalletContext.js

import React, { createContext, useContext, useState, useEffect } from "react";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { PhantomWalletAdapter, SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";

const WalletContext = createContext();

export const useWallet = () => useContext(WalletContext);

export const WalletProviderWrapper = ({ children }) => {
  const [wallet, setWallet] = useState(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    // Check local storage for saved wallet connection
    const savedWallet = localStorage.getItem("wallet");
    if (savedWallet) {
      setWallet(savedWallet);
      setConnected(true);
    }
  }, []);

  const connectWallet = async () => {
    // Connect the wallet (example using Phantom wallet adapter)
    const wallet = new PhantomWalletAdapter();
    await wallet.connect();
    setWallet(wallet);
    setConnected(true);
    localStorage.setItem("wallet", wallet);
  };

  const disconnectWallet = () => {
    // Disconnect the wallet
    setWallet(null);
    setConnected(false);
    localStorage.removeItem("wallet");
  };

  return (
    <WalletContext.Provider value={{ wallet, connected, connectWallet, disconnectWallet }}>
      {children}
    </WalletContext.Provider>
  );
};

export default WalletContext;
