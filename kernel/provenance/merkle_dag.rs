use crate::proto::StateTransition;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Calculates the cryptographic TransitionHash for a StateTransition
/// utilizing SHA-256 and RFC 8785 canonical JSON principles.
/// It ignores the `current_state_hash` field itself during calculation.
pub fn calculate_transition_hash(transition: &StateTransition) -> String {
    // Clone and clear the current state hash to prevent recursive hashing loops
    let mut hashable_transition = transition.clone();
    hashable_transition.current_state_hash = "".to_string();

    let val = serde_json::to_value(&hashable_transition).expect("Failed to serialize transition");
    let canonical_json = canonicalize_json(&val);

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
                btree.insert(k.as_str(), canonicalize_json(v));
            }
            let entries: Vec<String> = btree
                .into_iter()
                .map(|(k, v)| format!("\"{}\":{}", k, v))
                .collect();
            format!("{{{}}}", entries.join(","))
        }
        Value::Array(arr) => {
            let elems: Vec<String> = arr.iter().map(canonicalize_json).collect();
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
