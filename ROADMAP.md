# Unwalled Node - Project Roadmap & Handoff

## 1. Project Overview

This repository contains the foundational source code for the **Unwalled L1 Ad Exchange Node**. The goal of this project is to build a purpose-built Layer-1 blockchain for a decentralized, high-frequency advertising exchange, as detailed in the Unwalled whitepaper.

The architecture is based on several key principles:

- **High-Performance Core:** The node is written in Rust for performance and safety.
- **Modular Design:** The project is structured as a Cargo workspace to separate concerns (node logic, client libraries, examples).
- **Modern Networking:** It utilizes `libp2p` for peer-to-peer communication and `quiche` for a modern HTTP/3 RPC interface.
- **BFT Consensus:** It is designed to use `HotStuff-rs` for Byzantine Fault Tolerant consensus, ensuring fast and final transaction ordering.
- **Hybrid Settlement:** The node is designed to operate as a high-throughput L1 for ad exchange operations while relying on an external settlement layer (like Keeta) for identity and high-value fund management.

## 2. Current State & Architecture

The project is currently a **well-structured scaffold** with significant architectural progress. The code has evolved beyond initial scaffolding with concrete implementations for key components, but is **not yet in a fully compiling or runnable state**. The architecture demonstrates clear separation of concerns and includes working implementations for:

- ✅ **Core data structures** with proper serialization and cryptographic verification
- ✅ **Database layer** with RocksDB column families and state management
- ✅ **Network architecture** using libp2p with gossipsub and mDNS
- ✅ **Consensus integration** with HotStuff-rs App trait implementation
- ⚠️ **Partial RPC layer** with quiche HTTP/3 setup (needs certificate handling fixes)
- ⚠️ **Incomplete consensus engine** (missing Consensus struct and replica initialization)
- 📝 **Placeholder settlement layer** for L1 integration

### 2.1. Workspace Structure

The project is organized as a Cargo workspace with the following key parts:

- **`/node`**: The main blockchain node crate.
- **`/client`**: A client library for interacting with the node's RPC API.
- **`/examples`**: CLI tools (`place_bid.rs`, `trigger_auction.rs`) that demonstrate how to use the client library.
- **`/docs`**: Contains architecture-as-code using PlantUML (`architecture.puml`).
- **`/test-rocksdb`**: A separate test crate for RocksDB functionality.

### 2.2. Node Components (`/node/src`)

The node's internal architecture is broken down into the following modules:

- **`primitives.rs`**: Defines the core data structures, including `Bid`, `AuctionTrigger`, `Match`, and the `Signed<T>` wrapper which ensures all transactions are signed and include a fee. Includes signature verification logic using Ed25519.
- **`identity.rs`**: Provides a flexible abstraction for cryptographic identity, including a `Signer` trait and a `LocalWallet` implementation using Ed25519. Defines `PublicKey`, `Signature`, and `Address` types.
- **`config.rs`**: **NEW MODULE** - Handles node configuration with `Config` struct containing RPC address and database path. Currently uses default values.
- **`state.rs`**: Manages all state transitions and interaction with the RocksDB database. Uses Column Families (`accounts`, `bids`) with implemented methods for balance management, fee application, and bid storage. The `find_match` function is a placeholder.
- **`consensus.rs`**: Contains the bridge between the HotStuff consensus engine and application logic via the `ConsensusApp` struct. Implements the `App` trait with a `deliver` method for processing `PlaceBid` and `TriggerAuction` transactions.
- **`network.rs`**: Sets up the `libp2p` swarm with gossipsub for transaction propagation and mDNS for peer discovery. Includes `NetworkManager` and `MyBehaviour` structs.
- **`rpc.rs`**: HTTP/3 server skeleton using `quiche` with self-signed certificate generation. Contains placeholder logic for QUIC connection handling and request processing.
- **`settlement.rs`**: Placeholder module for L1 settlement layer interactions with `SettlementManager` struct and `onboard_funds`/`offboard_funds` methods.
- **`main.rs`**: Main entry point with component initialization, async message passing via channels, and a `tokio::select!` event loop coordinating RPC, network, and consensus components.

### 2.3. Key Architectural Decisions

**Transaction Model:**
- All transactions are wrapped in `Signed<T>` with Ed25519 signatures, nonces, and fees
- Fee verification prevents spam and funds validators
- Signature verification covers data + nonce + fee to prevent tampering

**Client Library Design:**
- Clean separation between node implementation and client API  
- Uses HTTP/1.1 with reqwest as placeholder for eventual HTTP/3 integration
- Re-exports primitives from node crate for type consistency

**State Management:**
- RocksDB with column families for scalable data separation (`accounts`, `bids`)
- Balance tracking with atomic fee deduction
- Placeholder auction matching logic ready for implementation

**Network Architecture:**
- libp2p for P2P networking with modern protocols (noise, yamux)
- Gossipsub for transaction propagation across the network
- mDNS for local peer discovery in development

