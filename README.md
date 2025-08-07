# Base Fees Substreams

A Substreams project for tracking fee events from multiple paymaster contracts on the Base network.

## Features

- Tracks FeeCollected, FeeDistributed, FeeUpdated, and OwnershipTransferred events
- Monitors 3 specific paymaster contracts on Base:
  - VerifyingSingletonPaymaster: `0x00000f79b7faf42eebadba19acc07cd08af44789`
  - BiconomyTokenPaymaster: `0x00000f7365ca6c59a2c93719ad53d567ed49c14c`
  - VerifyingSingletonPaymaster2: `0x000000f6faeda8f7bfa1a8589b4b6e2d71c37592`

## Setup

1. Install Rust and Cargo
2. Install Substreams CLI: `cargo install substreams`
3. Build the project: `cargo build --release`

## Usage

### Local Testing

```bash
# Test with authentication (get API key from https://substreams.dev)
substreams run -e base-mainnet.streamingfast.io:443 substreams.yaml map_fee_events --start-block 31229579 --stop-block +1
```

### Deploy to Substreams Registry

```bash
# Authenticate with Substreams
substreams auth

# Publish to registry
substreams publish
```

## Project Structure

- `src/lib.rs` - Main Rust implementation
- `substreams.yaml` - Substreams manifest
- `Cargo.toml` - Rust dependencies
- `target/wasm32-unknown-unknown/release/substreams.wasm` - Compiled WASM binary

## Event Tracking

The Substreams monitors these events:
- **FeeCollected**: When fees are collected from users
- **FeeDistributed**: When fees are distributed to recipients  
- **FeeUpdated**: When fee rates are updated
- **OwnershipTransferred**: When contract ownership changes

## Performance

This Substreams is optimized for high-throughput fee tracking with:
- Efficient event filtering by contract address
- Real-time processing capabilities
- Support for multiple downstream consumers
