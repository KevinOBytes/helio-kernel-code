use anyhow::{Context, Result};
use clap::Parser;
use helio_kernel::proto::{Experiment, HardwareCapability, StateTransition, Task};
use helio_kernel::provenance::merkle_dag::{generate_signing_key, sign_transition};
use helio_kernel::runtime::wasm_shim::DeterministicSandbox;
use std::fs;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about = "Project Helio Deterministic Wasm Execution CLI", long_about = None)]
struct Args {
    /// Path to the Experiment Manifest
    #[arg(short, long, default_value = "experiment.json")]
    manifest: String,

    /// Path to the compiled Wasm payload
    #[arg(short, long)]
    wasm: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("helio=info".parse()?))
        .with_target(false)
        .init();

    let args = Args::parse();
    info!("Starting Project Helio Orchestrator");
    info!("Loading Wasm payload from {:?}", args.wasm);

    let wasm_bytes = fs::read(&args.wasm).context("Failed to read Wasm payload file")?;

    // In a real application, the manifest would supply these capabilities.
    // For the orchestrator demo, we mock building an Experiment profile.
    let capability = HardwareCapability {
        allow_network: false,
        allow_gpu: false,
        memory_limits: vec!["10MB".to_string()],
    };

    let experiment = Experiment {
        id: "HELIO-DEMO-001".to_string(),
        tasks: vec![Task {
            id: "task-001".to_string(),
            wasm_logic_hash: "mock-hash-123".to_string(),
            fuel_limit: 10_000_000,
        }],
        capability: Some(capability.clone()),
    };

    info!("Initializing Deterministic Sandbox...");
    let sandbox = DeterministicSandbox::new(&capability);

    info!("Executing Wasm Logic...");
    let telemetry = sandbox
        .execute(
            &wasm_bytes,
            experiment.tasks[0].fuel_limit,
            &capability,
        )
        .context("Wasm execution failed")?;

    info!("Wasm execution successful. Building provenance graph...");

    let transition = StateTransition {
        current_state_hash: "".to_string(),
        parent_hash: "GENESIS".to_string(),
        input_manifest_hash: experiment.id.clone(),
        wasm_logic_hash: experiment.tasks[0].wasm_logic_hash.clone(),
        telemetry_attestation: telemetry,
    };

    let signing_key = generate_signing_key();
    let signature = sign_transition(&transition, &signing_key);

    info!("Cryptographic Signature generated: {}", signature);

    // Output final manifest payload
    let final_attestation = serde_json::json!({
        "experiment_id": experiment.id,
        "signature": signature,
        "transition": {
            "parent_hash": transition.parent_hash,
            "input_manifest_hash": transition.input_manifest_hash,
            "wasm_logic_hash": transition.wasm_logic_hash,
            "telemetry_attestation": transition.telemetry_attestation
        }
    });

    let manifest_path = "helio_manifest.sig.json";
    fs::write(
        manifest_path,
        serde_json::to_string_pretty(&final_attestation)?,
    )
    .context("Failed to write signed manifest")?;

    info!("Successfully wrote verified execution to {}", manifest_path);

    Ok(())
}
