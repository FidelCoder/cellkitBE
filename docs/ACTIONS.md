# MVP Actions

## `ckb.transfer`

Purpose: build an unsigned CKB transfer transaction from one testnet address to another.

Fields: `network`, `fromAddress`, `toAddress`, `amountCkb`, `feeRate`.

Required chain access: CKB indexer for live cells and CKB RPC for future dry-run support.

Limitations: no signing, no broadcast, no fake input cells.

## `xudt.transfer`

Purpose: build an unsigned xUDT transfer with token inputs, receiver output, token change, CKB funding/change, xUDT cell deps, and witness placeholders.

Fields: `network`, `fromAddress`, `toAddress`, `xudtTypeScript`, `amount`, `feeRate`.

Required chain access: CKB indexer. Required config: `TESTNET_XUDT_*` variables.

Limitations: script config is never guessed. Missing config returns `xUDT cell dep is not configured for this network.`

## `cell.consolidate`

Purpose: consolidate fragmented ordinary CKB cells back to one owner address.

Fields: `network`, `ownerAddress`, `maxCells`, `feeRate`.

Required chain access: CKB indexer.

Limitations: MVP must only consolidate ordinary cells without type scripts. Unknown type scripts are not touched.

## `capacity.lock`

Purpose: send CKB capacity to a target lock address for escrow-like or milestone-reserve flows.

Fields: `network`, `fromAddress`, `lockAddress`, `amountCkb`, `memo`, `feeRate`.

Required chain access: CKB indexer.

Limitations: this is not a custom lock script. The memo is action metadata unless future data-cell support stores it on-chain.

## `data_cell.create`

Purpose: create an ordinary cell containing simple data.

Fields: `network`, `fromAddress`, `data.encoding`, `data.content`, `capacityCkb`, `feeRate`.

Required chain access: CKB indexer.

Limitations: `data.encoding` supports `utf8` and `hex`. Capacity checks are validated before transaction construction, and full occupied-capacity enforcement belongs in the live transaction builder.
