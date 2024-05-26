"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import dynamic from "next/dynamic";
// import style from "../styles/Header.module.css";

const DynamicWalletMultiButton = dynamic(
  () =>
    import("@solana/wallet-adapter-react-ui").then(
      (mod) => mod.WalletMultiButton
    ),
  { ssr: false }
);

const Header = () => {
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  return (
    <div className="flex justify-between items-center px-10 py-4 border-b-[1px] border-slate-900 text-gray-200 bg-slate-900">
      <div className="text-2xl">Solana Iot Node Dashboard</div>
      {isClient && <DynamicWalletMultiButton />}
    </div>
  );
};

export default Header;
