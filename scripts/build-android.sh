#!/bin/bash
# Build Origin for Android

set -e

echo "Building Origin for Android..."

# Check for Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set"
    echo "Please set ANDROID_NDK_HOME to your Android NDK installation"
    exit 1
fi

# Build for Android targets
echo "Building for Android targets..."
cargo build --release --target aarch64-linux-android
cargo build --release --target armv7-linux-androideabi
cargo build --release --target x86_64-linux-android

echo "Android build complete!"
echo "APK/AAB packaging not yet implemented"
