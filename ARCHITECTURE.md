# Origin Architecture

This document provides a comprehensive overview of Origin's architecture, design decisions, and implementation details.

## Table of Contents

1. [Core Philosophy](#core-philosophy)
2. [7+1 Layer Architecture](#71-layer-architecture)
3. [Design Platform Adapters](#design-platform-adapters)
4. [Rendering Pipeline](#rendering-pipeline)
5. [Memory Model](#memory-model)
6. [Performance Strategy](#performance-strategy)
7. [Cross-Platform Strategy](#cross-platform-strategy)

---

## Core Philosophy

### No Retained Mode

Origin fundamentally rejects the traditional UI tree/retained mode approach. Instead:

- **Immediate Mode**: UI is regenerated every frame from state
- **Pure Functions**: UI = f(state), no mutable tree structures
- **Instance Rendering**: Everything is an instance in a flat buffer
- **Mathematical Layout**: Constraints solved at runtime, no layout tree

### Why No Trees?

1. **Simplicity**: No tree traversal, no node lifecycle management
2. **Performance**: Direct instance buffer updates, no tree diffing
3. **Predictability**: Same state = same output, always
4. **Parallelism**: Flat data structures enable easy parallelization
5. **Memory**: No pointer chasing, better cache locality

### State Model

```rust
// State is simple values
struct AppState {
    counter: u32,
    scroll_offset: f32,
    is_hovered: bool,
}

// UI generation is a pure function
fn render(state: &AppState) -> InstanceBuffer {
    // Generate instances directly
    let instances = vec![
        rect_instance(state.counter as f32, ...),
        text_instance(state.scroll_offset, ...),
    ];
    instances.into()
}
```

---

## 7+1 Layer Architecture

### Layer 0: Build & Deploy

**Purpose**: One-command build system for all platforms

**Components**:
- `justfile`: Command recipes for all build operations
- `scripts/`: Platform-specific build scripts
- `.github/workflows/`: CI/CD automation

**Key Files**:
- `justfile`: Main build commands
- `scripts/build-web.sh`: WebAssembly build
- `scripts/build-android.sh`: Android APK/AAB
- `scripts/build-ios.sh`: iOS IPA
- `scripts/build-desktop.sh`: Windows/macOS/Linux binaries

**Commands**:
```bash
just build              # Build for current platform
just web                # Build WebAssembly
just android            # Build Android APK
just ios                # Build iOS IPA
just desktop            # Build desktop binary
just test               # Run all tests
just bench              # Run benchmarks
```

---

### Layer 1: Platform Abstraction

**Location**: `crates/origin-platform/`

**Purpose**: Unified entry point for all platforms with minimal platform-specific code

**Architecture**:
```
origin-platform/
├── src/
│   ├── lib.rs           # Common traits and types
│   ├── web.rs           # WebAssembly entry point
│   ├── android.rs       # Android entry point
│   ├── ios.rs           # iOS entry point
│   ├── windows.rs       # Windows entry point
│   ├── macos.rs         # macOS entry point
│   └── linux.rs         # Linux entry point
```

**Responsibilities**:
1. Window creation and management (via winit)
2. Event loop handling
3. Surface creation for wgpu
4. Platform-specific input normalization
5. File system access (where applicable)

**Key Constraint**: < 600 lines total, zero platform-specific rendering code

**Example**:
```rust
// All platforms use the same interface
pub trait Platform {
    fn create_surface(&self) -> wgpu::Surface;
    fn run_event_loop(&mut self, app: App);
    fn handle_input(&mut self, event: Event);
}
```

---

### Layer 2: Render Core

**Location**: `crates/origin-render/`

**Purpose**: Pure wgpu rendering with single mega-shader

**Architecture**:
```
origin-render/
├── src/
│   ├── lib.rs           # Main render API
│   ├── gpu.rs           # wgpu context management
│   ├── instance.rs      # Instance buffer management
│   ├── uniform.rs       # Uniform buffer system
│   ├── texture.rs       # Texture atlas management
│   └── pipeline.rs      # Render pipeline setup
├── shaders/
│   └── mega.wgsl        # Single mega-shader
```

**Key Concepts**:

1. **Single Mega-Shader**: One shader handles all primitive types
   ```wgsl
   struct Instance {
       transform: mat4x4<f32>,
       color: vec4<f32>,
       primitive_type: u32,  // 0=rect, 1=circle, 2=line, etc.
       params: vec4<f32>,    // Type-specific parameters
   }
   ```

2. **Instance-Only Rendering**: No per-primitive shaders
   - All primitives are instances in a single buffer
   - Shader switches behavior based on `primitive_type`
   - Minimal draw calls (ideally 1 per frame)

3. **Full-Screen Triangle Technique**:
   ```rust
   // Vertex shader generates full-screen triangle
   // Fragment shader does all the work
   vertices: [(0,0), (2,0), (0,2)]  // Covers entire screen
   ```

**Rendering Pipeline**:
```
State → Instances → Instance Buffer → GPU → Mega-Shader → Frame
```

---

### Layer 3: Cache & Optimizer

**Location**: `crates/origin-cache/`

**Purpose**: Minimize GPU traffic through intelligent caching

**Key Techniques**:

1. **State Hashing**:
   ```rust
   struct StateHash(u64);
   
   impl Instance {
       fn compute_hash(&self) -> StateHash {
           // Hash all visual properties
           hash(self.transform, self.color, self.params)
       }
   }
   ```

2. **Differential Updates**:
   ```rust
   // Only update changed instances
   for (index, instance) in new_instances.iter().enumerate() {
       if instance.hash() != old_hashes[index] {
           buffer.update_range(index, instance);
       }
   }
   ```

3. **Draw Call Merging**:
   - Group consecutive instances of same type
   - Single draw call for entire groups
   - Target: 1-3 draw calls per frame

4. **Dirty Rectangle Optimization**:
   ```rust
   // Only re-render changed regions
   let dirty_rect = compute_dirty_rect(old_instances, new_instances);
   encoder.set_scissor_rect(dirty_rect);
   ```

**Performance Targets**:
- 100k instances: < 2ms CPU per frame
- State hash: < 0.5ms
- Buffer update: < 1ms
- 120fps maintained

---

### Layer 4: Primitive Layer

**Location**: `crates/origin-primitive/`

**Purpose**: The 6 core visual primitives, all SDF-based

**The 6 Primitives**:

1. **RoundedRect**:
   ```rust
   struct RoundedRect {
       position: Vec2,
       size: Vec2,
       radii: [f32; 4],  // Top-left, top-right, bottom-right, bottom-left
       fill: Color,
       stroke: Option<Stroke>,
   }
   ```
   - SDF for perfect anti-aliasing
   - Independent corner radii
   - Gradient fills supported

2. **Circle**:
   ```rust
   struct Circle {
       center: Vec2,
       radius: f32,
       fill: Color,
       stroke: Option<Stroke>,
   }
   ```
   - Pure SDF implementation
   - Sub-pixel accuracy

3. **Line**:
   ```rust
   struct Line {
       start: Vec2,
       end: Vec2,
       width: f32,
       cap: LineCap,  // Butt, Round, Square
       color: Color,
   }
   ```
   - SDF-based width
   - Various cap styles

4. **Arc**:
   ```rust
   struct Arc {
       center: Vec2,
       radius: f32,
       start_angle: f32,
       end_angle: f32,
       width: f32,
       color: Color,
   }
   ```
   - For circular progress indicators
   - SDF anti-aliasing

5. **Path**:
   ```rust
   struct Path {
       commands: Vec<PathCommand>,
       fill_rule: FillRule,
       fill: Color,
       stroke: Option<Stroke>,
   }
   
   enum PathCommand {
       MoveTo(Vec2),
       LineTo(Vec2),
       CurveTo(Vec2, Vec2, Vec2),
       Close,
   }
   ```
   - Boolean operations (union, subtract, intersect)
   - SDF composition tree
   - Complex shapes from primitives

6. **Text**:
   ```rust
   struct Text {
       content: String,
       position: Vec2,
       font: FontId,
       size: f32,
       color: Color,
   }
   ```
   - cosmic-text for layout
   - MSDF (Multi-channel Signed Distance Field) rendering
   - Support for CJK, Arabic, complex scripts
   - Subpixel positioning

**SDF Benefits**:
- Perfect anti-aliasing at any scale
- Effects (shadow, glow, outline) are shader operations
- Resolution-independent
- GPU-friendly

---

### Layer 5: Layout Engine

**Location**: `crates/origin-layout/`

**Purpose**: Mathematical constraint-based layout, no trees

**Core Concept**: Layout as Constraint Solving

```rust
// Layout is a set of constraints
struct Constraint {
    left: ConstraintExpr,
    relation: ConstraintRelation,  // ==, <=, >=
    right: ConstraintExpr,
    priority: f32,
}

enum ConstraintExpr {
    Constant(f32),
    Variable(VarId),
    Add(Box<ConstraintExpr>, Box<ConstraintExpr>),
    Multiply(Box<ConstraintExpr>, f32),
}
```

**Layout Primitives**:

1. **Column**:
   ```rust
   fn column(items: &[Item], spacing: f32) -> Vec<Constraint> {
       let mut constraints = vec![];
       for i in 1..items.len() {
           constraints.push(
               items[i].top == items[i-1].bottom + spacing
           );
       }
       constraints
   }
   ```

2. **Row**:
   ```rust
   fn row(items: &[Item], spacing: f32) -> Vec<Constraint> {
       let mut constraints = vec![];
       for i in 1..items.len() {
           constraints.push(
               items[i].left == items[i-1].right + spacing
           );
       }
       constraints
   }
   ```

3. **Grid**:
   ```rust
   fn grid(items: &[Item], cols: usize, spacing: f32) -> Vec<Constraint> {
       // Generate row and column constraints
       // Equal-width columns, equal-height rows
   }
   ```

4. **Constraint** (free-form):
   ```rust
   // Like iOS AutoLayout
   button.centerX == parent.centerX
   button.top == label.bottom + 20
   button.width == parent.width * 0.8
   ```

**Solver**: Gauss-Seidel Iterative Method
- Converges in ~5-10 iterations for typical UIs
- < 1ms for complex layouts
- Handles over-constrained systems gracefully

**Relative Coordinate Stack**:
```rust
// Parent-relative positioning
struct CoordinateFrame {
    origin: Vec2,
    size: Vec2,
}

// Transform child coords to parent coords
fn to_parent(child_pos: Vec2, frame: &CoordinateFrame) -> Vec2 {
    frame.origin + child_pos
}
```

---

### Layer 6: Tag / Picker / Input

**Location**: `crates/origin-tag/`, `crates/origin-input/`

**Purpose**: Interaction system without tree traversal

**Tag System**:
```rust
// u64 tag: high 32 bits = type, low 32 bits = instance ID
type Tag = u64;

fn make_tag(type_id: u32, instance_id: u32) -> Tag {
    ((type_id as u64) << 32) | (instance_id as u64)
}

fn extract_type(tag: Tag) -> u32 {
    (tag >> 32) as u32
}

fn extract_id(tag: Tag) -> u32 {
    tag as u32
}
```

**Pick Buffer**:
```rust
// RGBA32UI texture, each pixel stores a tag
struct PickBuffer {
    texture: wgpu::Texture,
    size: (u32, u32),
}

// Render pass that writes tags instead of colors
fn render_pick_buffer(instances: &[Instance]) {
    // Fragment shader outputs instance.tag to pick buffer
}

// Mouse click → read pixel from pick buffer → get tag
fn pick_at(x: u32, y: u32) -> Tag {
    pick_buffer.read_pixel(x, y)
}
```

**Input Handling**:
```rust
pub enum InputEvent {
    MouseMove { x: f32, y: f32 },
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseUp { button: MouseButton, x: f32, y: f32 },
    Scroll { delta_x: f32, delta_y: f32 },
    KeyDown { key: Key, modifiers: Modifiers },
    KeyUp { key: Key, modifiers: Modifiers },
    Touch { id: u64, phase: TouchPhase, x: f32, y: f32 },
}

// Normalized across platforms
pub struct InputState {
    mouse_pos: Vec2,
    pressed_buttons: HashSet<MouseButton>,
    hovered_tag: Option<Tag>,
    pressed_tag: Option<Tag>,
    keyboard_modifiers: Modifiers,
}
```

**Interaction Flow**:
```
1. User moves mouse → update input state
2. Query pick buffer at mouse position → get tag
3. Tag changed? → trigger hover enter/exit events
4. User clicks → mouse down on tag
5. Dispatch event to callback associated with tag
```

---

### Layer 7: Origin DSL

**Location**: `crates/origin-ui/`

**Purpose**: Hand-written UI with Rust macros

**Design Goals**:
- Readable, declarative syntax
- Type-safe
- Compile-time checks
- No runtime overhead

**Syntax Examples**:

```rust
use origin::prelude::*;

// Simple rectangle
rect! {
    width: 100,
    height: 100,
    radii: 10,
    fill: #FF6B6B,
}

// Column layout
column! {
    spacing: 20,
    padding: 40,
    align: Center,
    
    text!("Title", size: 24, color: #333333),
    text!("Subtitle", size: 16, color: #666666),
    
    row! {
        spacing: 10,
        
        button!("Cancel", on_click: |_| cancel()),
        button!("OK", on_click: |_| confirm()),
    }
}

// State-driven UI
fn counter_ui(count: u32) -> Element {
    column! {
        text!(format!("Count: {}", count)),
        button!("+1", on_click: |_| count += 1),
    }
}
```

**Macro Expansion**:
```rust
// rect! expands to:
rect! { width: 100, height: 100 }
// becomes:
Instance::rect(
    RectParams {
        width: 100.0,
        height: 100.0,
        ..Default::default()
    }
)
```

**Component System**:
```rust
// Define reusable components
fn_component! {
    Card(title: &str, content: &str) {
        column! {
            spacing: 10,
            padding: 20,
            background: #FFFFFF,
            border_radius: 8,
            
            text!(title, weight: Bold, size: 18),
            text!(content, size: 14, color: #666666),
        }
    }
}

// Use components
Card("Hello", "World")
```

---

### Layer 8: Design Platform Adapters

**Location**: `crates/design-adapters/`

**Purpose**: Transform design files into .origin binaries

**Architecture**:
```
design-adapters/
├── adapter-core/        # Common infrastructure
│   ├── src/
│   │   ├── ir.rs        # Intermediate Representation
│   │   ├── primitive_mapper.rs
│   │   ├── constraint_generator.rs
│   │   ├── atlas_builder.rs
│   │   └── binary_writer.rs
├── figma/              # Figma adapter (OriginFigma)
│   ├── src/
│   │   ├── parser.rs
│   │   ├── api.rs
│   │   └── converter.rs
│   └── plugin/         # Figma plugin (TypeScript)
├── sketch/             # Future: Sketch adapter
├── xd/                 # Future: Adobe XD adapter
└── penpot/            # Future: Penpot adapter
```

**Common Pipeline**:
```
Design File → Parser → IR → Primitive Mapper → 
Constraint Generator → Atlas Builder → .origin Binary
```

**Intermediate Representation (IR)**:
```rust
struct OriginIR {
    version: u32,
    artboards: Vec<Artboard>,
    components: HashMap<String, Component>,
    assets: Assets,
}

struct Artboard {
    id: String,
    name: String,
    size: Vec2,
    background: Color,
    nodes: Vec<Node>,
}

struct Node {
    id: String,
    transform: Transform,
    visual: VisualProperties,
    layout: LayoutProperties,
    interactions: Vec<Interaction>,
}
```

**Adapter Trait**:
```rust
pub trait DesignAdapter {
    fn parse(&self, file: &Path) -> Result<OriginIR>;
    fn supports_hot_reload(&self) -> bool;
    fn watch(&self, url: &str) -> Result<HotReloadStream>;
}
```

---

#### OriginFigma (Primary Adapter)

**Features**:
- Full Figma feature support
- Pixel-perfect rendering
- Auto Layout → constraints
- Components & variants
- Prototype interactions
- Hot reload via plugin

**File Format: .origin**
```rust
// Custom binary format
struct OriginFile {
    magic: [u8; 4],        // "ORGN"
    version: u32,
    
    // Embedded assets
    fonts: Vec<FontData>,
    atlas: TextureAtlas,
    
    // Flat instance list (no tree!)
    instances: Vec<SerializedInstance>,
    
    // Layout constraints
    constraints: Vec<SerializedConstraint>,
    
    // Components
    components: HashMap<u64, ComponentTemplate>,
    variants: HashMap<u64, VariantTable>,
    
    // Interactions
    interactions: Vec<Interaction>,
    
    // Entry point
    entry_artboard: u64,
}
```

**Figma Mapping**:

| Figma Feature | Origin Primitive | Notes |
|---------------|------------------|-------|
| Rectangle | RoundedRect | With radii |
| Ellipse | Circle | Or ellipse SDF |
| Line | Line | With caps |
| Vector | Path | SDF boolean ops |
| Text | Text | cosmic-text + MSDF |
| Frame | Container | Generates constraints |
| Auto Layout | Column/Row | With constraints |
| Component | ComponentTemplate | Instantiable |
| Variant | VariantTable | State switch |
| Prototype | Interaction | Click → transition |

**Hot Reload Flow**:
```
1. Designer edits in Figma
2. Figma plugin detects changes
3. Plugin exports to WebSocket server
4. Origin app watches WebSocket
5. Receives update → parses → hot reloads
6. < 100ms from save to screen update
```

**Plugin Code** (TypeScript):
```typescript
// Figma plugin (simplified)
figma.on('documentchange', async () => {
  const data = exportToOriginIR(figma.root);
  await sendToOriginRuntime(data);
});
```

---

#### Future Adapters

**OriginSketch**:
- Parse .sketch files (JSON-based)
- Similar primitive mapping
- Symbol → Component mapping

**OriginXD**:
- Adobe XD file format support
- Repeat Grid → Grid layout
- Component States → Variants

**OriginPenpot**:
- Open-source design tool
- Native SVG-based format
- Community-friendly

**Adapter Development Guide**:
```rust
// Implement the DesignAdapter trait
struct MyAdapter;

impl DesignAdapter for MyAdapter {
    fn parse(&self, file: &Path) -> Result<OriginIR> {
        // 1. Parse design file format
        // 2. Convert to OriginIR
        // 3. Return IR
    }
}
```

---

## Rendering Pipeline

### Frame Rendering Flow

```
┌─────────────────┐
│   App State     │ (f32, u32, bool)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Generate UI     │ (Pure function)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Solve Layout    │ (Constraint solver)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Build Instances │ (Flat buffer)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Compute Hashes  │ (Detect changes)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Update GPU      │ (Partial buffer update)
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Render Frame    │ (Mega-shader)
└─────────────────┘
```

### GPU Frame Pipeline

```
1. Clear backbuffer
2. Bind mega pipeline
3. Bind instance buffer
4. Bind uniform buffer
5. Bind texture atlas
6. Draw(instance_count, 1)  ← Single draw call!
7. Present
```

---

## Memory Model

### Zero-Copy Philosophy

All data is designed for direct GPU upload:

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuInstance {
    transform: [f32; 16],   // mat4
    color: [f32; 4],        // rgba
    primitive_type: u32,
    params: [f32; 4],       // type-specific
    tag: u64,               // for picking
}
```

### Memory Budget

- **Instance Buffer**: ~16MB (1M instances × 64 bytes)
- **Texture Atlas**: ~8MB (2048×2048 RGBA)
- **Uniform Buffers**: ~1MB
- **Shader Code**: ~500KB
- **Font Data**: ~10MB (subset)
- **Application Code**: ~20MB
- **Total**: < 60MB peak

### Memory Techniques

1. **No Allocations in Hot Path**:
   ```rust
   // Pre-allocated instance buffer
   let mut instances = Vec::with_capacity(100_000);
   
   // Reuse every frame
   instances.clear();
   generate_ui(&mut instances);
   ```

2. **Arena Allocators for Temporary Data**:
   ```rust
   let arena = Arena::new();
   let temp_data = arena.alloc(size);
   // ... use temp_data ...
   // arena.reset() at frame end
   ```

3. **Texture Atlas**: Pack all glyphs and images into single texture
   - Reduces texture switches
   - Better GPU cache utilization

---

## Performance Strategy

### Target Metrics

- **Frame Time**: 8.33ms (120fps)
  - Layout: < 1ms
  - Instance generation: < 1ms
  - GPU update: < 1ms
  - Rendering: < 3ms
  - Margin: 2.33ms

- **Startup Time**:
  - Desktop: < 50ms cold start
  - Web: < 100ms cold start
  - Mobile: < 200ms cold start

### Optimization Techniques

1. **Dirty Tracking**:
   ```rust
   // Only recompute changed parts
   if state.hash() == last_state_hash {
       return;  // Skip frame
   }
   ```

2. **Parallel Instance Generation**:
   ```rust
   // Use rayon for parallel UI generation
   instances.par_chunks_mut(chunk_size)
       .for_each(|chunk| generate_chunk(chunk));
   ```

3. **GPU Upload Optimization**:
   ```rust
   // Only update changed ranges
   buffer.write_range(
       changed_start..changed_end,
       &instances[changed_start..changed_end]
   );
   ```

4. **Shader Optimization**:
   - Early discard for out-of-bounds fragments
   - Precompute values in vertex shader
   - Use cheaper distance functions where possible

### Profiling

Built-in profiler for all platforms:
```rust
#[cfg(feature = "profile")]
fn profile_section(name: &str) -> ProfileGuard {
    // Tracks time, reports to UI
}
```

---

## Cross-Platform Strategy

### Write Once, Run Everywhere

The key to cross-platform is **zero platform-specific rendering code**:

```rust
// Same code on all platforms
fn render_frame(state: &State) {
    let instances = generate_ui(state);
    renderer.update(instances);
    renderer.draw();
}
```

### Platform Differences (Handled in Layer 1)

1. **Window Creation**: winit abstracts this
2. **Event Handling**: Normalized to common enum
3. **File I/O**: Different on web vs native
4. **Clipboard**: Platform-specific APIs
5. **System Fonts**: Different paths/APIs

### Testing Strategy

1. **Visual Regression Tests**:
   - Render same UI on all platforms
   - Compare pixel-by-pixel
   - Threshold: ≤ 1px difference

2. **Performance Tests**:
   - Same benchmarks on all platforms
   - Ensure 120fps on target hardware

3. **Input Tests**:
   - Verify mouse, touch, keyboard on each platform

---

## Conclusion

Origin's architecture is designed around a few core principles:

1. **No Trees**: Flat instance buffers, immediate mode
2. **One Shader**: Mega-shader handles all primitives
3. **Pure Functions**: UI = f(state), predictable and simple
4. **Math over Trees**: Constraint-based layout
5. **Instance Everything**: Single draw call rendering
6. **Design Integration**: First-class design tool support

This architecture enables Origin to achieve its ambitious goals: 120fps, pixel-perfect, cross-platform UI from design files with a < 12MB binary.

**Origin — Where design returns to its origin.** ⚡
