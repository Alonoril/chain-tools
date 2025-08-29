# Chain Tools

🔗 A comprehensive Rust toolkit for interacting with blockchain networks, focused on efficient integration with the Endless blockchain platform.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-brightgreen.svg)](https://www.rust-lang.org)

## 🚀 Project Overview

Chain Tools is a high-performance Rust workspace project that provides a complete client solution for the Endless blockchain. The project adopts a modular design, offering a full suite of features from account management to transaction processing.

## 📦 Project Structure

```
chain-tools/
├── Cargo.toml              # Workspace configuration
├── README.md              # Project documentation
└── endless-client/        # Endless blockchain client library
    ├── src/
    │   ├── client/        # Enhanced client implementation
    │   ├── sdk_ext/       # SDK extensions
    │   ├── utils/         # Utility functions
    │   └── error.rs       # Error definitions
    └── Cargo.toml
```

## ✨ Core Features

### 🔑 Account Management
- **Private Key Recovery**: Support for Ed25519 key local recovery
- **Account Creation**: Local account creation and management
- **Address Generation**: Automatic generation of corresponding account addresses

### 🌐 REST Client
- **Chain Information Retrieval**: Get chain ID, version, and other basic information
- **Smart Caching**: Built-in Chain ID caching mechanism
- **Transaction Submission**: Support for Entry Function calls
- **View Functions**: Read-only function call support

### 💰 Token Operations
- **EDS Transfers**: Native token transfer functionality
- **Token Transfers**: Support for Fungible Asset standard token transfers
- **Balance Queries**: Query EDS and other token balances
- **Transaction Simulation**: Simulate transactions before actual execution

### ⚙️ Advanced Features
- **Gas Configuration**: Custom gas limits and pricing
- **Timeout Management**: Flexible transaction timeout settings
- **Error Handling**: Comprehensive error classification and handling mechanisms
- **Async Support**: Built on Tokio async runtime

## 🛠️ Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
endless-client = { git = "https://github.com/Alonoril/chain-tools", branch = "main" }
tokio = { version = "1", features = ["rt", "rt-multi-thread", "macros"] }
```

### Basic Usage Examples

#### 1. Creating a Client

```rust
use endless_client::client::EnhancedClient;
use endless_client::sdk_ext::account::LocalAccountExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = EnhancedClient::new_with_url_str("https://endless-rpc-url.com")?;
    
    // Get chain information
    let index = client.get_index().await?;
    println!("Chain ID: {}", index.chain_id);
    
    Ok(())
}
```

#### 2. Account Recovery and Management

```rust
use endless_client::sdk_ext::account::LocalAccountExt;

// Recover account from private key
let private_key = "0x1234..."; // Your private key
let account = private_key.recover_account()?;

println!("Account Address: {}", account.address());
```

#### 3. EDS Token Transfer

```rust
use endless_sdk::move_types::account_address::AccountAddress;
use std::str::FromStr;

// Transfer EDS
let to_address = AccountAddress::from_str("0xrecipient_address")?;
let amount = 1000000; // 1 EDS (assuming 6 decimals)

let result = client
    .transfer(&account, to_address, amount, None)
    .await?;

println!("Transaction Hash: {}", result.inner().hash);

// Wait for transaction confirmation
let confirmed_tx = client.wait_for_txn(result.inner()).await?;
println!("Transaction confirmed: {}", confirmed_tx.inner().hash);
```

#### 4. Token Balance Queries

```rust
// Query EDS balance
let eds_balance = client.balance_of(&account.address()).await?;
println!("EDS Balance: {}", eds_balance.inner());

// Query other token balances
let token_address = AccountAddress::from_str("0xtoken_address")?;
let token_balance = client
    .get_token_balance(account.address(), token_address)
    .await?;
println!("Token Balance: {}", token_balance.inner());
```

#### 5. Token Transfers

```rust
// Transfer tokens
let token_address = AccountAddress::from_str("0xtoken_address")?;
let amount = 500000;

let result = client
    .transfer_token(&account, to_address, amount, token_address, None)
    .await?;

println!("Token Transfer Hash: {}", result.inner().hash);
```

#### 6. Transaction Simulation

```rust
// Simulate transfer transaction
let simulation = client
    .simulate_transfer(&account, to_address, amount, None)
    .await?;

println!("Gas Used: {:?}", simulation.inner()[0].gas_used);
println!("Success: {}", simulation.inner()[0].success);
```

### Advanced Configuration

#### Custom Gas Fees

```rust
use endless_sdk::helper_client::Overrides;

let overrides = Some(Overrides {
    max_gas_amount: 200000,
    gas_unit_price: 100,
    expiration_timeout_secs: 60,
    ..Overrides::default()
});

let result = client
    .transfer(&account, to_address, amount, overrides)
    .await?;
```

## 📚 API Documentation

### EnhancedClient

The main client class providing all functionality for interacting with the Endless blockchain.

#### Constructors

- `new(node_url: Url) -> Self`
- `new_with_url_str(node_url: &str) -> AppResult<Self>`

#### Chain Information Methods

- `get_index() -> AppResult<IndexData>` - Get basic chain information

#### Transfer Methods

- `transfer(from, to, amount, overrides) -> AppResult<Response<PendingTransaction>>`
- `transfer_token(from, to, amount, token, overrides) -> AppResult<Response<PendingTransaction>>`
- `simulate_transfer(from, to, amount, overrides) -> AppResult<Response<Vec<UserTransaction>>>`
- `simulate_transfer_token(from, to, amount, token, overrides) -> AppResult<Response<Vec<UserTransaction>>>`

#### Balance Query Methods

- `balance_of(owner) -> AppResult<Response<u128>>` - Query EDS balance
- `get_token_balance(owner, token) -> AppResult<Response<u128>>` - Query token balance

#### Utility Methods

- `wait_for_txn(pending_tx) -> AppResult<Response<Transaction>>` - Wait for transaction confirmation
- `rest_client() -> RestClient` - Get underlying REST client

### LocalAccountExt Trait

Provides account recovery functionality for private keys and strings.

- `recover_account(self) -> AppResult<LocalAccount>`

## 🔧 Building and Testing

### Building the Project

```bash
# Build all crates
cargo build

# Build release version
cargo build --release

# Build specific crate
cargo build -p endless-client
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p endless-client

# Run tests with output
cargo test -- --nocapture
```

### Code Quality Checks

```bash
# Format code
cargo fmt

# Static analysis
cargo clippy

# Generate documentation
cargo doc --open
```

## 📦 Dependencies

### Core Dependencies

- **endless-sdk**: Official Endless Rust SDK
- **base-infra**: Infrastructure utilities package
- **tokio**: Async runtime
- **serde**: Serialization framework

### Networking and Caching

- **reqwest**: HTTP client
- **moka**: High-performance caching library
- **url**: URL handling

### Cryptography and Encoding

- **hex**: Hexadecimal encoding/decoding
- **bcs**: Binary serialization
- **move-core-types**: Move language core types

## 🤝 Contributing

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Code Standards

- Follow Rust official coding standards
- Format code using `cargo fmt`
- Ensure `cargo clippy` passes without warnings
- Add test cases for new features
- Update relevant documentation

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🔗 Related Links

- [Endless Blockchain](https://endless-labs.github.io)
- [Rust Official Documentation](https://doc.rust-lang.org)
- [Issue Tracker](https://github.com/Alonoril/chain-tools/issues)

## 📈 Version History

- **v0.1.0** - Initial release
  - Basic Endless client functionality
  - Account management and recovery
  - EDS and token transfers
  - Balance query functionality

---

⭐ If this project helps you, please give it a star!