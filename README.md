# Kailua

**Real-time, pay-per-query blockchain data and services framework — fully Solana-native and powered by the x402 protocol.**

Kailua is an open-source Solana program that provides a generic, on-chain framework for **pay-per-query** access to data or off-chain services. It enables clients — including AI agents, decentralized applications, or traditional services — to submit structured requests on-chain, settle instant micropayments via the [x402 protocol](https://www.x402.org/), and receive verifiable fulfillment results.

The design emphasizes transparency, auditability, and extensibility, making it suitable for building monetized oracles, data marketplaces, virtual top-up (VTU) services, premium content gateways, and more.

## Features

- **Solana-Native Execution**: Built with Rust and the Anchor framework for efficient, secure on-chain logic.
- **x402 Integration**: Leverages the HTTP 402 "Payment Required" standard for seamless, non-custodial micropayments (developed by Coinbase).
- **Generic Query Model**: Arbitrary `query_payload` strings allow flexible request formats (e.g., JSON-encoded parameters).
- **Verifiable Lifecycle**: On-chain accounts track submission, payment status, and delivery with emitted events for off-chain monitoring.
- **Provider Flexibility**: Supports single-provider or multi-provider fulfillment models.

## Architecture Overview

The core workflow combines on-chain coordination with off-chain payment enforcement:

1. **Query Submission**: Client calls `submit_query` to create a PDA-seeded account with payload and payment amount.
2. **Payment Enforcement**: Off-chain server responds with HTTP 402 and payment instructions.
3. **Settlement & Verification**: Client pays on-chain; provider verifies and updates status.
4. **Delivery**: Provider executes the request off-chain and records results on-chain via `deliver_data`.

Key instructions include:
- `submit_query`
- `verify_payment`
- `deliver_data`
- `getquery_status`
- `log_metrics`

## Getting Started

### Prerequisites

- Rust toolchain (via `rustup`)
- Solana CLI
- Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`)
- Node.js and Yarn (for potential TypeScript clients/tests)

### Build and Test

```bash
# Clone the repository
git clone https://github.com/idorocodes/kailua.git
cd kailua

# Build the program
anchor build

# Run tests
anchor test
```

### Deployment

```bash
# Deploy to devnet (update Anchor.toml cluster as needed)
anchor deploy
```

For production use, deploy to mainnet-beta and integrate with an x402-compatible server (reference Coinbase's [@x402](https://github.com/coinbase/x402) libraries).

## Use Cases

- Real-time blockchain data feeds for AI agents
- Decentralized VTU and bill payment services
- Pay-per-view content or API access
- Custom oracles and computation tasks

## Status

Kailua is in early development. Contributions, feedback, and integrations are welcome.

## License

[MIT License](LICENSE) (or specify if different)

---

Built by [@idorocodes](https://github.com/idorocodes) on Solana.
