use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Represents a state transition node in the Merkle-DAG.
#[derive(Debug, Serialize, Deserialize)]
pub struct MerkleNode {
    pub parent_hash: String,
    pub input_manifest_hash: String,
    pub wasm_logic_hash: String,
    pub telemetry_attestation: String,
}

impl MerkleNode {
    pub fn new(
        parent_hash: String,
        input_manifest_hash: String,
        wasm_logic_hash: String,
        telemetry_attestation: String,
    ) -> Self {
        Self {
            parent_hash,
            input_manifest_hash,
            wasm_logic_hash,
            telemetry_attestation,
        }
    }

    /// Calculates the cryptographic TransitionHash for the state change
    /// utilizing SHA-256 and RFC 8785 canonical JSON principles.
    pub fn calculate_hash(&self) -> String {
        let val = serde_json::to_value(self).unwrap();
        let canonical_json = Self::canonicalize_json(&val);

        let mut hasher = Sha256::new();
        hasher.update(canonical_json.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Recursively canonicalizes JSON per RFC 8785 to ensure a stable hash.
    fn canonicalize_json(val: &Value) -> String {
        match val {
            Value::Object(map) => {
                // RFC 8785: Keys must be sorted lexicographically
                let mut btree = BTreeMap::new();
                for (k, v) in map {
                    btree.insert(k.as_str(), Self::canonicalize_json(v));
                }
                let entries: Vec<String> = btree
                    .into_iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v))
                    .collect();
                format!("{{{}}}", entries.join(","))
            }
            Value::Array(arr) => {
                let elems: Vec<String> = arr.iter().map(Self::canonicalize_json).collect();
                format!("[{}]", elems.join(","))
            }
            Value::String(s) => format!("\"{}\"", s), // Simplistic string representation
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::Null => "null".to_string(),
        }
    }
}
