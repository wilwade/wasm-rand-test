# Testing WASM vs Rust rand::SmallRng


1. `cargo install wasm-pack`
1. `wasm-pack build --target web`
1. Start a server: `npx serve ./`
1. `open http://localhost:3000`
1. See that the output values match the expected values in the test in `src/lib.rs`
1. Run `cargo test`
1. Fail
