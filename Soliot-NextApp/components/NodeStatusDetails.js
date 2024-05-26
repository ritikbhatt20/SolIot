import React, { useEffect } from "react";
import { useAppContext } from "../context/context";
import NodeStatusRow from "./NodeStatusRow";
import { shortenPk } from "../utils/helper";

const NodeStatusDetails = () => {
  const { nodeStatus, fetchNodeStatus } = useAppContext();
  // const nodeStatus = [
  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },

  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },
  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },
  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },
  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },
  //   {
  //     publicKey: "8KenPYeYoA3cX7n1yzA7V8DxDfd9AVBguBnGmpegMTjv",
  //     nodePubkey: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     authority: "4xFxSsA9vnMHS4WutAJvEwv8CFaaiLrmNHd58h7n2X3V",
  //     ip: [192, 168, 1, 1],
  //     uptime: "98",
  //     heartbeat: "67",
  //     bytes: "1024",
  //     lastUpdateSlot: "301338611",
  //     tokenEarnings: "87",
  //   },
  // ];

  useEffect(() => {
    fetchNodeStatus();

    const intervalId = setInterval(() => {
      fetchNodeStatus();
    }, 5000000);

    return () => clearInterval(intervalId);
  }, []);

  // useEffect(() => {
  //   console.log(nodeStatus);
  // }, [nodeStatus]);

  return (
    <>
      <div className="w-full h-full pt-5">
        <p className="text-xl font-semibold text-slate-300  justify-start px-8">
          Node Status Details
        </p>
      </div>
      <div className="w-full overflow-x-auto h-full">
        {nodeStatus.map((node, index) => (
          <div
            key={index}
            className="w-max xl:w-full flex flex-col justify-evenly items-center"
          >
            {/* <div className="w- h-full flex flex-col justify-center items-center py-3"> */}
            {/* <div className="flex flex-wrap gap-3 w-full justify-center items-center text-gray-200"> */}
            <div className="w-full h-full grid grid-cols-7 px-8 gap-8  py-3  text-slate-200">
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Node Public Key</p>
                  <p className="break-words">{shortenPk(node.nodePubkey)}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>IP Address</p>
                  <p className="break-words">{node.ip.join(".")}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Uptime</p>
                  <p className="break-words">{node.uptime}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Heartbeat</p>
                  <p className="break-words">{node.heartbeat}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Bytes</p>
                  <p className="break-words">{node.bytes}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Last Update Slot</p>
                  <p className="break-words">{node.lastUpdateSlot}</p>
                </div>
              </div>{" "}
              <div className="w-full min-h-[50px] bg-slate-900 rounded-xl">
                <div className="py-3 px-4 flex flex-col">
                  <p>Token Earnings</p>
                  <p className="break-words">{node.tokenEarnings}</p>
                </div>
              </div>
            </div>
              <div className="w-10/12 flex justify-center items-center border-t-[1px] border-slate-700"> 
               </div>
          </div>
        ))}
      </div>
    </>
  );
};

export default NodeStatusDetails;
