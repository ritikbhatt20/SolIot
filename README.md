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
- [Project Structure](#project-structure)
- [License](#license)

## Introduction

This project allows users to manage IoT nodes using the Solana blockchain. It involves deploying a smart contract to handle node registration and updates, a Rust client to interact with the smart contract, and a Next.js application for front-end interaction.

## Architecture

1. **Solana Smart Contract (Program)**: Written in Rust, deployed on the Solana blockchain.
2. **Rust Client**: Interacts with the deployed smart contract.
3. **Next.js Application**: User interface for managing the IoT nodes.

## Prerequisites

- Node.js (v14 or higher)
- Rust and Cargo
- Solana CLI
- Anchor CLI
- Yarn or npm
- Solana-compatible wallet (e.g., Phantom Wallet)

## Installation

### Smart Contract

1. **Install Solana CLI and Anchor CLI**:
   ```sh
   sh -c "$(curl -sSfL https://release.solana.com/v1.8.0/install)"
   cargo install --git https://github.com/project-serum/anchor --tag v0.18.0 anchor-cli --locked

2. **Build and Deploy the Smart Contract**:
    ```sh
    cd solana-program
    anchor build
    anchor deploy
3. **Obtain Program ID**:
    The deployment step will output the Program ID. Copy this ID to use in your client and front-end code.

### Rust Client

1. **Set Up the Rust Client**:

    ```sh
    cd rust-client
    cargo build

2. **Configure the Rust Client**:
    Update the config.json with your wallet path, cluster URL, and Program ID.

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
   Use the Next.js application to initialize the registry by clicking on the "Initialize Registry" button.

## Register Node

2. **Register a Node**:
   Use the Next.js application to register a new node by providing the necessary details such as IP address.
   
## Update Node

3. **Update a Node**:
   Use the Next.js application to update node details such as uptime, heartbeat, and bytes.
   
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
