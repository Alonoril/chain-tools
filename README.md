# Chain Tools

A collection of tools for interacting with blockchain networks, currently focused on the Endless blockchain platform.

## Project Structure

This is a Rust workspace project with the following crates:

- `endless-client`: A client library for interacting with the Endless blockchain

## endless-client

The `endless-client` crate provides a convenient interface for interacting with the Endless blockchain network. It includes:

### Features

1. **Account Management**
   - Private key recovery for Ed25519 keys
   - Local account creation and management

2. **REST Client**
   - Index endpoint interaction
   - Chain ID caching
   - Entry function submission
   - View function calls

3. **Transaction Handling**
   - Transaction building and signing
   - Gas configuration overrides
   - Timeout management

### Modules

- `account`: Extensions for local account recovery
- `rest_client`: Main client implementation for REST API interactions
- `types`: Supporting types and structures for client operations
- `error`: Error definitions and handling

## Dependencies

Key dependencies include:

- `endless-sdk`: Official Endless Rust SDK
- `base-infra`: Infrastructure utilities
- `tokio`: Asynchronous runtime
- `serde`: Serialization framework
- `moka`: Caching library
- `reqwest`: HTTP client

## Getting Started

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
endless-client = { git = "https://github.com/Alonoril/chain-tools", branch = "main" }
```
