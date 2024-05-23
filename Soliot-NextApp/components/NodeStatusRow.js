import React from "react";
import "bootstrap/dist/css/bootstrap.min.css";
import { shortenPk } from "../utils/helper";

const NodeStatusRow = ({ node }) => {
  const { nodePubkey, ip, uptime, heartbeat, bytes, lastUpdateSlot, tokenEarnings } = node;

  return (
    <tr>
      <td>{shortenPk(nodePubkey)}</td>
      <td>{ip.join('.')}</td>
      <td>{uptime}</td>
      <td>{heartbeat}</td>
      <td>{bytes}</td>
      <td>{lastUpdateSlot}</td>
      <td>{tokenEarnings}</td>
    </tr>
  );
};

export default NodeStatusRow;
