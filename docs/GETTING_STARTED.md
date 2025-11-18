# Getting Started with Origin

Welcome to Origin! This guide will help you get started with developing UI applications that run at 120fps across all platforms.

## Prerequisites

- Rust 1.70 or later
- `just` command runner (optional but recommended)
  ```bash
  cargo install just
  ```

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Genuineh/Origin.git
   cd Origin
   ```

2. **Build the project**:
   ```bash
   just build
   # or
   cargo build --release
   ```

3. **Run tests**:
   ```bash
   just test
   # or
   cargo test --all
   ```

## Quick Start

### Method 1: Using Design Files (Coming Soon)

```bash
# Convert a Figma file
origin convert design.fig -o app.origin

# Run the app
origin run app.origin

# Hot reload from Figma
origin watch https://figma.com/file/YOUR_FILE_ID
```

### Method 2: Using Origin DSL (Coming Soon)

Create a new file `src/main.rs`:

```rust
use origin::prelude::*;

#[origin::main]
fn app() {
    column! {
        spacing: 20,
        padding: 40,
        
        text!("Hello, Origin!", size: 32),
        
        rect! {
            width: 200,
            height: 100,
            radii: 16,
            fill: gradient!(#FF6B6B, #4ECDC4),
        },
        
        button!("Click Me", on_click: || {
            println!("Button clicked!");
        }),
    }
}
```

Run your app:
```bash
cargo run
```

## Project Structure

```
Origin/
├── crates/                  # All Rust crates
│   ├── origin/             # Main crate, re-exports everything
│   ├── origin-platform/    # Platform abstraction layer
│   ├── origin-render/      # Rendering core (wgpu + mega-shader)
│   ├── origin-cache/       # Optimization and caching
│   ├── origin-primitive/   # 6 core primitives
│   ├── origin-layout/      # Constraint-based layout engine
│   ├── origin-tag/         # Tag system for interactions
│   ├── origin-input/       # Cross-platform input handling
│   ├── origin-ui/          # DSL macros and builders
│   └── design-adapters/    # Design tool adapters
│       ├── adapter-core/   # Common adapter infrastructure
│       ├── figma/         # Figma adapter (OriginFigma)
│       ├── sketch/        # Sketch adapter (future)
│       └── xd/            # Adobe XD adapter (future)
├── docs/                   # Documentation
├── examples/              # Example applications
├── scripts/              # Build scripts
├── justfile              # Build commands
└── README.md
```

## Core Concepts

### The 15 Iron Rules

Origin is built on 15 fundamental principles (see [README.md](../README.md) for details):

1. No trees
2. One-frame regeneration
3. Simple state (f32/u32/bool only)
4. No native controls
5. Pixel perfect (≤ 1px)
... and 10 more

### 7+1 Layer Architecture

Origin's architecture consists of 8 layers:

| Layer | Purpose |
|-------|---------|
| 8 | Design Adapters (Figma, Sketch, etc. → .origin) |
| 7 | Origin DSL (Hand-written UI) |
| 6 | Tag / Picker / Input |
| 5 | Layout Engine |
| 4 | Primitive Layer |
| 3 | Cache & Optimizer |
| 2 | Render Core |
| 1 | Platform Abstraction |
| 0 | Build & Deploy |

See [ARCHITECTURE.md](../ARCHITECTURE.md) for details.

## Building for Different Platforms

### Desktop (Current Platform)
```bash
just build
```

### WebAssembly
```bash
just web
```

### Android
```bash
just android
```

### iOS
```bash
just ios
```

### All Platforms
```bash
just all
```

## Development Workflow

1. **Make changes** to the code
2. **Run tests**: `just test`
3. **Run linter**: `just lint`
4. **Format code**: `just fmt`
5. **Build**: `just build`
6. **Run**: `just dev`

## Common Commands

```bash
# Development
just dev              # Run in development mode
just watch            # Watch and rebuild on changes
just check            # Quick syntax check

# Building
just build            # Build release binary
just build-all        # Build all crates
just web              # Build for web
just android          # Build for Android
just ios              # Build for iOS

# Testing
just test             # Run tests
just bench            # Run benchmarks

# Maintenance
just fmt              # Format code
just lint             # Run clippy
just clean            # Clean build artifacts
just docs             # Generate documentation
```

## Examples

(Coming soon - examples will be added as development progresses)

## Next Steps

- Read [ARCHITECTURE.md](../ARCHITECTURE.md) to understand the system
- Check [TODO.md](../TODO.md) for development progress
- See [CONTRIBUTING.md](../CONTRIBUTING.md) to contribute
- Explore [docs/DESIGN_ADAPTERS.md](DESIGN_ADAPTERS.md) for design tool integration

## Getting Help

- **Documentation**: Check the `docs/` directory
- **GitHub Issues**: Report bugs or request features
- **GitHub Discussions**: Ask questions

## Current Status

⚠️ **Note**: Origin is in early development (Phase 0: Foundation).

Currently implemented:
- [x] Project structure
- [x] Basic crate organization
- [x] Documentation framework
- [ ] Core rendering (Phase 1)
- [ ] Layout engine (Phase 2)
- [ ] Design adapters (Phase 6)

See [TODO.md](../TODO.md) for the complete roadmap.

---

**Origin — Where design returns to its origin.** ⚡
