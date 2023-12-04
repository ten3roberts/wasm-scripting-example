set -e
cargo build --target wasm32-unknown-unknown
wasm-tools component new ../../target/wasm32-unknown-unknown/debug/test_single_component.wasm -o component.wasm
