# CellKit Actions Backend

CellKit Actions Backend is a Rust API for reusable CKB transaction-action templates. It is designed for CKB app developers who need normalized inputs, unsigned transaction payload structure, required deps/witness placeholders, fee estimates, warnings, and next steps without rebuilding common action logic in every app.

This is not a blockchain explorer, wallet, education app, transaction simulator, signing service, or custody service. It never stores private keys and never claims a transaction was sent unless a future broadcast endpoint explicitly does that.

## MVP Actions

- `ckb.transfer`: build an unsigned CKB transfer transaction.
- `xudt.transfer`: build an unsigned xUDT transfer transaction.
- `cell.consolidate`: consolidate ordinary CKB cells owned by one address.
- `capacity.lock`: move capacity to another lock address with memo metadata.
- `data_cell.create`: create a simple data cell.

The current implementation includes validation, action registry, transaction shape validation, fee estimation, configuration, API errors, docs, and test coverage. Action builders that require live cells return a clear error when `CKB_INDEXER_URL` is missing. If script config or live cell selection is unavailable, the API fails honestly rather than returning fake inputs or fake transaction hashes.

## Configuration

Copy `.env.example` to `.env` and fill values as needed:

```bash
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
CKB_NETWORK=testnet
CKB_RPC_URL=
CKB_INDEXER_URL=
DEFAULT_FEE_RATE=1000
TESTNET_XUDT_CODE_HASH=
TESTNET_XUDT_HASH_TYPE=type
TESTNET_XUDT_TX_HASH=
TESTNET_XUDT_INDEX=
TESTNET_XUDT_DEP_TYPE=code
```

`CKB_INDEXER_URL` is required for actions that select live cells. xUDT actions also require the `TESTNET_XUDT_*` variables. Testnet is the only supported MVP network.

## Run Locally

```bash
cargo fmt
cargo clippy
cargo test
cargo run
```

The server defaults to `http://localhost:8080`.

## API Examples

```bash
curl http://localhost:8080/health
curl http://localhost:8080/api/actions
```

```bash
curl -X POST http://localhost:8080/api/actions/ckb-transfer/build \
  -H 'content-type: application/json' \
  -d '{
    "network": "testnet",
    "fromAddress": "ckt1...",
    "toAddress": "ckt1...",
    "amountCkb": "100",
    "feeRate": "1000"
  }'
```

If no indexer is configured, the transfer endpoint returns:

```json
{
  "error": "missing_config",
  "message": "CKB indexer is not configured. Cell selection requires live chain access."
}
```

## Frontend Consumption

A frontend should call `GET /api/actions` for the supported action list, render the relevant form, submit to the matching build endpoint, and show either the unsigned transaction payload or the structured backend error. The frontend should pass the transaction object from build responses to `POST /api/actions/validate` and `POST /api/actions/estimate-fee` when it wants additional checks or fee previews.

## Current Limitations

- Testnet only.
- Unsigned payloads only.
- No wallet signing or broadcasting.
- No database.
- No private key storage.
- Live indexer/RPC transaction construction is stubbed with explicit errors rather than fake chain state.
- Full CKB address-to-lock-script parsing is deferred to the live CKB integration layer.

## Roadmap

- Implement CKB address parsing with official CKB crates.
- Fetch live cells from CKB indexer and select inputs for each action.
- Add real transaction skeleton construction with cell deps and witness placeholders.
- Add optional RPC dry-run/cycle estimation.
- Add optional broadcast endpoint for already signed transactions.
- Add Spore-related actions once script config and SDK assumptions are explicit.
