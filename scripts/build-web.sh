#!/bin/bash
# Build Origin for WebAssembly

set -e

echo "Building Origin for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build with wasm-pack
wasm-pack build --target web --out-dir dist/web

echo "WebAssembly build complete!"
echo "Output: dist/web/"
