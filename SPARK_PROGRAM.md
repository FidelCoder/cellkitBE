# Spark Program Scope: CellKit Actions

## Project Overview

Project name: CellKit Actions

One-sentence summary: CellKit Actions is a reusable, private-key-free transaction-action toolkit for CKB apps.

Project type: Developer tool

Applicant: Fidelcoder

Contact:

- GitHub: https://github.com/FidelCoder
- Telegram: @GriffinsOduol
- Email: griffinesonyango@gmail.com

Repositories:

- Backend: https://github.com/FidelCoder/cellkitBE
- Frontend: https://github.com/FidelCoder/cellkitFE

Deployments:

- Frontend: https://cellkitfe.vercel.app
- Backend health: https://cellkitbe.vercel.app/health

## Project Background

Building CKB applications requires developers to repeatedly handle low-level transaction work: cell lookup, capacity calculation, inputs, outputs, cell deps, witnesses, fee estimation, validation, dry-run, and broadcast.

This slows down application development and makes simple product flows harder to ship. A developer building a wallet, payment flow, xUDT interface, Spore/DOB tool, escrow flow, or data-cell app should not need to rebuild the same transaction patterns from scratch every time.

CellKit addresses this by turning common CKB transaction patterns into reusable, inspectable action flows.

## Solution

CellKit Actions provides a testnet-first workflow for reusable transaction actions:

```text
build unsigned CKB transaction
-> sign externally
-> validate signed payload
-> dry-run through CKB RPC
-> broadcast to CKB testnet
-> return transaction hash and explorer link
```

CellKit does not manage private keys, does not custody funds, and does not hide CKB transaction structure. The goal is to help developers build faster while keeping the Cell Model visible and verifiable.

## Technical Approach

CellKit has two repositories:

- Backend: Rust + Axum API for CKB transaction actions, validation, indexer integration, dry-run, and broadcast.
- Frontend: Next.js developer playground for building, inspecting, copying, validating, dry-running, and broadcasting transaction payloads.

Backend responsibilities:

- CKB testnet address parsing
- Live-cell lookup through CKB indexer
- Ordinary cell filtering
- Deterministic cell selection
- Fee estimation
- Unsigned transaction skeleton construction
- Signed transaction validation
- CKB RPC dry-run and broadcast

Frontend responsibilities:

- Action form UX
- API error display
- Transaction JSON preview
- Copy flow for external signing
- Signed transaction input
- Validate/dry-run/broadcast UI
- Explorer link display

CellKit remains testnet-first during this sprint.

Out of scope:

- Mainnet support
- Private key handling
- Wallet custody
- In-browser signing
- Token swaps
- User accounts
- Speculative or trading features

## Funding

Requested funding: USD 1,000

The budget has been adjusted from USD 1,500 to USD 1,000 so the proposal fits within the standard Spark Program budget for a technical developer-tool project.

The funded sprint focuses on hardening, verifying, documenting, and demonstrating the signed CKB testnet transaction workflow. It does not expand into mainnet support, wallet integrations, production xUDT flows, or additional action types.

## Milestones

### Milestone 1: Signed Transaction Validation and Safety Checks

Funding: USD 350

Output: CellKit can accept signed CKB transaction JSON and reject structurally invalid or unsigned payloads before RPC interaction.

Work included:

- Finalize `POST /api/transactions/validate-signed`
- Validate required transaction fields
- Check inputs, outputs, `outputsData`, cell deps, and witnesses
- Reject empty or unsigned witness payloads
- Return clear validation errors and warnings
- Add unit and integration tests for signed transaction validation

Acceptance criteria:

- Endpoint returns structured validation results
- Invalid transaction shapes are rejected
- Empty witnesses are detected
- Valid signed payload shapes pass local validation
- Tests cover success and failure cases

### Milestone 2: Testnet Dry-Run and Broadcast API

Funding: USD 350

Output: CellKit can dry-run and broadcast signed CKB testnet transactions through configured CKB RPC.

Work included:

- Finalize `POST /api/transactions/dry-run`
- Finalize `POST /api/transactions/broadcast`
- Run dry-run before broadcast by default
- Support explicit dry-run skipping only when requested
- Return RPC dry-run result, real tx hash, and explorer URL
- Add missing-config and RPC error handling tests

Acceptance criteria:

- Missing RPC configuration returns a clear error
- Dry-run calls CKB RPC and returns cycles/result data
- Broadcast returns a transaction hash on success
- Broadcast returns a testnet explorer link
- Failed dry-run prevents default broadcast

### Milestone 3: Web Flow, Documentation, Demo, and Verification

Funding: USD 300

Output: CellKit provides a usable browser flow and clear verification path for the committee/community.

Work included:

- Finalize `/broadcast` page
- Add signed transaction JSON input
- Add validate, dry-run, and broadcast controls
- Display validation errors, dry-run cycles, tx hash, and explorer link
- Update playground next-step instructions for external signing
- Add README and architecture documentation
- Add open-source contribution and security documentation
- Record a short demo showing the full testnet flow
- Publish final progress update with repo links, demo link, test output, and sample transaction hash

Acceptance criteria:

- User can paste signed JSON into `/broadcast`
- User can validate, dry-run, and broadcast from the UI
- UI shows errors, warnings, dry-run result, tx hash, and explorer URL
- Verification steps are documented without requiring code review
- Final update includes demo and test results

