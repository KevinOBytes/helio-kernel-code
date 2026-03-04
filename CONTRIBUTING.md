# Contributing to Project Helio

Thank you for your interest in contributing to Project Helio! As a deterministic execution kernel, we hold our code to a high standard of precision, security, and cryptographic correctness.

## Getting Started

1. **Fork & Branch:** Fork the repository and create your feature branch from `main`.
2. **Setup:** You will need Rust installed (`rustup`). The project uses Wasmtime and WASIp1.
3. **Build:** Run `cargo build` to test compilation.

## Determinism Rule
Project Helio enforces **100% byte-for-byte execution determinism**. When contributing:
- Never introduce host-dependent logic (like wall-clock timers or host RNG) inside the `wasm_shim.rs` sandbox without rigorously isolating it.
- Ensure all execution constraints tie back to `HardwareCapability` or `fuel_limit`.

## Testing Your Changes
Before opening a Pull Request, you must ensure all tests and lints pass:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Pull Request Process
1. Use the provided PR template.
2. Ensure your PR description clearly describes the problem and solution.
3. If applicable, add or update integration tests to demonstrate the functionality.
4. A maintainer will review and approve your PR before merging.
