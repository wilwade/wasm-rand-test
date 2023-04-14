use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_random_values() -> Vec<u32> {
    // Create a SmallRng instance with a fixed seed.
    let mut rng = SmallRng::seed_from_u64(42);

    // Generate 10 random values.
    let mut wasm_values = Vec::new();
    for _ in 0..10 {
        wasm_values.push(rng.gen_range(1..100));
    }
    return wasm_values;
}

#[test]
fn test_values_match_wasm() {
    assert_eq!(
        test_random_values(),
        vec![34, 36, 2, 68, 82, 14, 23, 47, 82, 67]
    );
}
