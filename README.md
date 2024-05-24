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
