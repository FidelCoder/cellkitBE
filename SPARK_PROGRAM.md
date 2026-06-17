# Spark Program Scope: CellKit Actions

## Project Overview

Project name: CellKit Actions

One-sentence summary: CellKit Actions is a reusable, private-key-free transaction-action toolkit for CKB apps.

Project type: Developer tool

Repositories:

- Backend: https://github.com/FidelCoder/cellkitBE
- Frontend: https://github.com/FidelCoder/cellkitFE

Deployments:

- Frontend: https://cellkitfe.vercel.app
- Backend health: https://cellkitbe.vercel.app/health

## Funded Sprint Scope

This Spark request is not for an idea from zero. CellKit already has a CKB transfer foundation, action routing, validation helpers, live-cell lookup, deterministic cell selection, and a web playground.

The funded sprint focuses on completing the next production step:

```text
build unsigned CKB transfer transaction
-> sign externally
-> validate signed payload
-> dry-run through CKB RPC
-> broadcast to CKB testnet
-> return real transaction hash and explorer link
```

CellKit does not manage private keys, does not custody funds, and does not hide CKB transaction structure.

## Funding

Requested funding: USD 1,000

This proposal is scoped as a standard Spark technical mini-grant and stays within the regular USD 1,000 threshold. The funded work focuses on completing and verifying the signed testnet transaction workflow rather than expanding into mainnet support, wallet integrations, xUDT production flows, or additional action types.

## Milestones

### Milestone 1: Signed Transaction Validation and Safety Checks

Funding: USD 350

Output: CellKit can accept signed CKB transaction JSON and reject structurally invalid or unsigned payloads before RPC interaction.

Work included:

- Add/complete `POST /api/transactions/validate-signed`
- Validate required transaction fields
- Check inputs, outputs, `outputsData`, cell deps, and witnesses
- Reject empty or placeholder-only witnesses
- Return clear validation errors and warnings
- Add unit and integration tests for signed transaction validation

### Milestone 2: Testnet Dry-Run and Broadcast API

Funding: USD 350

Output: CellKit can dry-run and broadcast signed CKB testnet transactions through configured CKB RPC.

Work included:

- Add/complete `POST /api/transactions/dry-run`
- Add/complete `POST /api/transactions/broadcast`
- Run dry-run before broadcast by default
- Support explicit dry-run skipping only when requested
- Return RPC dry-run result, tx hash, and explorer URL
- Add RPC error handling and missing-config tests

### Milestone 3: Web Flow, Documentation, Demo, and Verification

Funding: USD 300

Output: CellKit provides a usable browser flow and clear verification path for the committee/community.

Work included:

- Add/complete `/broadcast` page
- Add signed transaction JSON input
- Add validate, dry-run, and broadcast controls
- Display validation errors, dry-run cycles, tx hash, and explorer link
- Update playground next-step instructions for external signing
- Add README verification instructions
- Record a short demo showing the full testnet flow
- Publish final progress update with repo links, demo link, and test results

## Milestone-Linked To-Do List

### Week 1: Milestone 1

- Finalize signed transaction request/response models
- Implement signed transaction validation endpoint
- Add witness/signature presence checks
- Add transaction shape validation for required CKB JSON fields
- Add tests for invalid JSON shape, empty witnesses, mismatched outputs, and valid signed payloads

### Week 2: Milestone 2

- Implement CKB RPC dry-run integration
- Implement CKB RPC broadcast integration
- Add dry-run-before-broadcast behavior
- Add tx hash and explorer URL response
- Add missing RPC config and RPC failure tests

### Week 3: Milestone 3

- Build `/broadcast` frontend page
- Add signed transaction JSON input and format helper
- Add validate, dry-run, and broadcast buttons
- Add result panels for errors, cycles, tx hash, and explorer link
- Update playground to point users from unsigned payload generation to external signing and broadcast

### Week 4: Milestone 3

- Run full backend and frontend checks
- Prepare README verification steps
- Record demo video
- Publish final update with repo links, deployment links, test output, and sample testnet transaction hash

## Current State vs Funded Work

Current state:

- Backend action routing exists
- CKB transfer foundation exists
- Testnet address parsing exists
- Indexer-based ordinary cell lookup exists
- Deterministic live-cell selection exists
- Unsigned CKB transaction skeleton construction exists
- Web playground and transaction preview exist

Funded work:

- Complete signed transaction validation
- Complete dry-run and broadcast workflow
- Add public browser-based broadcast flow
- Add final tests and verification docs
- Publish demo and final progress update

## Verification

Backend:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Frontend:

```bash
pnpm lint
pnpm build
```

Manual testnet verification:

1. Start the backend.
2. Start the frontend.
3. Generate an unsigned CKB transfer transaction.
4. Sign the transaction externally with compatible CKB tooling.
5. Paste the signed transaction into CellKit.
6. Validate the signed transaction.
7. Dry-run the signed transaction.
8. Broadcast the signed transaction.
9. Confirm CellKit returns a real transaction hash.
10. Open the returned testnet explorer link.

## Open Source Commitment

CellKit will remain open source.

- License: MIT
- Release timing: all funded work will be pushed publicly during the Spark funding period, with the final version available immediately upon completion.
- Documentation: English documentation will be delivered with the final work. A Chinese summary of the project purpose, usage flow, and verification steps will also be provided for Spark Program review and community understanding.

## Relationship With Existing CKB Tools

CellKit is complementary to existing CKB developer tools such as CCC. It is not intended to replace low-level CKB SDKs, wallet libraries, or signing tools.

CellKit focuses on reusable, inspectable transaction-action workflows at the application layer. Developers can use it to generate and verify common CKB transaction flows, while still signing externally with compatible wallet/tooling. The goal is to reduce repeated transaction boilerplate without hiding the Cell Model or taking custody of private keys.

## CKB Alignment

CellKit is aligned with CKB because it directly improves the developer experience around the Cell Model.

CKB applications depend on transaction construction patterns that are powerful but repetitive: cell lookup, capacity calculation, inputs, outputs, cell deps, witnesses, fee estimation, validation, dry-run, and broadcast. CellKit turns common transaction patterns into reusable actions while keeping the transaction structure visible and verifiable.

