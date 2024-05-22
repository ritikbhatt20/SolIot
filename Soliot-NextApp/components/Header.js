"use client";

import { useEffect, useState } from "react";
import Link from 'next/link';
import dynamic from "next/dynamic";
import style from "../styles/Header.module.css";

const DynamicWalletMultiButton = dynamic(
  () => import("@solana/wallet-adapter-react-ui").then((mod) => mod.WalletMultiButton),
  { ssr: false }
);

const Header = () => {
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  return (
    <div className={style.wrapper}>
      <div className={style.title}>Solana Iot Node Dashboard</div>
      {isClient && <DynamicWalletMultiButton />}
    </div>
  );
};

export default Header;
