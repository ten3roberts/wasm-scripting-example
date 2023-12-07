set -e

pushd ./single_component/
./build.sh
popd

wasm-tools parse ./core_guest.wast -o bin/core_guest.wasm
