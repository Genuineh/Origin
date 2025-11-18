# Design Platform Adapters

Origin's innovative approach treats design tools as **source platforms** rather than just asset exporters.

## Philosophy

Instead of asking designers to "export" their work, Origin directly consumes design files and renders them with pixel-perfect accuracy at 120fps. This eliminates the design-to-code gap entirely.

## Adapter Architecture

```
┌────────────────┐
│  Design Tool   │ (Figma, Sketch, XD, Penpot, etc.)
└────────┬───────┘
         │
         v
┌────────────────┐
│ Design Adapter │ (Tool-specific parser)
└────────┬───────┘
         │
         v
┌────────────────┐
│   Origin IR    │ (Platform-agnostic intermediate representation)
└────────┬───────┘
         │
         v
┌────────────────┐
│ Primitive      │ (Map to Origin's 6 primitives)
│ Mapper         │
└────────┬───────┘
         │
         v
┌────────────────┐
│ Constraint     │ (Generate layout constraints)
│ Generator      │
└────────┬───────┘
         │
         v
┌────────────────┐
│ Binary Writer  │ (Produce .origin file)
└────────┬───────┘
         │
         v
┌────────────────┐
│ Origin Runtime │ (120fps rendering on all platforms)
└────────────────┘
```

## Supported Design Tools

### ✅ OriginFigma (Primary)

**Status**: In development (Week 14-17)

**Features**:
- Full Figma API integration
- Pixel-perfect rendering (≤ 1px error)
- Auto Layout → constraint translation
- Components & variants support
- Prototype interactions
- Hot reload via Figma plugin

**Usage**:
```bash
# From local .fig file
origin convert design.fig -o app.origin

# From Figma URL
origin convert https://figma.com/file/YOUR_FILE_ID -o app.origin

# Hot reload
origin watch https://figma.com/file/YOUR_FILE_ID
```

**Feature Coverage**:

| Figma Feature | Support | Origin Mapping |
|---------------|---------|----------------|
| Rectangle | ✅ | RoundedRect |
| Ellipse | ✅ | Circle |
| Line | ✅ | Line |
| Vector paths | ✅ | Path + SDF |
| Text | ✅ | Text + MSDF |
| Auto Layout | ✅ | Column/Row + constraints |
| Components | ✅ | ComponentTemplate |
| Variants | ✅ | VariantTable |
| Prototypes | ✅ | Interaction |
| Boolean operations | ✅ | SDF combine |
| Effects (shadow, blur) | ✅ | Shader effects |
| Gradients | ✅ | Shader gradients |

### 🚧 OriginSketch (Planned)

**Status**: Post-launch (Q2 2026)

**Target Features**:
- .sketch file parsing
- Symbols → Components
- Shared styles
- Similar pixel-perfect accuracy

### 🚧 OriginXD (Planned)

**Status**: Post-launch (Q3 2026)

**Target Features**:
- Adobe XD file support
- Repeat Grid → Grid layout
- Component States → Variants
- Cloud document support

### 🚧 OriginPenpot (Planned)

**Status**: Post-launch (Q4 2026)

**Target Features**:
- Open-source design tool support
- Native SVG format
- Community-friendly integration

## Creating a Custom Adapter

Origin's adapter system is designed to be extensible. You can create adapters for any design tool.

### Step 1: Create Adapter Crate

```bash
cd crates/design-adapters
mkdir my-tool
cd my-tool
cargo init --lib
```

### Step 2: Implement the Trait

```rust
use origin_adapter_core::{
    AdapterError, AdapterSource, DesignAdapter,
    ir::OriginIR,
};

pub struct MyToolAdapter;

impl DesignAdapter for MyToolAdapter {
    fn parse(&self, source: AdapterSource) -> Result<OriginIR, AdapterError> {
        match source {
            AdapterSource::FilePath(path) => {
                // 1. Read and parse the design file
                let file_data = std::fs::read(&path)?;
                let design = parse_my_tool_format(&file_data)?;
                
                // 2. Convert to Origin IR
                let ir = convert_to_ir(&design)?;
                
                Ok(ir)
            }
            _ => Err(AdapterError::UnsupportedFeature(
                "Only file paths supported".to_string()
            )),
        }
    }
}

fn parse_my_tool_format(data: &[u8]) -> Result<MyToolDesign, AdapterError> {
    // Parse your tool's format (JSON, binary, etc.)
    todo!()
}

fn convert_to_ir(design: &MyToolDesign) -> Result<OriginIR, AdapterError> {
    // Map to Origin IR
    // See the IR types in origin-adapter-core/src/ir.rs
    todo!()
}
```

