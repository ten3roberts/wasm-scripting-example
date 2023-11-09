mkdir -p bin

cargo build --target wasm32-unknown-unknown -p guest
wasm-tools component new ./target/wasm32-unknown-unknown/debug/guest.wasm -o guest.wasm
