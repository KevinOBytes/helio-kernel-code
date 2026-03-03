use rand::Rng;
use std::time::SystemTime;

fn main() {
    println!("--- Helio Deterministic Guest Worker started ---");

    // 1. Stochastic math using the environment RNG
    let mut rng = rand::rng();
    let r = rng.next_u32();
    println!("Generated Random Number: {}", r);

    // 2. Mock processing loop
    let mut state: u64 = 0;
    for i in 1..=5 {
        state = state.wrapping_add((r as u64).wrapping_mul(i));
        println!("Computing step {} -> State: {}", i, state);
    }

    // 3. Environment time grab
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    println!("Epoch Time Measured: {}", time);

    println!("--- Helio Deterministic Guest Worker complete ---");
    println!("FINAL_STATE_HASH: {:016x}", state);
}
