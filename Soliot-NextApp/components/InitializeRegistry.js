import React from "react";
import { useAppContext } from "../context/context";
import "bootstrap/dist/css/bootstrap.min.css";

const InitializeRegistry = () => {
  const { initializeRegistry } = useAppContext();

  return (
    <div className="d-flex justify-content-center mt-3">
      <button className="btn btn-primary" onClick={initializeRegistry}>
        Initialize Registry
      </button>
    </div>
  );
};

export default InitializeRegistry;