### Step 3: Map Primitives

```rust
use origin_adapter_core::ir::*;

fn map_rectangle(rect: &MyToolRect) -> Node {
    Node {
        id: rect.id.clone(),
        transform: Transform {
            position: rect.position,
            rotation: rect.rotation,
            scale: Vec2::ONE,
        },
        visual: VisualProperties {
            fill: Some(Fill::Solid(rect.color)),
            stroke: None,
            corner_radii: Some([rect.radius; 4]),
            effects: vec![],
            opacity: 1.0,
        },
        layout: LayoutProperties::default(),
        interactions: vec![],
    }
}
```

### Step 4: Generate Constraints

For layout containers (similar to Figma's Auto Layout):

```rust
fn map_container(container: &MyToolContainer) -> (Node, Vec<Node>) {
    let layout_mode = match container.direction {
        Direction::Vertical => LayoutMode::Column {
            spacing: container.spacing,
            align: Alignment::Start,
        },
        Direction::Horizontal => LayoutMode::Row {
            spacing: container.spacing,
            align: Alignment::Start,
        },
    };
    
    let parent = Node {
        // ... other fields
        layout: LayoutProperties {
            width: SizeConstraint::Hug,
            height: SizeConstraint::Hug,
            padding: container.padding,
            layout_mode: Some(layout_mode),
        },
        // ...
    };
    
    let children = container.children
        .iter()
        .map(|child| map_child(child))
        .collect();
    
    (parent, children)
}
```

### Step 5: Test Your Adapter

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_design() {
        let adapter = MyToolAdapter;
        let source = AdapterSource::FilePath("test.mytool".into());
        
        let ir = adapter.parse(source).unwrap();
        
        assert_eq!(ir.artboards.len(), 1);
        assert_eq!(ir.artboards[0].nodes.len(), 3);
    }
}
```

## Intermediate Representation (IR)

The Origin IR is a platform-agnostic format that all adapters target. Key components:

### Artboards
```rust
pub struct Artboard {
    pub id: String,
    pub name: String,
    pub size: Vec2,
    pub background: Color,
    pub nodes: Vec<Node>,
}
```

### Nodes
```rust
pub struct Node {
    pub id: String,
    pub transform: Transform,
    pub visual: VisualProperties,
    pub layout: LayoutProperties,
    pub interactions: Vec<Interaction>,
}
```

### Visual Properties
```rust
pub struct VisualProperties {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub corner_radii: Option<[f32; 4]>,
    pub effects: Vec<Effect>,
    pub opacity: f32,
}
```

### Layout Properties
```rust
pub struct LayoutProperties {
    pub width: SizeConstraint,
    pub height: SizeConstraint,
    pub padding: Padding,
    pub layout_mode: Option<LayoutMode>,
}
```

## Hot Reload Support

To support hot reload, implement a watch mechanism:

```rust
impl DesignAdapter for MyToolAdapter {
    fn supports_hot_reload(&self) -> bool {
        true
    }
    
    fn watch(&self, url: &str) -> Result<HotReloadStream, AdapterError> {
        // Watch for file changes
        // Or connect to design tool's API
        // Return a stream of updates
        todo!()
    }
}
```

## Best Practices

1. **Pixel-Perfect Mapping**: Ensure your adapter produces ≤ 1px error
2. **Performance**: Parsing should complete in < 50ms
3. **Completeness**: Support as many tool features as possible
4. **Testing**: Include test files from the design tool
5. **Documentation**: Document what features are supported

## Binary Format (.origin)

After conversion, the IR is compiled into a compact binary format:

```
.origin file structure:
┌─────────────────────┐
│ Header (16 bytes)   │ Magic "ORGN" + version
├─────────────────────┤
│ Font Table          │ Embedded font subsets
├─────────────────────┤
│ Texture Atlas       │ MSDF glyphs + images
├─────────────────────┤
│ Instance Buffer     │ Flat list of primitives
├─────────────────────┤
│ Constraint Table    │ Layout equations
├─────────────────────┤
│ Component Table     │ Reusable templates
├─────────────────────┤
│ Variant Table       │ State variants
├─────────────────────┤
│ Interaction Table   │ User interactions
└─────────────────────┘
```

Target: 1/8th the size of source JSON.

## Contributing

To add support for a new design tool:

1. Create a new crate under `crates/design-adapters/`
2. Implement the `DesignAdapter` trait
3. Add comprehensive tests
4. Document supported features
5. Submit a PR

Questions? Open a GitHub Discussion!

---

**Origin — Where design returns to its origin.** ⚡
