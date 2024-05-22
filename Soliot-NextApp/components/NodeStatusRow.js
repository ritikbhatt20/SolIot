import React from "react";
import "bootstrap/dist/css/bootstrap.min.css";
import { shortenPk } from "../utils/helper";

const NodeStatusRow = ({ node }) => {
  const { nodePubkey, ip, uptime, heartbeat, bytes } = node;

  return (
    <tr>
      <td>{shortenPk(nodePubkey)}</td>
      <td>{ip.join('.')}</td>
      <td>{uptime}</td>
      <td>{heartbeat}</td>
      <td>{bytes}</td>
    </tr>
  );
};

export default NodeStatusRow;