## Timeline and To-Do List

Estimated duration: 4 weeks

### Week 1: Milestone 1

- Finalize signed transaction request/response models
- Complete signed transaction validation endpoint
- Add witness/signature presence checks
- Add transaction shape validation for required CKB JSON fields
- Add tests for invalid shape, empty witnesses, mismatched outputs, and valid signed payloads

### Week 2: Milestone 2

- Complete CKB RPC dry-run integration
- Complete CKB RPC broadcast integration
- Add dry-run-before-broadcast behavior
- Add tx hash and explorer URL response
- Add missing RPC config and RPC failure tests

### Week 3: Milestone 3

- Finalize `/broadcast` frontend page
- Add signed transaction JSON input and format helper
- Add validate, dry-run, and broadcast buttons
- Add result panels for errors, cycles, tx hash, and explorer link
- Update playground to guide users from unsigned payload generation to external signing and broadcast

### Week 4: Milestone 3

- Run backend and frontend checks
- Run full CKB testnet verification
- Prepare README and architecture verification steps
- Record demo video
- Publish final update with repo links, deployment links, test output, and sample testnet transaction hash

## Deliverables

- Signed transaction validation endpoint
- CKB RPC dry-run endpoint
- CKB RPC broadcast endpoint
- Testnet explorer link after successful broadcast
- Broadcast page for pasting, validating, dry-running, and broadcasting signed transactions
- Playground update showing the external signing flow
- Tests covering validation, dry-run, broadcast errors, and transaction shape checks
- README verification instructions
- Architecture documentation
- Open-source contribution and security documentation
- Public demo showing the full workflow
- Final progress update

## How to Verify

The final work can be verified by running the following flow on CKB testnet:

1. Start CellKit backend locally.
2. Start CellKit frontend locally.
3. Generate an unsigned CKB transfer transaction.
4. Sign the transaction externally.
5. Paste the signed transaction into CellKit.
6. Validate the signed transaction.
7. Dry-run the signed transaction.
8. Broadcast the signed transaction.
9. Confirm that CellKit returns a real transaction hash.
10. Open the returned testnet explorer link.
11. Confirm all automated checks pass.

Verification commands:

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

Expected output:

- Backend formatting passes
- Backend clippy passes with no warnings
- Backend unit and integration tests pass
- Frontend lint passes
- Frontend production build succeeds
- Broadcast flow returns a real CKB testnet transaction hash and explorer link

## Current State vs Funded Work

Current state:

- Backend action routing exists
- CKB transfer action route exists
- xUDT transfer action route exists
- Cell consolidation action route exists
- Capacity lock action route exists
- Data cell create action route exists
- Action registry exists
- Validation helpers exist
- Fee estimation helpers exist
- Real CKB testnet address parsing exists
- Real CKB indexer `get_cells` integration exists
- Ordinary cell filtering exists
- Deterministic live-cell selection exists
- Two-pass fee estimation exists
- secp256k1 cell dep config loading exists
- Unsigned CKB transaction skeleton construction exists
- Web playground exists
- Action forms exist
- Transaction JSON preview exists
- Copy flow exists
- API error display exists
- Initial signed-transaction broadcast flow exists on main branches
- README and Spark scope documentation exist

Funded work:

- Harden and finalize signed transaction validation
- Harden and finalize dry-run and broadcast workflow
- Complete public browser-based broadcast flow
- Complete final tests and verification docs
- Improve architecture and open-source documentation
- Record demo and publish final progress update

This Spark request is not for an idea from zero. It is to complete, verify, document, and demonstrate the next production step: turning CellKit into a usable CKB testnet transaction-action workflow.

## Relationship With Existing CKB Tools

CellKit is complementary to existing CKB developer tools such as CCC. It is not intended to replace low-level CKB SDKs, wallet libraries, or signing tools.

CellKit focuses on reusable, inspectable transaction-action workflows at the application layer. Developers can use it to generate and verify common CKB transaction flows, while still signing externally with compatible wallet/tooling.

The goal is to reduce repeated transaction boilerplate without hiding the Cell Model or taking custody of private keys.

## CKB Alignment

CellKit is aligned with CKB because it directly improves the developer experience around the Cell Model.

CKB applications depend on transaction construction patterns that are powerful but repetitive. CellKit turns common transaction patterns into reusable actions without hiding the underlying structure.

This can help future CKB builders ship faster across:

- Wallets
- Payment tools
- xUDT apps
- Spore/DOB apps
- Data-cell apps
- Escrow flows
- Testnet prototypes
- Developer tools

The goal is practical ecosystem utility: make it easier to build working CKB apps.

## Open Source Commitment

CellKit will remain open source.

License: MIT

Release timing: all funded work will be pushed publicly during the Spark funding period, with the final version available immediately upon completion.

Public development model:

- Backend and frontend repositories are public
- Main-branch documentation describes setup, architecture, verification, contribution, and security policy
- Issues and pull requests are welcome for bugs, documentation improvements, tests, and scoped feature work
- Security issues should be reported privately by email before public disclosure

Documentation language plan:

- English documentation will be maintained in the repositories
- A Chinese summary of the project purpose, usage flow, and verification steps will be provided for Spark Program review and community understanding

Open-source materials:

- MIT license
- README documentation
- Architecture documentation
- Contributing guide
- Security policy
- Code of conduct
