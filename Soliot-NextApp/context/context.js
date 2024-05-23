import { createContext, useContext, useMemo, useEffect, useState } from "react";
import { BN } from "@project-serum/anchor";
import { SystemProgram, PublicKey } from "@solana/web3.js";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import toast from "react-hot-toast";

import {
  getProgram,
} from "../utils/program";
import { mockWallet } from "../utils/helper";

export const AppContext = createContext();

export const AppProvider = ({ children }) => {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const [registry, setRegistry] = useState(null);
  const [nodes, setNodes] = useState([]);
  const [nodeStatus, setNodeStatus] = useState([]);

  const program = useMemo(() => {
    if (connection) {
      return getProgram(connection, wallet ?? mockWallet());
    }
  }, [connection, wallet]);

  useEffect(() => {
    if (!program) return;
    fetchRegistry();
    fetchNodeStatus();
  }, [program]);

  const fetchRegistry = async () => {
    try {
      const registry = await program.account.registry.all();
      if (registry.length > 0 && registry[0].account.nodes.length > 0) {
        const nodes = registry[0].account.nodes;
        console.log("Nodes:", nodes);
        setNodes(nodes);
      } else {
        console.log("Registry is empty or has no nodes.");
      }
      setRegistry(registry);
    } catch (error) {
      console.error("Error fetching registry:", error);
    }
  };

  const fetchNodeStatus = async () => {
    if (!program) return;

    try {
      const nodeStatusList = await program.account.nodeStatus.all();
      const formattedNodeStatusList = nodeStatusList.map((nodeStatusAccount) => ({
        publicKey: nodeStatusAccount.publicKey.toString(),
        nodePubkey: nodeStatusAccount.account.nodePubkey.toString(),
        authority: nodeStatusAccount.account.authority.toString(),
        ip: nodeStatusAccount.account.ip,
        uptime: nodeStatusAccount.account.uptime.toString(),
        heartbeat: nodeStatusAccount.account.heartbeat.toString(),
        bytes: nodeStatusAccount.account.bytes.toString(),
        lastUpdateSlot: nodeStatusAccount.account.lastUpdateSlot.toString(),
        tokenEarnings: nodeStatusAccount.account.tokenEarnings.toString(),
      }));
      setNodeStatus(formattedNodeStatusList);
    } catch (error) {
      console.error("Error fetching node status:", error);
    }
  };

  const initializeRegistry = async () => {
    if (!program || !wallet) return;
    try {
      const [registryPda] = await PublicKey.findProgramAddressSync(
        [Buffer.from("registry")],
        program.programId
      );
      const tx = await program.methods
        .initializeRegistry()
        .accounts({
          registry: registryPda,
          user: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      await connection.confirmTransaction(tx);
      toast.success("Registry initialized!");
      fetchRegistry();
    } catch (error) {
      console.error("Error initializing registry:", error);
      toast.error("Failed to initialize registry");
    }
  };

  const registerNode = async (ip) => {
    if (!program || !wallet) return;
    try {
      const [nodePda] = await PublicKey.findProgramAddressSync(
        [Buffer.from("node"), wallet.publicKey.toBuffer()],
        program.programId
      );
      const [registryPda] = await PublicKey.findProgramAddress(
        [Buffer.from("registry")],
        program.programId
      );
      const tx = await program.methods
        .register(ip)
        .accounts({
          node: nodePda,
          authority: wallet.publicKey,
          registry: registryPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      await connection.confirmTransaction(tx);
      toast.success("Node registered!");
      fetchNodeStatus();
    } catch (error) {
      console.error("Error registering node:", error);
      toast.error("Failed to register node");
    }
  };

  const updateNode = async (uptime, heartbeat, bytes) => {
    if (!program || !wallet) return;
    try {
      const [nodePda] = await PublicKey.findProgramAddress(
        [Buffer.from("node"), wallet.publicKey.toBuffer()],
        program.programId
      );
      const uptimeBN = new BN(uptime);
      const heartbeatBN = new BN(heartbeat);
      const bytesBN = new BN(bytes);

      const tx = await program.methods
        .update(uptimeBN, heartbeatBN, bytesBN)
        .accounts({
          node: nodePda,
          authority: wallet.publicKey,
        })
        .rpc();

      await connection.confirmTransaction(tx);
      toast.success("Node updated!");
      fetchNodeStatus();
    } catch (error) {
      console.error("Error updating node:", error);
      toast.error("Failed to update node");
    }
  };

  return (
    <AppContext.Provider
      value={{ registry, nodes, nodeStatus, initializeRegistry, registerNode, updateNode, fetchNodeStatus }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => useContext(AppContext);
