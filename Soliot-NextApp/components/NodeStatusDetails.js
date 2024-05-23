import React, { useEffect } from "react";
import { useAppContext } from "../context/context";
import NodeStatusRow from "./NodeStatusRow";
import "bootstrap/dist/css/bootstrap.min.css";

const NodeStatusDetails = () => {
  const { nodeStatus, fetchNodeStatus } = useAppContext();

  useEffect(() => {
    fetchNodeStatus();

    const intervalId = setInterval(() => {
      fetchNodeStatus();
    }, 5000);

    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="container mt-5">
      <table className="table table-striped table-bordered">
        <thead>
          <tr>
            <th>Node Public Key</th>
            <th>IP Address</th>
            <th>Uptime</th>
            <th>Heartbeat</th>
            <th>Bytes</th>
            <th>Last Update Slot</th>
            <th>Token Earnings</th>
          </tr>
        </thead>
        <tbody>
          {nodeStatus.map((node, index) => (
            <NodeStatusRow key={index} node={node} />
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default NodeStatusDetails;
