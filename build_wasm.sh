#!/bin/bash
set -e

echo "Building maratha-war-of-independence for wasm32-unknown-unknown..."
cargo build --target wasm32-unknown-unknown --release

echo "Copying binary to web/..."
cp target/wasm32-unknown-unknown/release/maratha-war-of-independence.wasm web/game_engine.wasm
cp target/wasm32-unknown-unknown/release/maratha-war-of-independence.wasm web/maratha_war_of_independence.wasm

echo "Build complete! Artifact size:"
ls -lh web/game_engine.wasm
