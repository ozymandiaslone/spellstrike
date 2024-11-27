#!/bin/bash

set -e

echo "INFO: Compiling project for: 'wasm32-unknown-unknown...'"
cargo build --release --target wasm32-unknown-unknown
echo "INFO: Moving .wasm file here..."
mv target/wasm32-unknown-unknown/release/spellstrike.wasm .
echo "INFO: Serving .wasm with basic-http-server"
basic-http-server .

