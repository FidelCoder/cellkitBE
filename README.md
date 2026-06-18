# CellKit Actions Backend

CellKit Actions Backend is a Rust/Axum API for reusable CKB transaction-action workflows. It helps CKB app developers build inspectable transaction payloads, validate signed payloads, dry-run through CKB RPC, and broadcast to CKB testnet without handling private keys or custodying funds.

The backend is the chain-aware part of CellKit. It owns CKB address parsing, live-cell lookup, ordinary cell selection, transaction skeleton construction, transaction shape validation, fee estimation, dry-run, and broadcast.

## Status

Current status: testnet MVP.

Implemented:

- Action registry for reusable transaction templates
- CKB transfer request validation
- CKB testnet address parsing with CKB SDK
- CKB indexer `get_cells` integration
- Ordinary CKB cell filtering
- Deterministic live-cell selection
- Two-pass fee estimation for CKB transfers
- Unsigned CKB transaction skeleton construction
- Signed transaction shape validation
- CKB RPC dry-run endpoint
- CKB RPC broadcast endpoint
- Testnet explorer URL response
- Axum runtime and Vercel runtime entry points
- Unit and integration tests

Partially scaffolded:

- xUDT transfer
- Cell consolidation
- Capacity lock
- Data cell creation

Those action routes currently validate inputs/configuration but do not fake chain state. Live selection/build logic for those actions is intentionally outside the current MVP.

## Architecture

Read the full architecture document:

- [`ARCHITECTURE.md`](./ARCHITECTURE.md)

High-level flow:

```text
Frontend / external client
  -> Axum route
  -> action builder or transaction service
  -> CKB helper modules
  -> CKB indexer / CKB RPC
  -> structured JSON response
```

Core backend modules:

- `src/routes` - HTTP route wiring
- `src/actions` - action builders and transaction services
- `src/ckb` - CKB address, cell, indexer, RPC, script, and transaction helpers
- `src/models` - request/response/transaction API shapes
- `src/utils` - amount, fee, hex, and validation helpers
- `src/config.rs` - environment-driven runtime configuration
- `src/error.rs` - API error model and HTTP response conversion

## API Overview

Health:

- `GET /health`

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

## Transaction Workflow

```text
1. Build unsigned CKB transfer transaction
2. Sign externally with compatible CKB tooling or wallet software
3. Paste/submit signed transaction JSON
4. Validate signed transaction shape and witnesses
5. Dry-run through configured CKB RPC
6. Broadcast to CKB testnet
7. Return transaction hash and testnet explorer link
```

CellKit never asks for, stores, or derives private keys.

## Environment

Create a `.env` file from `.env.example`.

Common variables:

```bash
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
CKB_NETWORK=testnet
CKB_RPC_URL=
CKB_INDEXER_URL=
DEFAULT_FEE_RATE=1000
TESTNET_SECP256K1_TX_HASH=
TESTNET_SECP256K1_INDEX=0x0
TESTNET_SECP256K1_DEP_TYPE=dep_group
```

Optional xUDT configuration:

```bash
TESTNET_XUDT_CODE_HASH=
TESTNET_XUDT_HASH_TYPE=type
TESTNET_XUDT_TX_HASH=
TESTNET_XUDT_INDEX=0x0
TESTNET_XUDT_DEP_TYPE=code
```

Notes:

- `CKB_INDEXER_URL` is required for live-cell-backed transaction building.
- `CKB_RPC_URL` is required for dry-run and broadcast.
- `TESTNET_SECP256K1_*` is required for real CKB transfer skeletons.
- The MVP supports CKB testnet only.

## Development

Install Rust, then run:

```bash
cargo run
```

By default the backend listens on:

```text
http://localhost:8080
```

## Verification

Recommended checks:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Expected result:

- Formatting passes
- Clippy returns no warnings
- Unit and integration tests pass

## Deployment

This repo includes:

- `src/main.rs` for a standard Axum server
- `api/axum.rs` for Vercel runtime deployment
- `vercel.json` rewrite configuration

The deployed backend health check is:

```text
https://cellkitbe.vercel.app/health
```

## Security Model

CellKit is intentionally private-key-free.

- No private key input
- No wallet custody
- No in-browser signing
- No account system
- No mainnet support in this MVP
- Signed transactions are treated as user-provided JSON payloads
- Broadcast is testnet-only and should be reviewed by the user before submission

See [`SECURITY.md`](./SECURITY.md) for reporting and safety expectations.

## Relationship With Existing CKB Tools

CellKit is complementary to existing CKB developer tools such as CCC. It is not intended to replace low-level CKB SDKs, wallet libraries, or signing tools.

CellKit focuses on reusable, inspectable transaction-action workflows at the application layer. Developers can use it to generate and verify common CKB transaction flows, while still signing externally with compatible wallet/tooling.

The goal is to reduce repeated transaction boilerplate without hiding the Cell Model or taking custody of private keys.

## Open Source

CellKit is open source under the MIT License.

- License: [`LICENSE`](./LICENSE)
- Contributing guide: [`CONTRIBUTING.md`](./CONTRIBUTING.md)
- Security policy: [`SECURITY.md`](./SECURITY.md)
- Spark Program scope: [`SPARK_PROGRAM.md`](./SPARK_PROGRAM.md)

Public development happens in this repository. Issues, bug reports, documentation improvements, tests, and narrowly scoped feature contributions are welcome.

## Related Repository

Frontend:

```text
https://github.com/FidelCoder/cellkitFE
```
