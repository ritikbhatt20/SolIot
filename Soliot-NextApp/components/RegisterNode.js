import React, { useState } from "react";
import { useAppContext } from "../context/context";
import "bootstrap/dist/css/bootstrap.min.css";

const RegisterNode = () => {
  const { registerNode } = useAppContext();
  const [ip, setIp] = useState("");

  const handleRegisterNode = () => {
    const ipArray = ip.split('.').map(Number);
    registerNode(ipArray);
  };

  return (
    <div className="d-flex flex-column align-items-center mt-3">
      <div className="mb-3">
        <input
          type="text"
          className="form-control"
          placeholder="Enter IP (e.g. 192.168.1.1)"
          value={ip}
          onChange={(e) => setIp(e.target.value)}
        />
      </div>
      <button className="btn btn-primary" onClick={handleRegisterNode}>
        Register Node
      </button>
    </div>
  );
};

export default RegisterNode;
