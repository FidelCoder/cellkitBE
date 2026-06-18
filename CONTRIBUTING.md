# Contributing to CellKit Actions Backend

Thank you for considering a contribution to CellKit.

CellKit is a CKB testnet developer tool focused on reusable, inspectable transaction-action workflows. Contributions should preserve the project's core safety model: no private keys, no custody, no hidden transaction behavior, and testnet-first development.

## Ways to Contribute

- Bug reports
- Documentation improvements
- Test coverage
- Validation improvements
- CKB RPC/indexer integration improvements
- New action-builder implementations
- Error message improvements
- Developer experience improvements

## Scope Guidelines

Good contributions are:

- Small enough to review
- Backed by tests when behavior changes
- Explicit about CKB network assumptions
- Careful with transaction safety
- Compatible with the external-signing model

Please discuss larger changes before opening a large pull request.

## Development Setup

```bash
cargo run
```

The backend defaults to:

```text
http://localhost:8080
```

Use `.env.example` as the environment template.

## Required Checks

Before opening a pull request, run:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

If a check cannot run because of local environment limitations, explain that clearly in the pull request.

## Pull Request Expectations

Pull requests should include:

- Clear summary of the change
- Reason for the change
- Testing performed
- Any new environment variables
- Any security or transaction-safety considerations

For transaction-building changes, include enough detail for reviewers to understand:

- Which cells can be selected
- Which scripts are involved
- How fees are estimated
- How witnesses are handled
- What remains unsigned

## Safety Rules

Do not add:

- Private key handling
- Seed phrase handling
- Custodial flows
- Hidden signing
- Mainnet broadcast behavior without explicit project approval
- Speculative trading or swap behavior

## Documentation

Documentation should be accurate, concise, and reproducible. Prefer examples that can be verified through commands, UI steps, or testnet transaction hashes.

English documentation is the default. Chinese summaries may be added for Spark Program/community review.
