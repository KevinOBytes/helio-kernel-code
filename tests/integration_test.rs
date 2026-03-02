use helio_kernel::proto::StateTransition;
use helio_kernel::provenance::merkle_dag::calculate_transition_hash;
use helio_kernel::runtime::wasm_shim::DeterministicSandbox;

#[test]
fn test_wasi_deterministic_sandbox() {
    let wasm_wat = r#"
        (module
            (import "wasi_snapshot_preview1" "args_sizes_get" (func $args_sizes_get (param i32 i32) (result i32)))
            (memory 1)
            (export "memory" (memory 0))
            (func $main 
                ;; Just return immediately
            )
            (export "_start" (func $main))
        )
    "#;
    let wasm_bytes = wat::parse_str(wasm_wat).expect("Failed to parse WAT");

    let sandbox = DeterministicSandbox::new();
    let result = sandbox.execute(&wasm_bytes, 100);

    assert!(result.is_ok(), "Wasm execution failed: {:?}", result.err());
    assert_eq!(result.unwrap(), "Wasm execution completed");
}

#[test]
fn test_merkle_dag_canonical_hashing() {
    let mut transition1 = StateTransition {
        current_state_hash: "should-be-ignored".to_string(),
        parent_hash: "parent123".to_string(),
        input_manifest_hash: "input456".to_string(),
        wasm_logic_hash: "wasm789".to_string(),
        telemetry_attestation: "telemetry000".to_string(),
    };

    let hash1 = calculate_transition_hash(&transition1);

    // Change the current_state_hash; the result should be identical
    transition1.current_state_hash = "another-value".to_string();
    let hash2 = calculate_transition_hash(&transition1);

    assert_eq!(
        hash1, hash2,
        "Canonical hash should ignore current_state_hash"
    );

    // Change a meaningful field; the result should change
    transition1.parent_hash = "parent999".to_string();
    let hash3 = calculate_transition_hash(&transition1);

    assert_ne!(
        hash1, hash3,
        "Canonical hash should change when parent_hash changes"
    );

    // Test that the hash is stable across test runs (a hardcoded known hash check could be done,
    // but verifying consistency and ignoring `current_state_hash` is the goal here).
}
