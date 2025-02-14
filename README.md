# Mantra Escrow

Mantra Escrow is a CosmWasm-based escrow smart contract designed to facilitate secure escrow transactions on Cosmos SDK blockchains. This contract supports the creation of escrow agreements between a seller and a buyer by validating deposits, applying fees, and enforcing timeouts.

## Features

- **Configurable Parameters:**  
  Configure admin address, escrow fee (in basis points), minimum and maximum escrow durations, and allowed denominations during instantiation.
  
- **Escrow Lifecycle:**  
  Create a new escrow agreement and deposit funds according to the escrow conditions. The contract enforces correct sender, exact fund amounts, valid denominations, and timeout checks.

- **Comprehensive Testing:**  
  Integration and unit tests are available using the CosmWasm multi-test framework to simulate various escrow scenarios.

- **Query Functions:**  
  Fetch contract configuration, all escrows, individual escrow details, escrow count, and escrow lists by seller or buyer.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (with Cargo)
- Docker (optional, for WASM optimization)
- Familiarity with CosmWasm smart contracts and the Cosmos SDK

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/mantra-escrow.git
   cd mantra-escrow
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Build the contract for WebAssembly:

   ```bash
   cargo wasm
   ```

   *(Note: The command `cargo wasm` is set up as an alias in the `.cargo/config.toml` file.)*
### Running Tests
  Run `cargo test` on the root folder.