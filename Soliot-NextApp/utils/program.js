import { AnchorProvider, BN, Program } from "@project-serum/anchor";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react'

import IDL from './idl.json'

import { REGISTRY_SEED, NODE_SEED, PROGRAM_ID } from "./constants";

export const getProgram = (connection, wallet) => {
    const provider = new AnchorProvider(connection, wallet, {
      commitment: "confirmed",                                    
    });
    const program = new Program(IDL, PROGRAM_ID, provider);     
    return program;                                         
};  

export const getRegistryAddress = async () => {
    return (
      await PublicKey.findProgramAddressSync([Buffer.from(REGISTRY_SEED)], PROGRAM_ID)
    )[0];                                                       
};

export const getNodeStatusAddress = async (publicKey) => {
    return (
      await PublicKey.findProgramAddressSync([Buffer.from(NODE_SEED), publicKey.toBuffer()], PROGRAM_ID)
    )[0];                                                       
};

