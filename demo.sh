#!/bin/bash
set -e

# Project Helio Demonstration Script

echo "==============================================="
echo "🌞 Project Helio - Deterministic Execution Demo"
echo "==============================================="
echo ""

# 1. Ensure dependencies are met
echo "[1/4] Checking Rust target 'wasm32-wasip1'..."
rustup target add wasm32-wasip1 > /dev/null 2>&1

# 2. Build the mocked payload
echo "[2/4] Building Deterministic Scientific Payload (Guest Wasm)..."
cd payload
cargo build --target wasm32-wasip1 --release --quiet
cd ..

# 3. Build the Orchestrator
echo "[3/4] Compiling Helio Orchestrator..."
cargo build --release --quiet

echo "[4/4] Executing Project Helio Orchestrator Payload..."
echo "-----------------------------------------------"
# 4. Run the Orchestrator
HELIO_LOG=info ./target/release/helio --wasm payload/target/wasm32-wasip1/release/payload.wasm

echo "-----------------------------------------------"
echo "✅ Execution Complete."
echo "Inspect the signed cryptographic artifact generated at: helio_manifest.sig.json"
echo ""
