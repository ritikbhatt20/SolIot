# IoT Nodes Management on Solana

This project enables the management of IoT nodes using the Solana blockchain. It consists of a Solana smart contract for handling registry and node operations, a Rust client for interacting with the smart contract, and a Next.js application for a user-friendly interface.

## Table of Contents

- [Introduction](#introduction)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Smart Contract](#smart-contract)
  - [Rust Client](#rust-client)
  - [Next.js Application](#nextjs-application)
- [Usage](#usage)
  - [Initialize Registry](#initialize-registry)
  - [Register Node](#register-node)
  - [Update Node](#update-node)
  - [Unregister Node](#unregister-node)
- [Project Structure](#project-structure)
- [License](#license)

## Introduction

This project allows users to manage IoT nodes using the Solana blockchain. It involves deploying a smart contract to handle node registration, updates, and unregistration, a Rust client to interact with the smart contract, and a Next.js application for front-end interaction. The smart contract ensures decentralized and secure management of IoT nodes, providing a transparent and tamper-proof system for tracking node status and interactions.

## Architecture

1. **Solana Smart Contract (Program)**: Written in Rust, deployed on the Solana blockchain, the smart contract manages the registry of IoT nodes, allowing for operations such as initialization, registration, updating, and unregistration.
2. **Rust Client**: A command-line interface (CLI) tool written in Rust that interacts with the deployed smart contract. It provides functionality to initialize the registry, register nodes, update node status, and unregister nodes.
3. **Next.js Application**: A web application that offers a user-friendly interface for managing IoT nodes. It allows users to perform actions such as initializing the registry, registering nodes, updating node status, and unregistering nodes through a graphical interface.

## Prerequisites

- **Node.js (v14 or higher)**
- **Rust and Cargo**: The Rust programming language and its package manager, Cargo.
- **Solana CLI**: Command-line interface tools for interacting with the Solana blockchain.
- **Anchor CLI**: A framework for Solana smart contract development.
- **Yarn or npm**: Package managers for JavaScript.
- **Solana-compatible wallet (e.g., Phantom Wallet)**: Such as Phantom Wallet, used for interacting with the Solana blockchain and managing keys.

## Installation

### Smart Contract

1. **Install Solana CLI and Anchor CLI**:
   ```sh
   sh -c "$(curl -sSfL https://release.solana.com/v1.18.14/install)"
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   anchor --version

2. **Build and Deploy the Smart Contract**:
    ```sh
    cd solana-program
    anchor build
    anchor deploy
3. **Obtain Program ID**:
    After deploying the smart contract, the deployment step will output the Program ID. Copy this ID to use in your Rust client and Next.js application configurations.

### Rust Client

1. **Set Up the Rust Client**:

    ```sh
    cd rust-client
    cargo build

2. **Configure the Rust Client**:
    Update the .env file with the path to your wallet keypair, the mint authority, and the Program ID obtained from the smart contract deployment.
   
4. **Run the Rust Client code**:
   ```sh
   cargo run

### Next.js Application

1. **Install Dependencies**:

    ```sh
    cd nextjs-app
    yarn install
    npm install

2. **Configure the Application**:
   Update the constants.js file with your Program ID and Token Mint Address.

3. **Run the Application**:

    ```sh
    yarn dev
    or
    npm run dev
   
### Usage

## Initialize Registry

1. **Initialize the Registry**:
   Use the Next.js application to initialize the registry by clicking on the "Initialize Registry" button. This action creates a registry account on the Solana blockchain, which will store information         about the registered IoT nodes.

   ```sh
   cargo run -- initialize

## Register Node

2. **Register a Node**:
   Use the Next.js application to register a new node by providing the necessary details, such as the node's IP address. This action creates a new node account and adds it to the registry.

    ```sh
    cargo run -- register <ip>
   
## Update Node

3. **Update a Node**:
   Use the Next.js application to update the node details such as uptime, heartbeat, and bytes. This action records the current status of the node on the blockchain, ensuring a reliable and tamper-proof       log of node activity.

   ```sh
   cargo run -- update <uptime> <heartbeats> <bytes>

## Unregister Node

4. **Unregister a node**:
   Use the Next.js application to unregister a node by providing the node's details. This action removes the node from the registry and closes the node's account.

   ```sh
   cargo run -- unregister
   
### Project Structure

```sh
.
├── README.md
├── nextjs-app
│   ├── public
│   ├── src
│   │   ├── pages
│   │   ├── components
│   │   ├── utils
│   │   └── styles
│   ├── constants.js
│   ├── next.config.js
│   └── ...
├── rust-client
│   ├── src
│   ├── config.json
│   ├── Cargo.toml
│   └── ...
└── solana-program
    ├── src
    │   ├── lib.rs
    │   ├── processor.rs
    │   └── state.rs
    ├── migrations
    ├── tests
    ├── Anchor.toml
    └── ...
