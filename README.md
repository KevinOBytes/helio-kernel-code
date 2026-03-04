# Project Helio

Project Helio is a high-assurance, deterministic execution kernel designed for AI-driven scientific workflows. It guarantees byte-for-byte reproducible execution for stochastic processes, ensures robust capability-based hardware isolation via WASI, and enforces cryptographic provenance natively via a Merkle-DAG architecture.

## Architecture

* **Language**: Rust
* **Execution**: Wasmtime for isolated, architecture-independent task execution via WASI.
* **Determinism**: The `wasi_shim.rs` strictly manages the Wasm Linker, seamlessly overriding and stubbing out environment variables (e.g. `getrandom` and `clock_gettime`) to ensure a completely deterministic state machine.
* **Provenance**: A Merkle-DAG where every experimental step is a verifiable node linking its parent graph, hardware capabilities, and programmatic Wasm logic using RFC 8785 (Canonical JSON) and SHA-256 for stable hashing.
* **Authentication**: Final cryptographic signatures are secured over the payload utilizing Ed25519 signing (`ed25519-dalek`).

## Running the Demonstration

Project Helio includes a complete orchestrator CLI and a mock scientific payload compiled to Wasm to demonstrate its capabilities. 

To run the full pipeline automatically, simply execute the `demo.sh` script:

```bash
chmod +x demo.sh
./demo.sh
```

### What happens in the demo?
1. The shell script compiles a small Rust workload (`payload/`) down to `wasm32-wasip1`. This payload executes math using pseudo-random seeds and checks the system time.
2. The Orchestrator Kernel (`helio-kernel`) is built as a binary.
3. The Orchestrator ingests the compiled Wasm, sandboxes it in `wasmtime`, applies simulated strict `HardwareCapability` boundaries, enforces a strict `fuel_limit` of WebAssembly instructions to prevent runaway payloads, and runs it synchronously.
4. It catches the telemetry output, crafts a `StateTransition` Protobuf describing the state history, canonically serializes it (RFC 8785), hashes it, and strictly signs the execution graph via `Ed25519`.
5. The final proof is dumped to `helio_manifest.sig.json`.

If you run `./demo.sh` a thousand times, the execution hashes and internal logic progression will absolutely never diverge.

## Core Files

- `helio.proto`: The single source of truth for the capability architecture and Merkle-DAG objects.
- `kernel/runtime/wasm_shim.rs`: The Deterministic WASI wrapper enforcing deterministic compute and capabilities.
- `kernel/provenance/merkle_dag.rs`: Canonical Serialization, SHA-256 Hashing, and Ed25519 Signing.
- `tests/integration_test.rs`: Houses the `test_wasi_divergence_verification` proving execution determinism.
- `src/main.rs`: The `helio` CLI orchestrator.
