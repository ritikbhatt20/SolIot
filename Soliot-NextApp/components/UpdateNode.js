import React, { useState } from "react";
import { useAppContext } from "../context/context";
import "bootstrap/dist/css/bootstrap.min.css";

const UpdateNode = ({ node }) => {
  const { updateNode } = useAppContext();
  const [uptime, setUptime] = useState();
  const [heartbeat, setHeartbeat] = useState();
  const [bytes, setBytes] = useState();

  const handleUpdateNode = () => {
    updateNode(uptime, heartbeat, bytes);
  };

  return (
    <div className="d-flex flex-column align-items-center mt-3">
      <div className="mb-3">
        <input
          type="number"
          className="form-control"
          placeholder="Uptime"
          value={uptime}
          onChange={(e) => setUptime(Number(e.target.value))}
        />
      </div>
      <div className="mb-3">
        <input
          type="number"
          className="form-control"
          placeholder="Heartbeat"
          value={heartbeat}
          onChange={(e) => setHeartbeat(Number(e.target.value))}
        />
      </div>
      <div className="mb-3">
        <input
          type="number"
          className="form-control"
          placeholder="Bytes"
          value={bytes}
          onChange={(e) => setBytes(Number(e.target.value))}
        />
      </div>
      <button className="btn btn-primary" onClick={handleUpdateNode}>
        Update Node
      </button>
    </div>
  );
};

export default UpdateNode;
