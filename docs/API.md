# CellKit Actions API

Base URL: `http://localhost:8080`

## `GET /health`

Response:

```json
{
  "status": "ok",
  "service": "cellkit-actions-api"
}
```

## `GET /api/actions`

Returns the MVP action registry.

```json
{
  "actions": [
    {
      "id": "ckb.transfer",
      "name": "CKB Transfer",
      "description": "Build an unsigned CKB transfer transaction.",
      "endpoint": "/api/actions/ckb-transfer/build",
      "status": "mvp"
    }
  ]
}
```

## Build Endpoints

All build endpoints accept `network: "testnet"` and return either a `BuildActionResponse` or a structured error. Builders never fake chain state. If live cell selection is required and `CKB_INDEXER_URL` is missing, the API returns `503 missing_config`.

### `POST /api/actions/ckb-transfer/build`

```json
{
  "network": "testnet",
  "fromAddress": "ckt1...",
  "toAddress": "ckt1...",
  "amountCkb": "100",
  "feeRate": "1000"
}
```

### `POST /api/actions/xudt-transfer/build`

```json
{
  "network": "testnet",
  "fromAddress": "ckt1...",
  "toAddress": "ckt1...",
  "xudtTypeScript": {
    "codeHash": "0x...",
    "hashType": "type",
    "args": "0x..."
  },
  "amount": "1000",
  "feeRate": "1000"
}
```

### `POST /api/actions/cell-consolidation/build`

```json
{
  "network": "testnet",
  "ownerAddress": "ckt1...",
  "maxCells": 20,
  "feeRate": "1000"
}
```

### `POST /api/actions/capacity-lock/build`

```json
{
  "network": "testnet",
  "fromAddress": "ckt1...",
  "lockAddress": "ckt1...",
  "amountCkb": "100",
  "memo": "Grant milestone reserve",
  "feeRate": "1000"
}
```

### `POST /api/actions/data-cell-create/build`

```json
{
  "network": "testnet",
  "fromAddress": "ckt1...",
  "data": {
    "encoding": "utf8",
    "content": "Hello CKB"
  },
  "capacityCkb": "100",
  "feeRate": "1000"
}
```

## `POST /api/actions/validate`

Validates transaction skeleton shape.

```json
{
  "network": "testnet",
  "action": "xudt.transfer",
  "transaction": {
    "version": "0x0",
    "cellDeps": [],
    "headerDeps": [],
    "inputs": [],
    "outputs": [],
    "outputsData": [],
    "witnesses": []
  }
}
```

Response:

```json
{
  "valid": false,
  "errors": ["transaction.inputs must contain at least one input"],
  "warnings": []
}
```

## `POST /api/actions/estimate-fee`

Estimates fee from serialized transaction size and CKB fee rate in shannons per kilobyte.

```json
{
  "network": "testnet",
  "transaction": {
    "version": "0x0",
    "cellDeps": [],
    "headerDeps": [],
    "inputs": [],
    "outputs": [],
    "outputsData": [],
    "witnesses": []
  },
  "feeRate": "1000"
}
```

Response:

```json
{
  "feeRate": "1000",
  "estimatedSizeBytes": 94,
  "estimatedFeeShannons": "94",
  "estimatedFeeCkb": "0.00000094"
}
```
