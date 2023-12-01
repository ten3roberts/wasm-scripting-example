set -e

# cargo build --target wasm32-unknown-unknown

# echo "Running wasm-bindgen"
# wasm-bindgen ./target/wasm32-unknown-unknown/debug/scripting_example.wasm --keep-debug --target web --out-dir ./public/pkg
wasm-pack build --dev --target web --out-dir ./public/pkg
