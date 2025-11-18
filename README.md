# Origin — Where design returns to its origin. ⚡

**Origin** is a revolutionary UI runtime that transforms design files into 120fps, pixel-perfect, interactive native apps across all platforms.

**One Rust binary + One design file = Native App everywhere**

Platforms: Web • Android • iOS • Windows • macOS • Linux

## 🎯 The 15 Iron Rules

These rules are the foundation of Origin. They cannot be broken:

1. **No Trees Allowed**: The entire codebase is prohibited from using Widget/Element/Node/Tree terminology
2. **One-Frame Regeneration**: All UI must be capable of complete regeneration within a single frame
3. **Simple State**: State can only be f32/u32/bool, ultimately packed into Uniforms
4. **No Native Controls**: No platform-native UI controls are permitted
5. **Pixel Perfect**: Cross-platform pixel error must be ≤ 1px
6. **Fast Loading**: Design files must load in < 50ms regardless of complexity
7. **Instant Web**: Web cold start must be < 100ms
8. **Memory Efficient**: Peak memory target < 60MB
9. **Pure Shader Effects**: All visual effects must be implementable with SDF + shaders
10. **Mathematical Layout**: All layout must use mathematical equations, no trees
11. **u64 Tags**: Tags must use u64 (high 32 bits for type, low 32 bits for instance ID)
12. **No External UI Libraries**: Third-party UI libraries (egui, iced, etc.) are forbidden
13. **Single wgpu Instance**: All platforms must share a single wgpu::Instance
14. **Small Binary**: Final binary size must be < 12MB (release + strip)
15. **The Mission**: Origin — Where design returns to its origin.

## 🏗️ Architecture: 7+1 Layers

| Layer | Name | Purpose |
|-------|------|---------|
| 8 | Design Adapters | Transform design files (Figma, Sketch, etc.) → .origin binary |
| 7 | Origin DSL | Hand-written UI syntax (Makepad-inspired, but purer) |
| 6 | Tag / Picker / Input | u64 Tag + RGBA32UI Pick Buffer + unified cross-platform input |
| 5 | Layout Engine | Runtime constraint solver + relative coordinate stack |
| 4 | Primitive Layer | 6 core primitives via mega-shader + instance buffers |
| 3 | Cache & Optimizer | Per-instance StateHash → sub-buffer updates + draw call merging |
| 2 | Render Core | Pure wgpu 0.21 + single mega.wgsl shader |
| 1 | Platform Abstraction | winit + 6 platform entry points (< 600 lines total) |
| 0 | Build & Deploy | justfile one-command builds for all platforms |

## 🎨 Design Platform Adapters

Origin's unique approach treats design tools as **source platforms**:

### Currently Supported
- **OriginFigma**: Figma files → .origin binary (120fps runtime, pixel-perfect)

### Planned Adapters
- **OriginSketch**: Sketch files support
- **OriginXD**: Adobe XD support  
- **OriginPenpot**: Open-source Penpot support
- Other design tools can be added via the adapter interface

### Design Adapter Architecture
Each adapter follows a common pipeline:
```
Design File → Parser → IR (Intermediate Representation) → 
Constraint Solver → Instance Buffer → .origin binary
```

All adapters share:
- Common primitive mapping layer
- Unified constraint system
- Shared texture atlas generation
- Standard component/variant system

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/Genuineh/Origin.git
cd Origin

# Build for your platform
just build

# Run a design file
origin run app.origin

# Watch and hot-reload from Figma
origin watch figma.com/file/your-design
```

## 🚀 Quick Start

### Using Design Files
```bash
# Convert Figma file to .origin
origin convert design.fig -o app.origin

# Run the app
origin run app.origin

# Hot reload from design tool
origin watch https://www.figma.com/file/YOUR_FILE_ID
```

### Using Origin DSL (for developers)
```rust
use origin::prelude::*;

#[origin::main]
fn app() {
    column!(
        spacing: 20,
        padding: 40,
        
        // Rounded rectangle with gradient
        rect!(
            width: 300,
            height: 200,
            radii: [16, 16, 16, 16],
            fill: gradient!(#FF6B6B, #4ECDC4),
        ),
        
        // Text with custom styling
        text!(
            "Hello, Origin!",
            font_size: 24,
            color: #2C3E50,
        ),
        
        // Interactive button
        button!(
            "Click Me",
            on_click: || println!("Clicked!"),
        ),
    )
}
```

## 📋 Development Status

See [TODO.md](./TODO.md) for the detailed development roadmap.

Current Phase: **Phase 0 - Project Initialization** ✅

## 📚 Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Complete system architecture
- [TODO.md](./TODO.md) - Phased development plan
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [docs/](./docs/) - Detailed documentation for each layer

## 🎯 Project Goals

- **120fps**: Butter-smooth performance on all platforms
- **Pixel Perfect**: Design files render exactly as intended
- **Universal**: One codebase, six platforms (Web/Android/iOS/Windows/macOS/Linux)
- **Instant**: Sub-50ms load times, sub-100ms web cold start
- **Lightweight**: < 12MB binary, < 60MB memory
- **Designer-Friendly**: Direct design tool integration with hot reload

## 🤝 Contributing

Origin is an ambitious project that welcomes contributions. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🌟 Project Vision

Origin aims to revolutionize UI development by:

1. **Eliminating the design-to-code gap**: Designers work in their tools, developers see instant results
2. **Achieving true cross-platform**: Same binary, same visuals, everywhere
3. **Delivering uncompromising performance**: 120fps is the baseline, not the goal
4. **Maintaining simplicity**: No trees, no retained mode, pure immediate rendering

**Target Launch**: April 30, 2026

---

*Origin — Where design returns to its origin.* ⚡