**Consensus Integration:**  
- Clean separation via HotStuff-rs `App` trait implementation
- Transaction enum supports extensible operation types
- Event-driven architecture with async message passing

## 3. Next Steps for Development

This roadmap outlines the path from the current scaffold to a functional MVP.

### Phase 1: Environment Setup & First Compile

The immediate next step is to get the project compiling in a stable environment.

1.  **Set up a clean Ubuntu 24 environment** (as per our discussion, this is the most reliable path).
2.  **Install Prerequisites:** Ensure the following are installed:
    - The Rust toolchain (`rustup`)
    - A C++ toolchain (`build-essential`, `clang`, `cmake`)
    - The RocksDB shared library (`librocksdb-dev`, `libgflags-dev`)
3.  **Achieve First Build:** Run `cargo build` from the `unwalled-node` directory (not root). Current known compilation issues:
    - **Missing identity module import** in `primitives.rs:1` 
    - **Incomplete RPC certificate loading** in `rpc.rs:27` - `load_cert_chain_from_pem_file` method doesn't exist
    - **Incomplete consensus engine** in `consensus.rs` - missing actual HotStuff initialization 
    - **Missing consensus struct** in `main.rs:27` - `consensus::Consensus` is not implemented

### Phase 2: Implement Core RPC & State Logic

Once the project builds, the focus should be on making the node functional as a standalone service.

1.  **Complete the RPC Server (`rpc.rs`):**
    - Fix certificate loading - replace `load_cert_chain_from_pem_file` with proper quiche certificate handling
    - Implement complete QUIC connection management and HTTP/3 request parsing
    - Deserialize incoming transaction data (`Signed<Bid>`, `Signed<AuctionTrigger>`) from request bodies
    - Send deserialized transactions into the `tx_to_consensus` channel
2.  **Flesh out the State Manager (`state.rs`):**
    - Implement the `find_match` function with bid iteration logic over the `bids` column family
    - Complete the TODO for converting `PublicKey` to `Address` in consensus transaction processing
    - Implement validator reward pool for fee collection
    - Write robust unit tests for all `StateManager` functions, especially `apply_fees` and bid/match logic

### Phase 3: Wire Up Consensus & Networking

This phase turns the standalone service into a distributed system.

1.  **Initialize HotStuff (`consensus.rs`):**
    - Implement the missing `consensus::Consensus` struct referenced in `main.rs:27`
    - Create the `hotstuff_rs::Replica` instance with proper configuration
    - Implement the network backend for HotStuff, bridging `libp2p` for consensus message passing
2.  **Connect the Event Loop (`main.rs`):**
    - Complete the transaction processing flow in the `tokio::select!` loop
    - Pass transactions from `rx_from_components` to the HotStuff replica for ordering
    - Wire up the `ConsensusApp::deliver` callback when blocks are committed
3.  **Complete Network Integration (`network.rs`):**
    - Implement the missing `event_loop` function referenced in `main.rs:43`
    - Add transaction broadcasting via gossipsub when RPC receives new transactions
    - Implement deduplication logic to avoid processing the same transaction multiple times
    - Complete the NetworkManager initialization and peer connection handling

### Phase 4: Economics & Settlement

1.  **Validator Rewards (`state.rs`):**
    - Implement the logic for a validator reward pool. When `apply_fees` is called, the fee should be added to a pool.
    - At the end of each block, the consensus leader could be rewarded from this pool.
2.  **Settlement Layer (`settlement.rs`):**
    - Begin implementing the `onboard_funds` and `offboard_funds` functions by connecting to a Keeta testnet client.
    - This will likely involve creating a Keeta-specific `Signer` in the `identity.rs` module.

## 4. Current Build Status & Dependencies

**Dependency Analysis:**
The project has a well-defined dependency stack in Cargo.toml files:
- **Core:** tokio, serde, uuid, anyhow for async runtime and data handling
- **Crypto:** ed25519-dalek, ring for signature verification
- **Storage:** rocksdb for persistent state  
- **Networking:** libp2p with comprehensive feature set, quiche for HTTP/3
- **Consensus:** hotstuff_rs for BFT consensus
- **Development:** rcgen for certificate generation, log/env_logger for diagnostics

**Critical Path to First Compile:**
1. Fix `primitives.rs:1` - add missing identity module import
2. Fix `rpc.rs:27` - replace invalid quiche certificate loading method
3. Implement `consensus::Consensus` struct referenced in `main.rs:27`
4. Implement `network::event_loop` function referenced in `main.rs:43`
5. Address any workspace member dependency resolution issues

**Architecture Maturity:**
The codebase demonstrates sophisticated understanding of Rust async patterns, proper error handling with `anyhow::Result`, and modern blockchain architecture principles. The scaffold is significantly more advanced than typical initial implementations and shows clear path to production readiness.

This roadmap provides a clear path forward. The immediate priority is achieving a successful build in a clean environment, after which the well-architected core logic can be completed feature by feature.
