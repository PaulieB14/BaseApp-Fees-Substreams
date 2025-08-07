# BaseApp Fees Substreams

A high-performance Substreams project for tracking fee events from multiple paymaster contracts on the Base network.

## Features

- **Real-time Fee Tracking**: Monitors FeeCollected, FeeDistributed, FeeUpdated, and OwnershipTransferred events
- **Multi-Contract Support**: Tracks 3 specific paymaster contracts on Base:
  - VerifyingSingletonPaymaster: `0x00000f79b7faf42eebadba19acc07cd08af44789`
  - BiconomyTokenPaymaster: `0x00000f7365ca6c59a2c93719ad53d567ed49c14c`
  - VerifyingSingletonPaymaster2: `0x000000f6faeda8f7bfa1a8589b4b6e2d71c37592`
- **High-Performance Architecture**: Optimized for real-time processing and multiple consumers
- **Production Ready**: Published to Substreams registry for easy deployment

## Setup

1. Install Rust and Cargo
2. Install Substreams CLI: `cargo install substreams`
3. Build the project: `cargo build --release`

## Usage

### From Registry (Recommended)

```bash
# Stream with GUI
substreams gui baseapp_fees@v1.0.1

# Run from command line (Base mainnet)
substreams run -e base-mainnet.streamingfast.io:443 baseapp_fees@v1.0.1 map_fee_events --start-block 31229579 --stop-block +1
```

### Local Development

```bash
# Test with authentication (get API key from https://substreams.dev)
substreams run -e base-mainnet.streamingfast.io:443 substreams.yaml map_fee_events --start-block 31229579 --stop-block +1
```

## Event Tracking

The Substreams monitors these critical fee events:
- **FeeCollected**: When fees are collected from users
- **FeeDistributed**: When fees are distributed to recipients  
- **FeeUpdated**: When fee rates are updated
- **OwnershipTransferred**: When contract ownership changes

## Performance Benefits

This Substreams is optimized for high-throughput fee tracking with:
- **Efficient Event Filtering**: By contract address for minimal processing overhead
- **Real-time Processing**: Sub-second latency for fee event detection
- **Multiple Consumers**: Support for databases, APIs, dashboards, and analytics
- **Scalable Architecture**: Can handle high-volume fee monitoring across multiple contracts

## Project Structure

- `src/lib.rs` - Core fee tracking logic
- `substreams.yaml` - Substreams manifest with Base logo
- `Cargo.toml` - Rust dependencies
- `target/wasm32-unknown-unknown/release/substreams.wasm` - Compiled WASM binary

## Registry

Published to Substreams Registry: https://substreams.dev/packages/baseapp-fees/v1.0.1

## Why BaseApp Fees?

BaseApp Fees is specifically designed for Base network fee monitoring with:
- **Base Network Optimization**: Tailored for Base's specific requirements (Chain ID: 8453)
- **Paymaster Focus**: Specialized for paymaster contract monitoring
- **Real-time Analytics**: Perfect for fee tracking dashboards and alerts
- **Production Deployment**: Ready for enterprise fee monitoring solutions

## Network Configuration

This Substreams is configured for **Base mainnet** (Chain ID: 8453) and uses the StreamingFast endpoint for optimal performance.
