# CellKit Actions Backend

Rust/Axum backend for CellKit Actions, a testnet-first CKB developer tool for reusable transaction-action workflows.

CellKit helps developers build, inspect, validate, dry-run, and broadcast CKB transaction payloads without managing private keys or custodying funds. The backend is responsible for chain-aware work such as address parsing, live cell lookup, transaction skeleton construction, RPC dry-run, and testnet broadcast.

## Current Scope

- CKB testnet only
- Private-key-free workflow
- Unsigned CKB transfer transaction skeletons
- External signing flow
- Signed transaction validation
- CKB RPC dry-run
- CKB RPC broadcast
- Transaction hash and testnet explorer link response

Out of scope for the current Spark sprint:

- Mainnet support
- Private key handling
- Wallet custody
- In-browser signing
- Token swaps
- User accounts
- Trading/speculative features

## API Endpoints

Action endpoints:

- `GET /api/actions`
- `POST /api/actions/ckb-transfer/build`
- `POST /api/actions/xudt-transfer/build`
- `POST /api/actions/cell-consolidation/build`
- `POST /api/actions/capacity-lock/build`
- `POST /api/actions/data-cell-create/build`
- `POST /api/actions/validate`
- `POST /api/actions/estimate-fee`

Signed transaction endpoints:

- `POST /api/transactions/validate-signed`
- `POST /api/transactions/dry-run`
- `POST /api/transactions/broadcast`

Health:

- `GET /health`

## Environment

Create a `.env` file from `.env.example`.

Important variables:

- `CKB_NETWORK=testnet`
- `CKB_RPC_URL`
- `CKB_INDEXER_URL`
- `DEFAULT_FEE_RATE`
- `TESTNET_SECP256K1_TX_HASH`
- `TESTNET_SECP256K1_INDEX`
- `TESTNET_SECP256K1_DEP_TYPE`
- `TESTNET_XUDT_CODE_HASH`
- `TESTNET_XUDT_TX_HASH`
- `TESTNET_XUDT_INDEX`

`CKB_INDEXER_URL` is required for live-cell-backed transaction building. `CKB_RPC_URL` is required for dry-run and broadcast.

## Development

```bash
cargo run
```

By default the backend listens on `0.0.0.0:8080`.

## Verification

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Expected result:

- Formatting passes
- Clippy returns no warnings
- Unit and integration tests pass

## Relationship With Existing CKB Tools

CellKit is complementary to existing CKB developer tools such as CCC. It is not intended to replace low-level SDKs, wallet libraries, or signing tools.

CellKit focuses on reusable, inspectable transaction-action workflows at the application layer. Developers can use it to generate and verify common CKB transaction flows, while still signing externally with compatible wallet/tooling.

## Open Source

This repository is released under the MIT License. See `LICENSE`.

