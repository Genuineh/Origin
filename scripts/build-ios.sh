#!/bin/bash
# Build Origin for iOS

set -e

echo "Building Origin for iOS..."

# Build for iOS targets
echo "Building for iOS targets..."
cargo build --release --target aarch64-apple-ios
cargo build --release --target x86_64-apple-ios

echo "iOS build complete!"
echo "IPA packaging not yet implemented"
