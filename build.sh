#!/bin/bash

set -e

echo "INFO: Compiling project for: 'wasm32-unknown-unknown...'"
cargo build --release --target wasm32-unknown-unknown
echo "INFO: Moving .wasm file here..."
rm spellstrikeweb/static/spellstrike.wasm
mv target/wasm32-unknown-unknown/release/spellstrike.wasm spellstrikeweb/static
cd spellstrikeweb
cargo run --release


