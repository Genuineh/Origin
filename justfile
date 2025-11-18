# Origin Build System
# One-command builds for all platforms

# Default recipe - show available commands
default:
    @just --list

# Build for current platform
build:
    cargo build --release

# Build all crates
build-all:
    cargo build --all --release

# Run tests
test:
    cargo test --all

# Run benchmarks
bench:
    cargo bench --all

# Check code without building
check:
    cargo check --all

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --all -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Build for WebAssembly
web:
    @echo "Building for WebAssembly..."
    @bash scripts/build-web.sh

# Build for Android
android:
    @echo "Building for Android..."
    @bash scripts/build-android.sh

# Build for iOS
ios:
    @echo "Building for iOS..."
    @bash scripts/build-ios.sh

# Build for all desktop platforms
desktop:
    @echo "Building for desktop..."
    cargo build --release

# Build for Windows
windows:
    @echo "Building for Windows..."
    cargo build --release --target x86_64-pc-windows-msvc

# Build for macOS
macos:
    @echo "Building for macOS..."
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin

# Build for Linux
linux:
    @echo "Building for Linux..."
    cargo build --release --target x86_64-unknown-linux-gnu

# Build for all platforms
all: web android ios desktop

# Run in development mode
dev:
    cargo run

# Watch and rebuild on changes
watch:
    cargo watch -x run

# Generate documentation
docs:
    cargo doc --all --no-deps --open

# Install development dependencies
install-deps:
    @echo "Installing development dependencies..."
    cargo install cargo-watch
    cargo install wasm-pack
    @echo "Done!"

# Check binary size
size:
    @echo "Checking binary size..."
    @ls -lh target/release/origin 2>/dev/null || echo "Binary not built yet. Run 'just build' first."

# Profile the application
profile:
    cargo build --release
    @echo "Profiling not yet implemented"

# Run with release optimizations
run-release:
    cargo run --release

# Update dependencies
update:
    cargo update

# Audit dependencies for security issues
audit:
    cargo audit
